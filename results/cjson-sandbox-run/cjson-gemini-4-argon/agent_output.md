ROUND 1 build=ok match_rate=0.9933333333333333
ROUND 2 build=ok match_rate=1.0
=== VERIFY_JSON_BEGIN ===
{"match_rate":1.0,"behaviors_tested":300,"counts":{"match":300,"divergence":0,"safety_improvement":0,"rust_crash":0,"both_crash":0,"timeout":0},"divergences":[],"crashes":[]}
=== VERIFY_JSON_END ===
```rust
#![allow(dead_code)]

pub const CJSON_NESTING_LIMIT: usize = 1000;

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
    fn can_read(&self, size: usize) -> bool {
        self.offset.saturating_add(size) <= self.length
    }

    fn can_access_at_index(&self, index: usize) -> bool {
        self.offset.saturating_add(index) < self.length
    }

    fn cannot_access_at_index(&self, index: usize) -> bool {
        !self.can_access_at_index(index)
    }

    fn byte_at(&self, index: usize) -> u8 {
        self.content[self.offset + index]
    }

    fn slice_from_offset(&self) -> &'a [u8] {
        &self.content[self.offset..]
    }

    fn skip_utf8_bom(&mut self) {
        if self.offset != 0 {
            return;
        }
        if self.can_access_at_index(4) && self.slice_from_offset().starts_with(b"\xEF\xBB\xBF") {
            self.offset += 3;
        }
    }

    fn skip_whitespace(&mut self) {
        if self.cannot_access_at_index(0) {
            return;
        }
        while self.can_access_at_index(0) && self.byte_at(0) <= 32 {
            self.offset += 1;
        }
        if self.offset == self.length {
            self.offset -= 1;
        }
    }
}

pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut owned = Vec::with_capacity(input.len() + 1);
    owned.extend_from_slice(input);
    owned.push(0);

    let mut buffer = ParseBuffer {
        content: &owned,
        length: owned.len(),
        offset: 0,
        depth: 0,
    };

    buffer.skip_utf8_bom();
    buffer.skip_whitespace();

    let value = match parse_value(&mut buffer) {
        Some(v) => v,
        None => return Err(make_error(&buffer)),
    };

    buffer.skip_whitespace();
    if buffer.offset >= buffer.length || buffer.byte_at(0) != 0 {
        return Err(make_error(&buffer));
    }

    Ok(value)
}

fn make_error(buffer: &ParseBuffer<'_>) -> ParseError {
    let pos = if buffer.offset < buffer.length {
        buffer.offset
    } else if buffer.length > 0 {
        buffer.length - 1
    } else {
        0
    };
    ParseError { offset: Some(pos) }
}

fn strncmp_ascii(slice: &[u8], pat: &[u8]) -> bool {
    if slice.len() < pat.len() {
        return false;
    }
    for i in 0..pat.len() {
        if slice[i] != pat[i] {
            return false;
        }
        if slice[i] == 0 {
            return true;
        }
    }
    true
}

fn parse_value(input_buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if input_buffer.can_read(4) && strncmp_ascii(input_buffer.slice_from_offset(), b"null") {
        input_buffer.offset += 4;
        return Some(Value::Null);
    }
    if input_buffer.can_read(5) && strncmp_ascii(input_buffer.slice_from_offset(), b"false") {
        input_buffer.offset += 5;
        return Some(Value::False);
    }
    if input_buffer.can_read(4) && strncmp_ascii(input_buffer.slice_from_offset(), b"true") {
        input_buffer.offset += 4;
        return Some(Value::True);
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'"' {
        return parse_string(input_buffer).map(Value::String);
    }
    if input_buffer.can_access_at_index(0)
        && (input_buffer.byte_at(0) == b'-'
            || (input_buffer.byte_at(0) >= b'0' && input_buffer.byte_at(0) <= b'9'))
    {
        return parse_number(input_buffer).map(Value::Number);
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'[' {
        return parse_array(input_buffer).map(Value::Array);
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'{' {
        return parse_object(input_buffer).map(Value::Object);
    }
    None
}

fn parse_number(input_buffer: &mut ParseBuffer<'_>) -> Option<f64> {
    let mut number_string_length = 0usize;
    let mut i = 0usize;
    while input_buffer.can_access_at_index(i) {
        match input_buffer.byte_at(i) {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
        i += 1;
    }

    let num_slice = &input_buffer.slice_from_offset()[..number_string_length];
    let (number, consumed) = strtod_num_slice(num_slice)?;
    if consumed == 0 {
        return None;
    }

    input_buffer.offset += consumed;
    Some(number)
}

fn strtod_num_slice(s: &[u8]) -> Option<(f64, usize)> {
    if s.is_empty() {
        return None;
    }
    let mut idx = 0usize;
    if s[idx] == b'+' || s[idx] == b'-' {
        idx += 1;
    }
    let mut digits = 0usize;
    while idx < s.len() && s[idx].is_ascii_digit() {
        idx += 1;
        digits += 1;
    }
    if idx < s.len() && s[idx] == b'.' {
        idx += 1;
        while idx < s.len() && s[idx].is_ascii_digit() {
            idx += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return None;
    }
    let after_significand = idx;
    if idx < s.len() && (s[idx] == b'e' || s[idx] == b'E') {
        idx += 1;
        if idx < s.len() && (s[idx] == b'+' || s[idx] == b'-') {
            idx += 1;
        }
        let mut exp_digits = 0usize;
        while idx < s.len() && s[idx].is_ascii_digit() {
            idx += 1;
            exp_digits += 1;
        }
        if exp_digits == 0 {
            idx = after_significand;
        }
    }
    let valid_str = std::str::from_utf8(&s[..idx]).ok()?;
    let val = valid_str.parse::<f64>().ok()?;
    Some((val, idx))
}

fn parse_hex4(input: &[u8]) -> u32 {
    if input.len() < 4 {
        return 0;
    }
    let mut h: u32 = 0;
    for i in 0..4 {
        let b = input[i];
        let d = if b >= b'0' && b <= b'9' {
            (b - b'0') as u32
        } else if b >= b'A' && b <= b'F' {
            10 + (b - b'A') as u32
        } else if b >= b'a' && b <= b'f' {
            10 + (b - b'a') as u32
        } else {
            return 0;
        };
        h += d;
        if i < 3 {
            h <<= 4;
        }
    }
    h
}

fn utf16_literal_to_utf8(input_slice: &[u8], output: &mut Vec<u8>) -> usize {
    if input_slice.len() < 6 {
        return 0;
    }
    let first_code = parse_hex4(&input_slice[2..6]);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }
    let (sequence_length, mut codepoint) = if (0xD800..=0xDBFF).contains(&first_code) {
        if input_slice.len() < 12 {
            return 0;
        }
        if input_slice[6] != b'\\' || input_slice[7] != b'u' {
            return 0;
        }
        let second_code = parse_hex4(&input_slice[8..12]);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (12usize, cp)
    } else {
        (6usize, first_code)
    };

    let (utf8_length, first_byte_mark) = if codepoint < 0x80 {
        (1usize, 0u8)
    } else if codepoint < 0x800 {
        (2usize, 0xC0u8)
    } else if codepoint < 0x10000 {
        (3usize, 0xE0u8)
    } else if codepoint <= 0x10FFFF {
        (4usize, 0xF0u8)
    } else {
        return 0;
    };

    let mut buf = [0u8; 4];
    for pos in (1..utf8_length).rev() {
        buf[pos] = ((codepoint | 0x80) & 0xBF) as u8;
        codepoint >>= 6;
    }
    if utf8_length > 1 {
        buf[0] = ((codepoint | (first_byte_mark as u32)) & 0xFF) as u8;
    } else {
        buf[0] = (codepoint & 0x7F) as u8;
    }
    output.extend_from_slice(&buf[..utf8_length]);
    sequence_length
}

fn parse_string(input_buffer: &mut ParseBuffer<'_>) -> Option<Vec<u8>> {
    let start_offset = input_buffer.offset;
    let mut input_pointer = start_offset + 1;
    let mut input_end = start_offset + 1;

    if input_buffer.byte_at(0) != b'"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let mut skipped_bytes = 0usize;
    while input_end < input_buffer.length && input_buffer.content[input_end] != b'"' {
        if input_buffer.content[input_end] == b'\\' {
            if input_end + 1 >= input_buffer.length {
                input_buffer.offset = input_pointer;
                return None;
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }
    if input_end >= input_buffer.length || input_buffer.content[input_end] != b'"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let allocation_length = (input_end - start_offset).saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(allocation_length);

    while input_pointer < input_end {
        if input_buffer.content[input_pointer] != b'\\' {
            output.push(input_buffer.content[input_pointer]);
            input_pointer += 1;
        } else {
            let mut sequence_length = 2usize;
            if input_end - input_pointer < 1 {
                input_buffer.offset = input_pointer;
                return None;
            }
            match input_buffer.content[input_pointer + 1] {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'"' | b'\\' | b'/' => output.push(input_buffer.content[input_pointer + 1]),
                b'u' => {
                    sequence_length = utf16_literal_to_utf8(
                        &input_buffer.content[input_pointer..input_end],
                        &mut output,
                    );
                    if sequence_length == 0 {
                        input_buffer.offset = input_pointer;
                        return None;
                    }
                }
                _ => {
                    input_buffer.offset = input_pointer;
                    return None;
                }
            }
            input_pointer += sequence_length;
        }
    }

    input_buffer.offset = input_end + 1;
    Some(output)
}

fn parse_array(input_buffer: &mut ParseBuffer<'_>) -> Option<Vec<Value>> {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    input_buffer.depth += 1;

    if input_buffer.byte_at(0) != b'[' {
        return None;
    }

    input_buffer.offset += 1;
    input_buffer.skip_whitespace();
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b']' {
        input_buffer.depth -= 1;
        input_buffer.offset += 1;
        return Some(Vec::new());
    }

    if input_buffer.cannot_access_at_index(0) {
        input_buffer.offset -= 1;
        return None;
    }

    input_buffer.offset -= 1;
    let mut elements = Vec::new();
    loop {
        input_buffer.offset += 1;
        input_buffer.skip_whitespace();
        let val = parse_value(input_buffer)?;
        elements.push(val);
        input_buffer.skip_whitespace();

        if !(input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b',') {
            break;
        }
    }

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b']' {
        return None;
    }

    input_buffer.depth -= 1;
    input_buffer.offset += 1;
    Some(elements)
}

fn parse_object(input_buffer: &mut ParseBuffer<'_>) -> Option<Vec<(Vec<u8>, Value)>> {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    input_buffer.depth += 1;

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b'{' {
        return None;
    }

    input_buffer.offset += 1;
    input_buffer.skip_whitespace();
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'}' {
        input_buffer.depth -= 1;
        input_buffer.offset += 1;
        return Some(Vec::new());
    }

    if input_buffer.cannot_access_at_index(0) {
        input_buffer.offset -= 1;
        return None;
    }

    input_buffer.offset -= 1;
    let mut members = Vec::new();
    loop {
        if input_buffer.cannot_access_at_index(1) {
            return None;
        }

        input_buffer.offset += 1;
        input_buffer.skip_whitespace();
        let key = parse_string(input_buffer)?;
        input_buffer.skip_whitespace();

        if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b':' {
            return None;
        }

        input_buffer.offset += 1;
        input_buffer.skip_whitespace();
        let val = parse_value(input_buffer)?;
        members.push((key, val));
        input_buffer.skip_whitespace();

        if !(input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b',') {
            break;
        }
    }

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b'}' {
        return None;
    }

    input_buffer.depth -= 1;
    input_buffer.offset += 1;
    Some(members)
}

pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    print_value(value, &mut out);
    out
}

fn print_value(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::False => out.extend_from_slice(b"false"),
        Value::True => out.extend_from_slice(b"true"),
        Value::Number(d) => print_number(*d, out),
        Value::Raw(raw) => {
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            out.extend_from_slice(&raw[..end]);
        }
        Value::String(s) => print_string_ptr(s, out),
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
        Value::Object(members) => {
            out.push(b'{');
            for (i, (k, v)) in members.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_string_ptr(k, out);
                out.push(b':');
                print_value(v, out);
            }
            out.push(b'}');
        }
    }
}

fn print_string_ptr(input: &[u8], out: &mut Vec<u8>) {
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());
    let slice = &input[..end];

    out.push(b'"');
    for &b in slice {
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
                    let hex = format!("u{:04x}", b);
                    out.extend_from_slice(hex.as_bytes());
                }
            }
        }
    }
    out.push(b'"');
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
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
        out.extend_from_slice(valueint.to_string().as_bytes());
        return;
    }

    let s15 = format_g(d, 15);
    if let Ok(test) = s15.parse::<f64>() {
        if compare_double(test, d) {
            out.extend_from_slice(s15.as_bytes());
            return;
        }
    }

    let s17 = format_g(d, 17);
    out.extend_from_slice(s17.as_bytes());
}

fn format_g(d: f64, precision: usize) -> String {
    let sci = format!("{:.*e}", precision - 1, d);
    let (neg, rest) = if let Some(r) = sci.strip_prefix('-') {
        (true, r)
    } else {
        (false, sci.as_str())
    };

    let (mantissa, exp_str) = match rest.split_once('e') {
        Some(pair) => pair,
        None => return sci,
    };
    let exp: i32 = exp_str.parse().unwrap_or(0);

    let mut digits = String::with_capacity(precision);
    for ch in mantissa.chars() {
        if ch != '.' {
            digits.push(ch);
        }
    }

    let mut result = String::with_capacity(26);
    if neg {
        result.push('-');
    }

    if exp < -4 || exp >= (precision as i32) {
        let trimmed = digits.trim_end_matches('0');
        let first = &digits[0..1];
        result.push_str(first);
        if trimmed.len() > 1 {
            result.push('.');
            result.push_str(&trimmed[1..]);
        }
        result.push('e');
        if exp >= 0 {
            result.push('+');
        } else {
            result.push('-');
        }
        let abs_exp = exp.unsigned_abs();
        if abs_exp < 10 {
            result.push('0');
        }
        result.push_str(&abs_exp.to_string());
    } else if exp < 0 {
        result.push_str("0.");
        for _ in 0..(-1 - exp) {
            result.push('0');
        }
        let trimmed = digits.trim_end_matches('0');
        result.push_str(trimmed);
    } else {
        let split_idx = (exp as usize) + 1;
        if split_idx >= digits.len() {
            result.push_str(&digits);
            for _ in digits.len()..split_idx {
                result.push('0');
            }
        } else {
            result.push_str(&digits[..split_idx]);
            let frac = digits[split_idx..].trim_end_matches('0');
            if !frac.is_empty() {
                result.push('.');
                result.push_str(frac);
            }
        }
    }

    result
}
```