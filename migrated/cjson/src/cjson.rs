//! cJSON v1.7.19 parse + print, translated to Rust.

#![allow(dead_code)]

/// Mirrors CJSON_NESTING_LIMIT in cJSON.h.
pub const CJSON_NESTING_LIMIT: usize = 1000;

/// One cJSON node. Variants mirror the cJSON type flags.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    False,
    True,
    Null,
    Number(f64),
    String(Vec<u8>),
    Raw(Vec<u8>),
    Array(Vec<Value>),
    Object(Vec<(Vec<u8>, Value)>),
}

/// Parse failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub offset: Option<usize>,
}

struct ParseBuffer<'a> {
    content: &'a [u8],
    length: usize,
    offset: usize,
    depth: usize,
}

impl<'a> ParseBuffer<'a> {
    fn new(content: &'a [u8]) -> Self {
        Self {
            content,
            length: content.len(),
            offset: 0,
            depth: 0,
        }
    }
}

fn buffer_skip_whitespace(buffer: &mut ParseBuffer) {
    if buffer.offset >= buffer.length {
        return;
    }
    while buffer.offset < buffer.length {
        if let Some(&b) = buffer.content.get(buffer.offset) {
            if b <= 32 {
                buffer.offset += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if buffer.offset == buffer.length {
        buffer.offset -= 1;
    }
}

fn skip_utf8_bom(buffer: &mut ParseBuffer) {
    if buffer.offset != 0 {
        return;
    }
    // can_access_at_index(buffer, 4) means offset + 4 < length
    if buffer.offset + 4 < buffer.length {
        if let Some(slice) = buffer.content.get(buffer.offset..) {
            if slice.starts_with(b"\xEF\xBB\xBF") {
                buffer.offset += 3;
            }
        }
    }
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h = 0u32;
    for i in 0..4 {
        let b = match input.get(i) {
            Some(&v) => v,
            None => return 0,
        };
        let digit = if (b'0'..=b'9').contains(&b) {
            u32::from(b - b'0')
        } else if (b'A'..=b'F').contains(&b) {
            u32::from(10 + b - b'A')
        } else if (b'a'..=b'f').contains(&b) {
            u32::from(10 + b - b'a')
        } else {
            return 0;
        };
        h = (h << 4) | digit;
    }
    h
}

fn utf16_literal_to_utf8(input: &[u8], output: &mut Vec<u8>) -> usize {
    if input.len() < 6 {
        return 0;
    }
    let first_hex = match input.get(2..6) {
        Some(s) => s,
        None => return 0,
    };
    let first_code = parse_hex4(first_hex);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let codepoint;
    let sequence_length;

    if (0xD800..=0xDBFF).contains(&first_code) {
        sequence_length = 12;
        if input.len() < 12 {
            return 0;
        }
        if input.get(6).copied() != Some(b'\\') || input.get(7).copied() != Some(b'u') {
            return 0;
        }
        let second_hex = match input.get(8..12) {
            Some(s) => s,
            None => return 0,
        };
        let second_code = parse_hex4(second_hex);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        codepoint = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
    } else {
        sequence_length = 6;
        codepoint = first_code;
    }

    if codepoint < 0x80 {
        output.push(codepoint as u8);
    } else if codepoint < 0x800 {
        output.push((0xC0 | (codepoint >> 6)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
    } else if codepoint < 0x10000 {
        output.push((0xE0 | (codepoint >> 12)) as u8);
        output.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
    } else if codepoint <= 0x10FFFF {
        output.push((0xF0 | (codepoint >> 18)) as u8);
        output.push((0x80 | ((codepoint >> 12) & 0x3F)) as u8);
        output.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
    } else {
        return 0;
    }

    sequence_length
}

fn parse_string(buffer: &mut ParseBuffer) -> Result<Vec<u8>, ()> {
    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(b'\"') {
        buffer.offset += 1;
        return Err(());
    }

    let start_offset = buffer.offset;
    let mut input_end = start_offset + 1;
    let mut skipped_bytes = 0;

    while input_end < buffer.length && buffer.content.get(input_end).copied() != Some(b'\"') {
        if buffer.content.get(input_end).copied() == Some(b'\\') {
            if input_end + 1 >= buffer.length {
                buffer.offset = start_offset + 1;
                return Err(());
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }

    if input_end >= buffer.length || buffer.content.get(input_end).copied() != Some(b'\"') {
        buffer.offset = start_offset + 1;
        return Err(());
    }

    let mut input_pointer = start_offset + 1;
    let capacity = (input_end - start_offset).saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(capacity);

    while input_pointer < input_end {
        let b = match buffer.content.get(input_pointer) {
            Some(&v) => v,
            None => {
                buffer.offset = input_pointer;
                return Err(());
            }
        };
        if b != b'\\' {
            output.push(b);
            input_pointer += 1;
        } else {
            if input_end - input_pointer < 1 {
                buffer.offset = input_pointer;
                return Err(());
            }
            let escape = match buffer.content.get(input_pointer + 1) {
                Some(&v) => v,
                None => {
                    buffer.offset = input_pointer;
                    return Err(());
                }
            };
            match escape {
                b'b' => {
                    output.push(0x08);
                    input_pointer += 2;
                }
                b'f' => {
                    output.push(0x0C);
                    input_pointer += 2;
                }
                b'n' => {
                    output.push(b'\n');
                    input_pointer += 2;
                }
                b'r' => {
                    output.push(b'\r');
                    input_pointer += 2;
                }
                b't' => {
                    output.push(b'\t');
                    input_pointer += 2;
                }
                b'\"' | b'\\' | b'/' => {
                    output.push(escape);
                    input_pointer += 2;
                }
                b'u' => {
                    let slice = match buffer.content.get(input_pointer..input_end) {
                        Some(s) => s,
                        None => {
                            buffer.offset = input_pointer;
                            return Err(());
                        }
                    };
                    let seq_len = utf16_literal_to_utf8(slice, &mut output);
                    if seq_len == 0 {
                        buffer.offset = input_pointer;
                        return Err(());
                    }
                    input_pointer += seq_len;
                }
                _ => {
                    buffer.offset = input_pointer;
                    return Err(());
                }
            }
        }
    }

    buffer.offset = input_end + 1;
    Ok(output)
}

fn strtod_scan(s: &[u8]) -> usize {
    let mut idx = 0;
    if let Some(&b) = s.get(idx) {
        if b == b'+' || b == b'-' {
            idx += 1;
        }
    }

    let start_digits = idx;
    while let Some(&b) = s.get(idx) {
        if b.is_ascii_digit() {
            idx += 1;
        } else {
            break;
        }
    }
    let has_digits_before = idx > start_digits;

    let mut has_digits_after = false;
    if s.get(idx).copied() == Some(b'.') {
        idx += 1;
        let start_after = idx;
        while let Some(&b) = s.get(idx) {
            if b.is_ascii_digit() {
                idx += 1;
            } else {
                break;
            }
        }
        has_digits_after = idx > start_after;
    }

    if !has_digits_before && !has_digits_after {
        return 0;
    }

    let mut valid_end = idx;
    if let Some(&b) = s.get(idx) {
        if b == b'e' || b == b'E' {
            let mut exp_idx = idx + 1;
            if let Some(&sign) = s.get(exp_idx) {
                if sign == b'+' || sign == b'-' {
                    exp_idx += 1;
                }
            }
            let start_exp_digits = exp_idx;
            while let Some(&d) = s.get(exp_idx) {
                if d.is_ascii_digit() {
                    exp_idx += 1;
                } else {
                    break;
                }
            }
            if exp_idx > start_exp_digits {
                valid_end = exp_idx;
            }
        }
    }

    valid_end
}

fn parse_number(buffer: &mut ParseBuffer) -> Result<f64, ()> {
    let mut len = 0;
    while buffer.offset + len < buffer.length {
        if let Some(&b) = buffer.content.get(buffer.offset + len) {
            match b {
                b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => len += 1,
                _ => break,
            }
        } else {
            break;
        }
    }

    let slice = match buffer.content.get(buffer.offset..buffer.offset + len) {
        Some(s) => s,
        None => return Err(()),
    };
    let consumed = strtod_scan(slice);
    if consumed == 0 {
        return Err(());
    }

    let num_slice = match slice.get(..consumed) {
        Some(s) => s,
        None => return Err(()),
    };
    let mut norm = Vec::with_capacity(num_slice.len() + 2);
    for (i, &b) in num_slice.iter().enumerate() {
        norm.push(b);
        if b == b'.' {
            let next_is_exp_or_end = match num_slice.get(i + 1) {
                None => true,
                Some(&next_b) => next_b == b'e' || next_b == b'E',
            };
            if next_is_exp_or_end {
                norm.push(b'0');
            }
        }
    }

    let norm_str = match std::str::from_utf8(&norm) {
        Ok(s) => s,
        Err(_) => return Err(()),
    };

    let val = match norm_str.parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            if norm_str.starts_with('-') {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        }
    };

    buffer.offset += consumed;
    Ok(val)
}

fn parse_array(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return Err(());
    }
    buffer.depth += 1;

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(b'[') {
        buffer.depth -= 1;
        return Err(());
    }

    buffer.offset += 1;
    buffer_skip_whitespace(buffer);
    if buffer.offset < buffer.length && buffer.content.get(buffer.offset).copied() == Some(b']') {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Ok(Value::Array(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        buffer.offset -= 1;
        buffer.depth -= 1;
        return Err(());
    }

    buffer.offset -= 1;
    let mut items = Vec::new();

    loop {
        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        let val = match parse_value(buffer) {
            Ok(v) => v,
            Err(()) => {
                buffer.depth -= 1;
                return Err(());
            }
        };
        items.push(val);
        buffer_skip_whitespace(buffer);
        if buffer.offset < buffer.length && buffer.content.get(buffer.offset).copied() == Some(b',')
        {
            continue;
        }
        break;
    }

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(b']') {
        buffer.depth -= 1;
        return Err(());
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Ok(Value::Array(items))
}

fn parse_object(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return Err(());
    }
    buffer.depth += 1;

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(b'{') {
        buffer.depth -= 1;
        return Err(());
    }

    buffer.offset += 1;
    buffer_skip_whitespace(buffer);
    if buffer.offset < buffer.length && buffer.content.get(buffer.offset).copied() == Some(b'}') {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Ok(Value::Object(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        buffer.offset -= 1;
        buffer.depth -= 1;
        return Err(());
    }

    buffer.offset -= 1;
    let mut entries = Vec::new();

    loop {
        if buffer.offset + 1 >= buffer.length {
            buffer.depth -= 1;
            return Err(());
        }

        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        let key = match parse_string(buffer) {
            Ok(k) => k,
            Err(()) => {
                buffer.depth -= 1;
                return Err(());
            }
        };
        buffer_skip_whitespace(buffer);

        if buffer.offset >= buffer.length
            || buffer.content.get(buffer.offset).copied() != Some(b':')
        {
            buffer.depth -= 1;
            return Err(());
        }

        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        let val = match parse_value(buffer) {
            Ok(v) => v,
            Err(()) => {
                buffer.depth -= 1;
                return Err(());
            }
        };
        buffer_skip_whitespace(buffer);

        entries.push((key, val));

        if buffer.offset < buffer.length && buffer.content.get(buffer.offset).copied() == Some(b',')
        {
            continue;
        }
        break;
    }

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(b'}') {
        buffer.depth -= 1;
        return Err(());
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Ok(Value::Object(entries))
}

fn parse_value(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.offset >= buffer.length {
        return Err(());
    }

    // null
    if buffer.offset + 4 <= buffer.length
        && buffer.content.get(buffer.offset..buffer.offset + 4) == Some(b"null".as_slice())
    {
        buffer.offset += 4;
        return Ok(Value::Null);
    }
    // false
    if buffer.offset + 5 <= buffer.length
        && buffer.content.get(buffer.offset..buffer.offset + 5) == Some(b"false".as_slice())
    {
        buffer.offset += 5;
        return Ok(Value::False);
    }
    // true
    if buffer.offset + 4 <= buffer.length
        && buffer.content.get(buffer.offset..buffer.offset + 4) == Some(b"true".as_slice())
    {
        buffer.offset += 4;
        return Ok(Value::True);
    }

    let b = match buffer.content.get(buffer.offset) {
        Some(&v) => v,
        None => return Err(()),
    };

    // string
    if b == b'\"' {
        let s = parse_string(buffer)?;
        return Ok(Value::String(s));
    }
    // number
    if b == b'-' || (b'0'..=b'9').contains(&b) {
        let n = parse_number(buffer)?;
        return Ok(Value::Number(n));
    }
    // array
    if b == b'[' {
        return parse_array(buffer);
    }
    // object
    if b == b'{' {
        return parse_object(buffer);
    }

    Err(())
}

pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut buf = Vec::with_capacity(input.len() + 1);
    buf.extend_from_slice(input);
    buf.push(0);

    let mut buffer = ParseBuffer::new(&buf);

    skip_utf8_bom(&mut buffer);
    buffer_skip_whitespace(&mut buffer);

    let val = match parse_value(&mut buffer) {
        Ok(v) => v,
        Err(()) => {
            let offset = if buffer.offset < buffer.length {
                buffer.offset
            } else if buffer.length > 0 {
                buffer.length - 1
            } else {
                0
            };
            return Err(ParseError {
                offset: Some(offset),
            });
        }
    };

    buffer_skip_whitespace(&mut buffer);
    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset).copied() != Some(0) {
        let offset = if buffer.offset < buffer.length {
            buffer.offset
        } else if buffer.length > 0 {
            buffer.length - 1
        } else {
            0
        };
        return Err(ParseError {
            offset: Some(offset),
        });
    }

    Ok(val)
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}

fn format_g_pos(val: f64, p: usize) -> String {
    let s = format!("{:.*e}", p - 1, val);
    let e_pos = match s.find('e') {
        Some(pos) => pos,
        None => return s,
    };
    let dot_pos = match s.find('.') {
        Some(pos) => pos,
        None => return s,
    };
    let exp_slice = match s.get(e_pos + 1..) {
        Some(sl) => sl,
        None => return s,
    };
    let exp_val: i32 = match exp_slice.parse() {
        Ok(v) => v,
        Err(_) => return s,
    };

    let first_digit = match s.get(..dot_pos) {
        Some(sl) => sl,
        None => return s,
    };
    let frac_digits = match s.get(dot_pos + 1..e_pos) {
        Some(sl) => sl,
        None => return s,
    };
    let mut all_digits = String::with_capacity(p);
    all_digits.push_str(first_digit);
    all_digits.push_str(frac_digits);

    let p_i32 = p as i32;
    if exp_val >= -4 && exp_val < p_i32 {
        let mut res = String::new();
        if exp_val >= 0 {
            let int_digits_count = (exp_val + 1) as usize;
            if let (Some(int_part), Some(frac)) = (
                all_digits.get(..int_digits_count),
                all_digits.get(int_digits_count..),
            ) {
                res.push_str(int_part);
                let trimmed_frac = frac.trim_end_matches('0');
                if !trimmed_frac.is_empty() {
                    res.push('.');
                    res.push_str(trimmed_frac);
                }
            } else {
                res.push_str(&all_digits);
                for _ in 0..int_digits_count.saturating_sub(all_digits.len()) {
                    res.push('0');
                }
            }
        } else {
            res.push_str("0.");
            let leading_zeros = (-1 - exp_val) as usize;
            for _ in 0..leading_zeros {
                res.push('0');
            }
            let trimmed = all_digits.trim_end_matches('0');
            res.push_str(trimmed);
        }
        res
    } else {
        let first = all_digits.get(..1).unwrap_or("");
        let rest = all_digits.get(1..).unwrap_or("");
        let trimmed_rest = rest.trim_end_matches('0');
        let mut res = String::new();
        res.push_str(first);
        if !trimmed_rest.is_empty() {
            res.push('.');
            res.push_str(trimmed_rest);
        }
        let exp_sign = if exp_val >= 0 { '+' } else { '-' };
        let exp_abs = exp_val.abs();
        res.push_str(&format!("e{}{:02}", exp_sign, exp_abs));
        res
    }
}

fn format_g(d: f64, p: usize) -> String {
    if d < 0.0 {
        let mut res = String::from("-");
        res.push_str(&format_g_pos(d.abs(), p));
        res
    } else {
        format_g_pos(d, p)
    }
}

fn print_number(d: f64, out: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        out.extend_from_slice(b"null");
        return;
    }

    let valueint: i32 = if d >= f64::from(i32::MAX) {
        i32::MAX
    } else if d <= f64::from(i32::MIN) {
        i32::MIN
    } else {
        d as i32
    };

    if d == f64::from(valueint) {
        out.extend_from_slice(valueint.to_string().as_bytes());
        return;
    }

    let str15 = format_g(d, 15);
    let mut use_17 = false;
    if let Ok(test) = str15.parse::<f64>() {
        if !compare_double(test, d) {
            use_17 = true;
        }
    } else {
        use_17 = true;
    }

    if use_17 {
        let str17 = format_g(d, 17);
        out.extend_from_slice(str17.as_bytes());
    } else {
        out.extend_from_slice(str15.as_bytes());
    }
}

fn print_string(bytes: &[u8], out: &mut Vec<u8>) {
    out.push(b'\"');
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let hex = b"0123456789abcdef";
    if let Some(slice) = bytes.get(..len) {
        for &b in slice {
            match b {
                b'\"' => out.extend_from_slice(b"\\\""),
                b'\\' => out.extend_from_slice(b"\\\\"),
                0x08 => out.extend_from_slice(b"\\b"),
                0x0C => out.extend_from_slice(b"\\f"),
                b'\n' => out.extend_from_slice(b"\\n"),
                b'\r' => out.extend_from_slice(b"\\r"),
                b'\t' => out.extend_from_slice(b"\\t"),
                0..=31 => {
                    out.extend_from_slice(b"\\u00");
                    if let Some(&h1) = hex.get(usize::from(b >> 4)) {
                        out.push(h1);
                    }
                    if let Some(&h2) = hex.get(usize::from(b & 0x0F)) {
                        out.push(h2);
                    }
                }
                _ => out.push(b),
            }
        }
    }
    out.push(b'\"');
}

fn print_value(val: &Value, out: &mut Vec<u8>) {
    match val {
        Value::Null => out.extend_from_slice(b"null"),
        Value::False => out.extend_from_slice(b"false"),
        Value::True => out.extend_from_slice(b"true"),
        Value::Number(num) => print_number(*num, out),
        Value::String(bytes) => print_string(bytes, out),
        Value::Raw(bytes) => {
            let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            if let Some(slice) = bytes.get(..len) {
                out.extend_from_slice(slice);
            }
        }
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_value(item, out);
            }
            out.push(b']');
        }
        Value::Object(entries) => {
            out.push(b'{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_string(key, out);
                out.push(b':');
                print_value(item, out);
            }
            out.push(b'}');
        }
    }
}

pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    print_value(value, &mut out);
    out
}
