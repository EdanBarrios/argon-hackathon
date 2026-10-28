//! cJSON v1.7.19 parse and print translated to safe Rust.
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

fn skip_utf8_bom(buffer: &mut ParseBuffer) {
    if buffer.offset == 0 && (buffer.offset + 4) < buffer.length {
        if buffer.content.len() >= 3 && &buffer.content[0..3] == b"\xEF\xBB\xBF" {
            buffer.offset += 3;
        }
    }
}

fn buffer_skip_whitespace(buffer: &mut ParseBuffer) {
    while buffer.offset < buffer.length && buffer.content[buffer.offset] <= 32 {
        buffer.offset += 1;
    }
    if buffer.offset == buffer.length && buffer.length > 0 {
        buffer.offset -= 1;
    }
}

fn parse_hex4(slice: &[u8]) -> u32 {
    let mut h = 0u32;
    for &b in slice.iter().take(4) {
        let nibble = if (b'0'..=b'9').contains(&b) {
            (b - b'0') as u32
        } else if (b'A'..=b'F').contains(&b) {
            (b - b'A' + 10) as u32
        } else if (b'a'..=b'f').contains(&b) {
            (b - b'a' + 10) as u32
        } else {
            return 0;
        };
        h = (h << 4) | nibble;
    }
    h
}

fn utf16_literal_to_utf8(
    content: &[u8],
    first_sequence: usize,
    input_end: usize,
    output: &mut Vec<u8>,
) -> Option<usize> {
    if input_end.saturating_sub(first_sequence) < 6 {
        return None;
    }
    let first_code = parse_hex4(content.get(first_sequence + 2..first_sequence + 6)?);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return None;
    }
    let (codepoint, sequence_length) = if (0xD800..=0xDBFF).contains(&first_code) {
        let second_sequence = first_sequence + 6;
        if input_end.saturating_sub(second_sequence) < 6 {
            return None;
        }
        if *content.get(second_sequence)? != b'\\' || *content.get(second_sequence + 1)? != b'u' {
            return None;
        }
        let second_code = parse_hex4(content.get(second_sequence + 2..second_sequence + 6)?);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return None;
        }
        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (cp, 12)
    } else {
        (first_code, 6)
    };

    let utf8_length;
    let first_byte_mark: u8;
    if codepoint < 0x80 {
        utf8_length = 1;
        first_byte_mark = 0;
    } else if codepoint < 0x800 {
        utf8_length = 2;
        first_byte_mark = 0xC0;
    } else if codepoint < 0x10000 {
        utf8_length = 3;
        first_byte_mark = 0xE0;
    } else if codepoint <= 0x10FFFF {
        utf8_length = 4;
        first_byte_mark = 0xF0;
    } else {
        return None;
    }

    let mut cp = codepoint;
    let mut temp = [0u8; 4];
    for pos in (1..utf8_length).rev() {
        temp[pos] = ((cp | 0x80) & 0xBF) as u8;
        cp >>= 6;
    }
    if utf8_length > 1 {
        temp[0] = ((cp | (first_byte_mark as u32)) & 0xFF) as u8;
    } else {
        temp[0] = (cp & 0x7F) as u8;
    }
    output.extend_from_slice(&temp[..utf8_length]);
    Some(sequence_length)
}

fn parse_string(buffer: &mut ParseBuffer) -> Option<Value> {
    let start_offset = buffer.offset;
    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b'"' {
        buffer.offset = buffer.offset.saturating_add(1);
        return None;
    }

    let mut input_pointer = start_offset + 1;
    let mut input_end = start_offset + 1;
    let mut skipped_bytes = 0;

    while input_end < buffer.length && *buffer.content.get(input_end)? != b'"' {
        if *buffer.content.get(input_end)? == b'\\' {
            if input_end + 1 >= buffer.length {
                buffer.offset = input_pointer;
                return None;
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }

    if input_end >= buffer.length || *buffer.content.get(input_end)? != b'"' {
        buffer.offset = input_pointer;
        return None;
    }

    let alloc_len = (input_end - start_offset).saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(alloc_len);

    while input_pointer < input_end {
        if *buffer.content.get(input_pointer)? != b'\\' {
            output.push(*buffer.content.get(input_pointer)?);
            input_pointer += 1;
        } else {
            if input_end.saturating_sub(input_pointer) < 2 {
                buffer.offset = input_pointer;
                return None;
            }
            let esc = *buffer.content.get(input_pointer + 1)?;
            let mut sequence_length = 2;
            match esc {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'"' | b'\\' | b'/' => output.push(esc),
                b'u' => {
                    match utf16_literal_to_utf8(buffer.content, input_pointer, input_end, &mut output) {
                        Some(seq_len) => {
                            sequence_length = seq_len;
                        }
                        None => {
                            buffer.offset = input_pointer;
                            return None;
                        }
                    }
                }
                _ => {
                    buffer.offset = input_pointer;
                    return None;
                }
            }
            input_pointer += sequence_length;
        }
    }

    buffer.offset = input_end + 1;
    Some(Value::String(output))
}

fn strtod_like(b: &[u8]) -> Option<(usize, f64)> {
    let n = b.len();
    let mut i = 0;
    if i < n && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }

    let mut has_digits = false;
    while i < n && (b'0'..=b'9').contains(&b[i]) {
        has_digits = true;
        i += 1;
    }

    if i < n && b[i] == b'.' {
        i += 1;
        while i < n && (b'0'..=b'9').contains(&b[i]) {
            has_digits = true;
            i += 1;
        }
    }

    if !has_digits {
        return None;
    }

    let exp_pos = i;
    if i < n && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < n && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        let mut has_exp_digits = false;
        while j < n && (b'0'..=b'9').contains(&b[j]) {
            has_exp_digits = true;
            j += 1;
        }
        if has_exp_digits {
            i = j;
        } else {
            i = exp_pos;
        }
    }

    let consumed = i;
    let s = std::str::from_utf8(b.get(..consumed)?).ok()?;
    let val = s.parse::<f64>().ok()?;
    Some((consumed, val))
}

fn parse_number(buffer: &mut ParseBuffer) -> Option<Value> {
    if buffer.offset >= buffer.length {
        return None;
    }
    let mut num_len = 0;
    while buffer.offset + num_len < buffer.length {
        let b = *buffer.content.get(buffer.offset + num_len)?;
        match b {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                num_len += 1;
            }
            _ => break,
        }
    }

    let slice = buffer.content.get(buffer.offset..buffer.offset + num_len)?;
    let (consumed, val) = strtod_like(slice)?;
    if consumed == 0 {
        return None;
    }

    buffer.offset += consumed;
    Some(Value::Number(val))
}

fn parse_array(buffer: &mut ParseBuffer) -> Option<Value> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    buffer.depth += 1;

    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b'[' {
        buffer.depth -= 1;
        return None;
    }

    buffer.offset += 1;
    buffer_skip_whitespace(buffer);
    if buffer.offset < buffer.length && *buffer.content.get(buffer.offset)? == b']' {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Some(Value::Array(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        if buffer.offset > 0 {
            buffer.offset -= 1;
        }
        buffer.depth -= 1;
        return None;
    }

    buffer.offset -= 1;
    let mut items = Vec::new();

    loop {
        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        match parse_value(buffer) {
            Some(val) => items.push(val),
            None => {
                buffer.depth -= 1;
                return None;
            }
        }
        buffer_skip_whitespace(buffer);
        if !(buffer.offset < buffer.length && *buffer.content.get(buffer.offset)? == b',') {
            break;
        }
    }

    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b']' {
        buffer.depth -= 1;
        return None;
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Some(Value::Array(items))
}

fn parse_object(buffer: &mut ParseBuffer) -> Option<Value> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    buffer.depth += 1;

    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b'{' {
        buffer.depth -= 1;
        return None;
    }

    buffer.offset += 1;
    buffer_skip_whitespace(buffer);
    if buffer.offset < buffer.length && *buffer.content.get(buffer.offset)? == b'}' {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Some(Value::Object(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        if buffer.offset > 0 {
            buffer.offset -= 1;
        }
        buffer.depth -= 1;
        return None;
    }

    buffer.offset -= 1;
    let mut entries = Vec::new();

    loop {
        if buffer.offset + 1 >= buffer.length {
            buffer.depth -= 1;
            return None;
        }

        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        let key = match parse_string(buffer) {
            Some(Value::String(k)) => k,
            _ => {
                buffer.depth -= 1;
                return None;
            }
        };
        buffer_skip_whitespace(buffer);

        if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b':' {
            buffer.depth -= 1;
            return None;
        }

        buffer.offset += 1;
        buffer_skip_whitespace(buffer);
        let val = match parse_value(buffer) {
            Some(v) => v,
            None => {
                buffer.depth -= 1;
                return None;
            }
        };
        buffer_skip_whitespace(buffer);

        entries.push((key, val));

        if !(buffer.offset < buffer.length && *buffer.content.get(buffer.offset)? == b',') {
            break;
        }
    }

    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset)? != b'}' {
        buffer.depth -= 1;
        return None;
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Some(Value::Object(entries))
}

fn parse_value(buffer: &mut ParseBuffer) -> Option<Value> {
    if buffer.offset >= buffer.length {
        return None;
    }

    if buffer.offset + 4 <= buffer.length && &buffer.content[buffer.offset..buffer.offset + 4] == b"null" {
        buffer.offset += 4;
        return Some(Value::Null);
    }
    if buffer.offset + 5 <= buffer.length && &buffer.content[buffer.offset..buffer.offset + 5] == b"false" {
        buffer.offset += 5;
        return Some(Value::False);
    }
    if buffer.offset + 4 <= buffer.length && &buffer.content[buffer.offset..buffer.offset + 4] == b"true" {
        buffer.offset += 4;
        return Some(Value::True);
    }

    let b = *buffer.content.get(buffer.offset)?;
    if b == b'"' {
        return parse_string(buffer);
    }
    if b == b'-' || (b'0'..=b'9').contains(&b) {
        return parse_number(buffer);
    }
    if b == b'[' {
        return parse_array(buffer);
    }
    if b == b'{' {
        return parse_object(buffer);
    }

    None
}

/// Parse JSON text.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut content = Vec::with_capacity(input.len().saturating_add(1));
    content.extend_from_slice(input);
    content.push(0);

    let mut buffer = ParseBuffer {
        length: content.len(),
        content: &content,
        offset: 0,
        depth: 0,
    };

    skip_utf8_bom(&mut buffer);
    buffer_skip_whitespace(&mut buffer);

    let val = match parse_value(&mut buffer) {
        Some(v) => v,
        None => {
            let error_pos = if buffer.offset < buffer.length {
                buffer.offset
            } else if buffer.length > 0 {
                buffer.length - 1
            } else {
                0
            };
            return Err(ParseError { offset: Some(error_pos) });
        }
    };

    buffer_skip_whitespace(&mut buffer);
    if buffer.offset >= buffer.length || *buffer.content.get(buffer.offset).unwrap_or(&1) != 0 {
        let error_pos = if buffer.offset < buffer.length {
            buffer.offset
        } else if buffer.length > 0 {
            buffer.length - 1
        } else {
            0
        };
        return Err(ParseError { offset: Some(error_pos) });
    }

    Ok(val)
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = a.abs().max(b.abs());
    (a - b).abs() <= max_val * f64::EPSILON
}

fn format_g(prec: usize, d: f64) -> String {
    let is_neg = d.is_sign_negative();
    let d_abs = d.abs();
    let s_e = format!("{:.*e}", prec.saturating_sub(1), d_abs);
    let e_pos = match s_e.bytes().position(|b| b == b'e') {
        Some(p) => p,
        None => return String::new(),
    };
    let mantissa_with_dot = match s_e.get(..e_pos) {
        Some(s) => s,
        None => return String::new(),
    };
    let exp_str = match s_e.get(e_pos + 1..) {
        Some(s) => s,
        None => return String::new(),
    };
    let exp: i32 = match exp_str.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return String::new(),
    };

    let mut mantissa_str = String::with_capacity(prec);
    for c in mantissa_with_dot.chars() {
        if c != '.' {
            mantissa_str.push(c);
        }
    }
    if mantissa_str.is_empty() {
        return String::new();
    }

    let prec_i32 = prec as i32;
    let prefix = if is_neg { "-" } else { "" };

    if exp < -4 || exp >= prec_i32 {
        let first_digit = match mantissa_str.get(..1) {
            Some(s) => s,
            None => return String::new(),
        };
        let frac = mantissa_str.get(1..).unwrap_or("").trim_end_matches('0');
        let body = if frac.is_empty() {
            first_digit.to_string()
        } else {
            let mut b = String::with_capacity(1 + 1 + frac.len());
            b.push_str(first_digit);
            b.push('.');
            b.push_str(frac);
            b
        };
        let exp_sign = if exp >= 0 { '+' } else { '-' };
        let exp_abs = exp.unsigned_abs();
        format!("{}{}{}e{}{:02}", prefix, body, "", exp_sign, exp_abs)
    } else if exp >= 0 {
        let int_part_len = (exp as usize).saturating_add(1);
        if int_part_len <= mantissa_str.len() {
            let int_part = match mantissa_str.get(..int_part_len) {
                Some(s) => s,
                None => return String::new(),
            };
            let frac_part = mantissa_str.get(int_part_len..).unwrap_or("").trim_end_matches('0');
            if frac_part.is_empty() {
                format!("{}{}", prefix, int_part)
            } else {
                format!("{}{}.{}", prefix, int_part, frac_part)
            }
        } else {
            let zeros = "0".repeat(int_part_len.saturating_sub(mantissa_str.len()));
            format!("{}{}{}", prefix, mantissa_str, zeros)
        }
    } else {
        let zeros_len = (-exp - 1) as usize;
        let zeros = "0".repeat(zeros_len);
        let mut frac_part = String::with_capacity(zeros_len + mantissa_str.len());
        frac_part.push_str(&zeros);
        frac_part.push_str(&mantissa_str);
        let frac_trimmed = frac_part.trim_end_matches('0');
        format!("{}0.{}", prefix, frac_trimmed)
    }
}

fn print_number(d: f64, out: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        out.extend_from_slice(b"null");
        return;
    }
    let valueint: i32 = if d >= (i32::MAX as f64) {
        i32::MAX
    } else if d <= (i32::MIN as f64) {
        i32::MIN
    } else {
        d as i32
    };
    if d == (valueint as f64) {
        out.extend_from_slice(format!("{}", valueint).as_bytes());
        return;
    }
    let s15 = format_g(15, d);
    if let Ok(test) = s15.parse::<f64>() {
        if compare_double(test, d) {
            out.extend_from_slice(s15.as_bytes());
            return;
        }
    }
    let s17 = format_g(17, d);
    out.extend_from_slice(s17.as_bytes());
}

fn print_string_ptr(input: &[u8], out: &mut Vec<u8>) {
    out.push(b'"');
    let mut i = 0;
    while i < input.len() {
        let b = input[i];
        if b == 0 {
            break;
        }
        if b > 31 && b != b'"' && b != b'\\' {
            out.push(b);
        } else {
            out.push(b'\\');
            match b {
                b'\\' => out.push(b'\\'),
                b'"' => out.push(b'"'),
                0x08 => out.push(b'b'),
                0x0C => out.push(b'f'),
                b'\n' => out.push(b'n'),
                b'\r' => out.push(b'r'),
                b'\t' => out.push(b't'),
                _ => {
                    let hex_chars = b"0123456789abcdef";
                    out.push(b'u');
                    out.push(b'0');
                    out.push(b'0');
                    out.push(hex_chars[((b >> 4) & 0x0F) as usize]);
                    out.push(hex_chars[(b & 0x0F) as usize]);
                }
            }
        }
        i += 1;
    }
    out.push(b'"');
}

fn print_value(val: &Value, out: &mut Vec<u8>) {
    match val {
        Value::False => out.extend_from_slice(b"false"),
        Value::True => out.extend_from_slice(b"true"),
        Value::Null => out.extend_from_slice(b"null"),
        Value::Number(d) => print_number(*d, out),
        Value::Raw(bytes) => {
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            out.extend_from_slice(&bytes[..end]);
        }
        Value::String(bytes) => print_string_ptr(bytes, out),
        Value::Array(items) => {
            out.push(b'[');
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(b',');
                }
                print_value(item, out);
            }
            out.push(b']');
        }
        Value::Object(entries) => {
            out.push(b'{');
            for (idx, (key, item)) in entries.iter().enumerate() {
                if idx > 0 {
                    out.push(b',');
                }
                print_string_ptr(key, out);
                out.push(b':');
                print_value(item, out);
            }
            out.push(b'}');
        }
    }
}

/// Render a Value to unformatted JSON bytes.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    print_value(value, &mut out);
    out
}
