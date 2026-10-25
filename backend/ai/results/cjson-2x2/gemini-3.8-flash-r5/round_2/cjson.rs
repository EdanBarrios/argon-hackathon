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
    /// Byte offset of cJSON's error pointer into the input, if any.
    pub offset: Option<usize>,
}

struct ParseBuffer<'a> {
    content: &'a [u8],
    length: usize,
    offset: usize,
    depth: usize,
}

fn skip_utf8_bom(buffer: &mut ParseBuffer) {
    if buffer.offset != 0 {
        return;
    }

    if buffer.offset.saturating_add(4) < buffer.length
        && buffer.content.get(buffer.offset..buffer.offset.saturating_add(3)) == Some(b"\xEF\xBB\xBF")
    {
        buffer.offset = buffer.offset.saturating_add(3);
    }
}

fn skip_whitespace(buffer: &mut ParseBuffer) {
    if buffer.offset >= buffer.length {
        return;
    }

    while buffer.offset < buffer.length {
        if let Some(&b) = buffer.content.get(buffer.offset) {
            if b <= 32 {
                buffer.offset = buffer.offset.saturating_add(1);
                continue;
            }
        }
        break;
    }

    if buffer.offset == buffer.length && buffer.offset > 0 {
        buffer.offset -= 1;
    }
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h = 0u32;
    for i in 0..4 {
        let b = match input.get(i) {
            Some(&val) => val,
            None => return 0,
        };
        let digit = match b {
            b'0'..=b'9' => (b - b'0') as u32,
            b'A'..=b'F' => 10 + (b - b'A') as u32,
            b'a'..=b'f' => 10 + (b - b'a') as u32,
            _ => return 0,
        };
        h += digit;
        if i < 3 {
            h <<= 4;
        }
    }
    h
}

fn utf16_literal_to_utf8(
    input: &[u8],
    input_offset: usize,
    input_end: usize,
    output: &mut Vec<u8>,
) -> Option<usize> {
    let remaining = input_end.saturating_sub(input_offset);
    if remaining < 6 {
        return None;
    }

    let first_slice = input.get(input_offset.saturating_add(2)..input_offset.saturating_add(6))?;
    let first_code = parse_hex4(first_slice);

    if (0xDC00..=0xDFFF).contains(&first_code) {
        return None;
    }

    let (codepoint, sequence_length) = if (0xD800..=0xDBFF).contains(&first_code) {
        let second_offset = input_offset.saturating_add(6);
        if input_end.saturating_sub(second_offset) < 6 {
            return None;
        }

        if input.get(second_offset) != Some(&b'\\') || input.get(second_offset.saturating_add(1)) != Some(&b'u') {
            return None;
        }

        let second_slice = input.get(second_offset.saturating_add(2)..second_offset.saturating_add(6))?;
        let second_code = parse_hex4(second_slice);
        if second_code < 0xDC00 || second_code > 0xDFFF {
            return None;
        }

        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (cp, 12)
    } else {
        (first_code, 6)
    };

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
        return None;
    }

    Some(sequence_length)
}

fn parse_string(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    let start_offset = buffer.offset;
    if buffer.content.get(start_offset) != Some(&b'\"') {
        buffer.offset = start_offset.saturating_add(1);
        return Err(());
    }

    let mut input_end = start_offset.saturating_add(1);
    while input_end < buffer.length && buffer.content.get(input_end) != Some(&b'\"') {
        if buffer.content.get(input_end) == Some(&b'\\') {
            if input_end.saturating_add(1) >= buffer.length {
                buffer.offset = start_offset.saturating_add(1);
                return Err(());
            }
            input_end = input_end.saturating_add(1);
        }
        input_end = input_end.saturating_add(1);
    }

    if input_end >= buffer.length || buffer.content.get(input_end) != Some(&b'\"') {
        buffer.offset = start_offset.saturating_add(1);
        return Err(());
    }

    let mut input_pointer = start_offset.saturating_add(1);
    let mut output = Vec::with_capacity(input_end.saturating_sub(start_offset));

    while input_pointer < input_end {
        let b = match buffer.content.get(input_pointer) {
            Some(&c) => c,
            None => {
                buffer.offset = input_pointer;
                return Err(());
            }
        };

        if b != b'\\' {
            output.push(b);
            input_pointer = input_pointer.saturating_add(1);
        } else {
            if input_end.saturating_sub(input_pointer) < 2 {
                buffer.offset = input_pointer;
                return Err(());
            }

            let esc = match buffer.content.get(input_pointer.saturating_add(1)) {
                Some(&c) => c,
                None => {
                    buffer.offset = input_pointer;
                    return Err(());
                }
            };

            match esc {
                b'b' => {
                    output.push(0x08);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b'f' => {
                    output.push(0x0C);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b'n' => {
                    output.push(0x0A);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b'r' => {
                    output.push(0x0D);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b't' => {
                    output.push(0x09);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b'\"' | b'\\' | b'/' => {
                    output.push(esc);
                    input_pointer = input_pointer.saturating_add(2);
                }
                b'u' => {
                    match utf16_literal_to_utf8(
                        buffer.content,
                        input_pointer,
                        input_end,
                        &mut output,
                    ) {
                        Some(seq_len) => {
                            input_pointer = input_pointer.saturating_add(seq_len);
                        }
                        None => {
                            buffer.offset = input_pointer;
                            return Err(());
                        }
                    }
                }
                _ => {
                    buffer.offset = input_pointer;
                    return Err(());
                }
            }
        }
    }

    buffer.offset = input_end.saturating_add(1);
    Ok(Value::String(output))
}

fn parse_strtod_prefix(slice: &[u8]) -> Option<(f64, usize)> {
    if slice.is_empty() {
        return None;
    }

    let mut i = 0;
    let is_negative = if slice.get(i) == Some(&b'-') {
        i += 1;
        true
    } else if slice.get(i) == Some(&b'+') {
        i += 1;
        false
    } else {
        false
    };

    let mut digits_before_dot = 0;
    while i < slice.len() {
        if let Some(b) = slice.get(i) {
            if b.is_ascii_digit() {
                digits_before_dot += 1;
                i += 1;
                continue;
            }
        }
        break;
    }

    let mut digits_after_dot = 0;
    if slice.get(i) == Some(&b'.') {
        i += 1;
        while i < slice.len() {
            if let Some(b) = slice.get(i) {
                if b.is_ascii_digit() {
                    digits_after_dot += 1;
                    i += 1;
                    continue;
                }
            }
            break;
        }
    }

    if digits_before_dot == 0 && digits_after_dot == 0 {
        return None;
    }

    let mut exp_end = i;
    if i < slice.len() && (slice.get(i) == Some(&b'e') || slice.get(i) == Some(&b'E')) {
        let mut j = i + 1;
        if j < slice.len() && (slice.get(j) == Some(&b'+') || slice.get(j) == Some(&b'-')) {
            j += 1;
        }
        let mut exp_digits = 0;
        while j < slice.len() {
            if let Some(b) = slice.get(j) {
                if b.is_ascii_digit() {
                    exp_digits += 1;
                    j += 1;
                    continue;
                }
            }
            break;
        }
        if exp_digits > 0 {
            exp_end = j;
        }
    }

    let consumed = exp_end;
    let num_bytes = slice.get(..consumed)?;
    let mut normalized = String::with_capacity(num_bytes.len().saturating_add(4));

    let mut s = match std::str::from_utf8(num_bytes) {
        Ok(v) => v,
        Err(_) => return None,
    };

    if s.starts_with('.') {
        normalized.push('0');
    } else if s.starts_with("+.") {
        normalized.push_str("+0");
        s = s.get(1..).unwrap_or("");
    } else if s.starts_with("-.") {
        normalized.push_str("-0");
        s = s.get(1..).unwrap_or("");
    }

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        normalized.push(c);
        if c == '.' {
            match chars.peek() {
                Some(&'e') | Some(&'E') | None => {
                    normalized.push('0');
                }
                _ => {}
            }
        }
    }

    let mut val: f64 = normalized.parse().unwrap_or(0.0);
    if is_negative && val == 0.0 && !val.is_sign_negative() {
        val = -0.0;
    }

    Some((val, consumed))
}

fn parse_number(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    let mut number_string_length = 0;
    while buffer.offset.saturating_add(number_string_length) < buffer.length {
        let idx = buffer.offset.saturating_add(number_string_length);
        match buffer.content.get(idx).copied() {
            Some(b'0'..=b'9')
            | Some(b'+')
            | Some(b'-')
            | Some(b'e')
            | Some(b'E')
            | Some(b'.') => {
                number_string_length = number_string_length.saturating_add(1);
            }
            _ => break,
        }
    }

    let slice_end = buffer.offset.saturating_add(number_string_length);
    let num_slice = match buffer.content.get(buffer.offset..slice_end) {
        Some(s) => s,
        None => return Err(()),
    };

    if let Some((number, consumed)) = parse_strtod_prefix(num_slice) {
        buffer.offset = buffer.offset.saturating_add(consumed);
        Ok(Value::Number(number))
    } else {
        Err(())
    }
}

fn parse_array(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return Err(());
    }
    buffer.depth = buffer.depth.saturating_add(1);

    if buffer.content.get(buffer.offset) != Some(&b'[') {
        return Err(());
    }

    buffer.offset = buffer.offset.saturating_add(1);
    skip_whitespace(buffer);

    if buffer.offset < buffer.length && buffer.content.get(buffer.offset) == Some(&b']') {
        buffer.depth = buffer.depth.saturating_sub(1);
        buffer.offset = buffer.offset.saturating_add(1);
        return Ok(Value::Array(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        if buffer.offset > 0 {
            buffer.offset -= 1;
        }
        return Err(());
    }

    buffer.offset = buffer.offset.saturating_sub(1);

    let mut elements = Vec::new();
    loop {
        buffer.offset = buffer.offset.saturating_add(1);
        skip_whitespace(buffer);
        let val = parse_value(buffer)?;
        elements.push(val);
        skip_whitespace(buffer);

        if buffer.offset < buffer.length && buffer.content.get(buffer.offset) == Some(&b',') {
            continue;
        } else {
            break;
        }
    }

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset) != Some(&b']') {
        return Err(());
    }

    buffer.depth = buffer.depth.saturating_sub(1);
    buffer.offset = buffer.offset.saturating_add(1);
    Ok(Value::Array(elements))
}

fn parse_object(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return Err(());
    }
    buffer.depth = buffer.depth.saturating_add(1);

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset) != Some(&b'{') {
        return Err(());
    }

    buffer.offset = buffer.offset.saturating_add(1);
    skip_whitespace(buffer);

    if buffer.offset < buffer.length && buffer.content.get(buffer.offset) == Some(&b'}') {
        buffer.depth = buffer.depth.saturating_sub(1);
        buffer.offset = buffer.offset.saturating_add(1);
        return Ok(Value::Object(Vec::new()));
    }

    if buffer.offset >= buffer.length {
        if buffer.offset > 0 {
            buffer.offset -= 1;
        }
        return Err(());
    }

    buffer.offset = buffer.offset.saturating_sub(1);

    let mut members = Vec::new();
    loop {
        if buffer.offset.saturating_add(1) >= buffer.length {
            return Err(());
        }

        buffer.offset = buffer.offset.saturating_add(1);
        skip_whitespace(buffer);

        let key = match parse_string(buffer) {
            Ok(Value::String(k)) => k,
            _ => return Err(()),
        };
        skip_whitespace(buffer);

        if buffer.offset >= buffer.length || buffer.content.get(buffer.offset) != Some(&b':') {
            return Err(());
        }

        buffer.offset = buffer.offset.saturating_add(1);
        skip_whitespace(buffer);

        let val = parse_value(buffer)?;
        members.push((key, val));
        skip_whitespace(buffer);

        if buffer.offset < buffer.length && buffer.content.get(buffer.offset) == Some(&b',') {
            continue;
        } else {
            break;
        }
    }

    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset) != Some(&b'}') {
        return Err(());
    }

    buffer.depth = buffer.depth.saturating_sub(1);
    buffer.offset = buffer.offset.saturating_add(1);
    Ok(Value::Object(members))
}

fn parse_value(buffer: &mut ParseBuffer) -> Result<Value, ()> {
    if buffer.offset >= buffer.length {
        return Err(());
    }

    let remaining = buffer.length.saturating_sub(buffer.offset);

    if remaining >= 4 && buffer.content.get(buffer.offset..buffer.offset.saturating_add(4)) == Some(b"null") {
        buffer.offset = buffer.offset.saturating_add(4);
        return Ok(Value::Null);
    }

    if remaining >= 5 && buffer.content.get(buffer.offset..buffer.offset.saturating_add(5)) == Some(b"false") {
        buffer.offset = buffer.offset.saturating_add(5);
        return Ok(Value::False);
    }

    if remaining >= 4 && buffer.content.get(buffer.offset..buffer.offset.saturating_add(4)) == Some(b"true") {
        buffer.offset = buffer.offset.saturating_add(4);
        return Ok(Value::True);
    }

    let b = match buffer.content.get(buffer.offset) {
        Some(&val) => val,
        None => return Err(()),
    };

    if b == b'\"' {
        return parse_string(buffer);
    }

    if b == b'-' || b.is_ascii_digit() {
        return parse_number(buffer);
    }

    if b == b'[' {
        return parse_array(buffer);
    }

    if b == b'{' {
        return parse_object(buffer);
    }

    Err(())
}

pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut content = Vec::with_capacity(input.len().saturating_add(1));
    content.extend_from_slice(input);
    content.push(b'\0');

    let total_len = content.len();
    let mut buffer = ParseBuffer {
        content: &content,
        length: total_len,
        offset: 0,
        depth: 0,
    };

    skip_utf8_bom(&mut buffer);
    skip_whitespace(&mut buffer);

    let val = match parse_value(&mut buffer) {
        Ok(v) => v,
        Err(_) => {
            let error_offset = if buffer.offset < buffer.length {
                buffer.offset
            } else if buffer.length > 0 {
                buffer.length.saturating_sub(1)
            } else {
                0
            };
            return Err(ParseError {
                offset: Some(error_offset),
            });
        }
    };

    skip_whitespace(&mut buffer);
    if buffer.offset >= buffer.length || buffer.content.get(buffer.offset) != Some(&b'\0') {
        let error_offset = if buffer.offset < buffer.length {
            buffer.offset
        } else if buffer.length > 0 {
            buffer.length.saturating_sub(1)
        } else {
            0
        };
        return Err(ParseError {
            offset: Some(error_offset),
        });
    }

    Ok(val)
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = a.abs().max(b.abs());
    (a - b).abs() <= max_val * f64::EPSILON
}

fn format_g(d: f64, precision: usize) -> String {
    let prec_minus_one = if precision > 0 { precision - 1 } else { 0 };
    let s = format!("{:.*e}", prec_minus_one, d);
    let (is_negative, rest) = if s.starts_with('-') {
        (true, s.get(1..).unwrap_or(""))
    } else {
        (false, s.as_str())
    };

    let mut parts = rest.split('e');
    let mantissa_str = parts.next().unwrap_or("");
    let exp_str = parts.next().unwrap_or("0");
    let exp: i32 = exp_str.parse().unwrap_or(0);

    let mut digits = Vec::with_capacity(precision);
    for b in mantissa_str.bytes() {
        if b != b'.' {
            digits.push(b);
        }
    }

    let mut result = String::with_capacity(32);
    if is_negative {
        result.push('-');
    }

    if exp < -4 || exp >= (precision as i32) {
        if let Some(&first_digit) = digits.first() {
            result.push(first_digit as char);
        }
        let frac = digits.get(1..).unwrap_or(&[]);
        if let Some(last_nz) = frac.iter().rposition(|&b| b != b'0') {
            result.push('.');
            if let Some(sub) = frac.get(..=last_nz) {
                for &b in sub {
                    result.push(b as char);
                }
            }
        }
        result.push('e');
        if exp >= 0 {
            result.push('+');
            if exp < 10 {
                result.push('0');
            }
            result.push_str(&exp.to_string());
        } else {
            result.push('-');
            let abs_exp = exp.unsigned_abs();
            if abs_exp < 10 {
                result.push('0');
            }
            result.push_str(&abs_exp.to_string());
        }
    } else if exp >= 0 {
        let num_before = (exp as usize).saturating_add(1);
        if let Some(sub) = digits.get(..num_before) {
            for &b in sub {
                result.push(b as char);
            }
        }
        let frac = digits.get(num_before..).unwrap_or(&[]);
        if let Some(last_nz) = frac.iter().rposition(|&b| b != b'0') {
            result.push('.');
            if let Some(sub) = frac.get(..=last_nz) {
                for &b in sub {
                    result.push(b as char);
                }
            }
        }
    } else {
        result.push_str("0.");
        let num_zeros = (-exp as usize).saturating_sub(1);
        for _ in 0..num_zeros {
            result.push('0');
        }
        let last_nz = digits.iter().rposition(|&b| b != b'0').unwrap_or(0);
        if let Some(sub) = digits.get(..=last_nz) {
            for &b in sub {
                result.push(b as char);
            }
        }
    }

    result
}

fn print_number(d: f64, out: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        out.extend_from_slice(b"null");
        return;
    }

    let valueint = if d >= (i32::MAX as f64) {
        i32::MAX
    } else if d <= (i32::MIN as f64) {
        i32::MIN
    } else {
        d as i32
    };

    if d == (valueint as f64) {
        let s = valueint.to_string();
        out.extend_from_slice(s.as_bytes());
        return;
    }

    let s15 = format_g(d, 15);
    let recovered: Option<f64> = s15.parse().ok();
    let matches = match recovered {
        Some(test) => compare_double(test, d),
        None => false,
    };

    let s = if matches {
        s15
    } else {
        format_g(d, 17)
    };

    out.extend_from_slice(s.as_bytes());
}

fn print_string_ptr(s: &[u8], out: &mut Vec<u8>) {
    let end = s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let slice = s.get(..end).unwrap_or(s);

    out.push(b'\"');
    for &b in slice {
        match b {
            b'\"' => {
                out.push(b'\\');
                out.push(b'\"');
            }
            b'\\' => {
                out.push(b'\\');
                out.push(b'\\');
            }
            0x08 => {
                out.push(b'\\');
                out.push(b'b');
            }
            0x0C => {
                out.push(b'\\');
                out.push(b'f');
            }
            0x0A => {
                out.push(b'\\');
                out.push(b'n');
            }
            0x0D => {
                out.push(b'\\');
                out.push(b'r');
            }
            0x09 => {
                out.push(b'\\');
                out.push(b't');
            }
            0..=31 => {
                let hex = format!("\\u{:04x}", b);
                out.extend_from_slice(hex.as_bytes());
            }
            _ => {
                out.push(b);
            }
        }
    }
    out.push(b'\"');
}

fn print_value_to_buf(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => {
            out.extend_from_slice(b"null");
        }
        Value::False => {
            out.extend_from_slice(b"false");
        }
        Value::True => {
            out.extend_from_slice(b"true");
        }
        Value::Number(d) => {
            print_number(*d, out);
        }
        Value::String(s) => {
            print_string_ptr(s, out);
        }
        Value::Raw(raw) => {
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            if let Some(slice) = raw.get(..end) {
                out.extend_from_slice(slice);
            }
        }
        Value::Array(arr) => {
            out.push(b'[');
            for (i, elem) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_value_to_buf(elem, out);
            }
            out.push(b']');
        }
        Value::Object(obj) => {
            out.push(b'{');
            for (i, (key, val)) in obj.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_string_ptr(key, out);
                out.push(b':');
                print_value_to_buf(val, out);
            }
            out.push(b'}');
        }
    }
}

pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    print_value_to_buf(value, &mut out);
    out
}
