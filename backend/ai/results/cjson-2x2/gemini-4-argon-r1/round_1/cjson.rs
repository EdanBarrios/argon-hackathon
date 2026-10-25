//! cJSON v1.7.19 parse and print implementation in safe Rust.

#![allow(dead_code)]

/// Mirrors CJSON_NESTING_LIMIT in cJSON.h.
pub const CJSON_NESTING_LIMIT: usize = 1000;

/// One cJSON node. Variants mirror the cJSON type flags.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    False,
    True,
    Null,
    /// cJSON keeps valuedouble (used for printing) and a saturated valueint.
    Number(f64),
    /// Raw bytes, not necessarily UTF-8.
    String(Vec<u8>),
    /// cJSON_Raw: printed verbatim. Never produced by parse.
    Raw(Vec<u8>),
    Array(Vec<Value>),
    /// Insertion order preserved, duplicate keys kept.
    Object(Vec<(Vec<u8>, Value)>),
}

/// Parse failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Byte offset of cJSON's error pointer into the input, if any.
    pub offset: Option<usize>,
}

struct ParseBuffer {
    content: Vec<u8>,
    length: usize,
    offset: usize,
    depth: usize,
}

impl ParseBuffer {
    #[inline]
    fn can_read(&self, size: usize) -> bool {
        self.offset.checked_add(size).is_some_and(|end| end <= self.length)
    }

    #[inline]
    fn can_access_at_index(&self, index: usize) -> bool {
        self.offset.checked_add(index).is_some_and(|pos| pos < self.length)
    }

    #[inline]
    fn cannot_access_at_index(&self, index: usize) -> bool {
        !self.can_access_at_index(index)
    }

    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.content[self.offset + index]
    }

    fn skip_utf8_bom(&mut self) {
        if self.offset != 0 {
            return;
        }
        if self.can_access_at_index(4)
            && self.content[self.offset..self.offset + 3] == *b"\xEF\xBB\xBF"
        {
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

/// Parses `input` with the exact semantics of:
/// `cJSON_ParseWithLengthOpts(input_with_nul, input.len() + 1, &end, 1)`.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let buffer_length = input.len() + 1;
    let mut content = Vec::with_capacity(buffer_length);
    content.extend_from_slice(input);
    content.push(0);

    let mut buffer = ParseBuffer {
        content,
        length: buffer_length,
        offset: 0,
        depth: 0,
    };

    buffer.skip_utf8_bom();
    buffer.skip_whitespace();

    if let Some(value) = parse_value(&mut buffer) {
        buffer.skip_whitespace();
        if buffer.offset < buffer.length && buffer.byte_at(0) == b'\0' {
            return Ok(value);
        }
    }

    let position = if buffer.offset < buffer.length {
        buffer.offset
    } else if buffer.length > 0 {
        buffer.length - 1
    } else {
        0
    };

    Err(ParseError {
        offset: Some(position),
    })
}

fn parse_value(input_buffer: &mut ParseBuffer) -> Option<Value> {
    if input_buffer.can_read(4)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 4] == b"null"
    {
        input_buffer.offset += 4;
        return Some(Value::Null);
    }
    if input_buffer.can_read(5)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 5] == b"false"
    {
        input_buffer.offset += 5;
        return Some(Value::False);
    }
    if input_buffer.can_read(4)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 4] == b"true"
    {
        input_buffer.offset += 4;
        return Some(Value::True);
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'"' {
        return parse_string(input_buffer).map(Value::String);
    }
    if input_buffer.can_access_at_index(0) {
        let b = input_buffer.byte_at(0);
        if b == b'-' || b.is_ascii_digit() {
            return parse_number(input_buffer).map(Value::Number);
        }
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'[' {
        return parse_array(input_buffer).map(Value::Array);
    }
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'{' {
        return parse_object(input_buffer).map(Value::Object);
    }

    None
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for i in 0..4 {
        let b = input[i];
        if b.is_ascii_digit() {
            h += u32::from(b - b'0');
        } else if (b'A'..=b'F').contains(&b) {
            h += 10 + u32::from(b - b'A');
        } else if (b'a'..=b'f').contains(&b) {
            h += 10 + u32::from(b - b'a');
        } else {
            return 0;
        }
        if i < 3 {
            h <<= 4;
        }
    }
    h
}

fn utf16_literal_to_utf8(
    content: &[u8],
    first_sequence: usize,
    input_end: usize,
    output: &mut Vec<u8>,
) -> usize {
    if input_end.saturating_sub(first_sequence) < 6 {
        return 0;
    }

    let first_code = parse_hex4(&content[first_sequence + 2..first_sequence + 6]);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let (sequence_length, mut codepoint) = if (0xD800..=0xDBFF).contains(&first_code) {
        let second_sequence = first_sequence + 6;
        if input_end.saturating_sub(second_sequence) < 6 {
            return 0;
        }
        if content[second_sequence] != b'\\' || content[second_sequence + 1] != b'u' {
            return 0;
        }
        let second_code = parse_hex4(&content[second_sequence + 2..second_sequence + 6]);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (12usize, cp)
    } else {
        (6usize, first_code)
    };

    let (utf8_length, first_byte_mark): (usize, u32) = if codepoint < 0x80 {
        (1, 0)
    } else if codepoint < 0x800 {
        (2, 0xC0)
    } else if codepoint < 0x10000 {
        (3, 0xE0)
    } else if codepoint <= 0x10FFFF {
        (4, 0xF0)
    } else {
        return 0;
    };

    let mut encoded = [0u8; 4];
    let mut pos = utf8_length - 1;
    while pos > 0 {
        encoded[pos] = ((codepoint | 0x80) & 0xBF) as u8;
        codepoint >>= 6;
        pos -= 1;
    }
    if utf8_length > 1 {
        encoded[0] = ((codepoint | first_byte_mark) & 0xFF) as u8;
    } else {
        encoded[0] = (codepoint & 0x7F) as u8;
    }

    output.extend_from_slice(&encoded[..utf8_length]);
    sequence_length
}

fn parse_string(input_buffer: &mut ParseBuffer) -> Option<Vec<u8>> {
    let mut input_pointer = input_buffer.offset + 1;
    let mut input_end = input_buffer.offset + 1;

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b'"' {
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

    let allocation_length = (input_end - input_buffer.offset).saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(allocation_length);

    while input_pointer < input_end {
        let b = input_buffer.content[input_pointer];
        if b != b'\\' {
            output.push(b);
            input_pointer += 1;
        } else {
            if input_end - input_pointer < 2 {
                input_buffer.offset = input_pointer;
                return None;
            }
            let mut sequence_length = 2usize;
            match input_buffer.content[input_pointer + 1] {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'"' | b'\\' | b'/' => output.push(input_buffer.content[input_pointer + 1]),
                b'u' => {
                    sequence_length = utf16_literal_to_utf8(
                        &input_buffer.content,
                        input_pointer,
                        input_end,
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

fn strtod_prefix_len(s: &[u8]) -> usize {
    let mut i = 0;
    if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
        i += 1;
    }
    let mut digits = 0usize;
    while i < s.len() && s[i].is_ascii_digit() {
        i += 1;
        digits += 1;
    }
    if i < s.len() && s[i] == b'.' {
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0;
    }
    let sig_end = i;
    if i < s.len() && (s[i] == b'e' || s[i] == b'E') {
        i += 1;
        if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
            i += 1;
        }
        let mut exp_digits = 0usize;
        while i < s.len() && s[i].is_ascii_digit() {
            i += 1;
            exp_digits += 1;
        }
        if exp_digits == 0 {
            return sig_end;
        }
    }
    i
}

fn parse_number(input_buffer: &mut ParseBuffer) -> Option<f64> {
    let mut number_string_length = 0usize;
    while input_buffer.can_access_at_index(number_string_length) {
        match input_buffer.byte_at(number_string_length) {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
    }

    let num_slice =
        &input_buffer.content[input_buffer.offset..input_buffer.offset + number_string_length];
    let consumed = strtod_prefix_len(num_slice);
    if consumed == 0 {
        return None;
    }

    let valid_str = std::str::from_utf8(&num_slice[..consumed]).ok()?;
    let number = valid_str.parse::<f64>().ok()?;

    input_buffer.offset += consumed;
    Some(number)
}

fn parse_array(input_buffer: &mut ParseBuffer) -> Option<Vec<Value>> {
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

fn parse_object(input_buffer: &mut ParseBuffer) -> Option<Vec<(Vec<u8>, Value)>> {
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

/// Renders `value` to unformatted JSON bytes matching `cJSON_PrintUnformatted`.
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
        Value::Array(items) => print_array(items, out),
        Value::Object(members) => print_object(members, out),
    }
}

#[inline]
fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}

#[inline]
fn saturated_valueint(d: f64) -> i32 {
    if d >= (i32::MAX as f64) {
        i32::MAX
    } else if d <= (i32::MIN as f64) {
        i32::MIN
    } else {
        d as i32
    }
}

fn format_g(d: f64, precision: usize, out: &mut Vec<u8>) {
    let sci = format!("{:.*e}", precision - 1, d);
    let bytes = sci.as_bytes();
    let (negative, rest) = if !bytes.is_empty() && bytes[0] == b'-' {
        (true, &bytes[1..])
    } else {
        (false, bytes)
    };

    let e_pos = match rest.iter().position(|&b| b == b'e') {
        Some(p) => p,
        None => return,
    };
    let mantissa = &rest[..e_pos];
    let exp_str = match std::str::from_utf8(&rest[e_pos + 1..]) {
        Ok(s) => s,
        Err(_) => return,
    };
    let exp: i32 = match exp_str.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut digits = [0u8; 24];
    let mut num_digits = 0usize;
    for &b in mantissa {
        if b != b'.' && num_digits < digits.len() {
            digits[num_digits] = b;
            num_digits += 1;
        }
    }

    let mut sig_len = num_digits;
    while sig_len > 1 && digits[sig_len - 1] == b'0' {
        sig_len -= 1;
    }

    if negative {
        out.push(b'-');
    }

    let prec_i32 = precision as i32;
    if exp < -4 || exp >= prec_i32 {
        out.push(digits[0]);
        if sig_len > 1 {
            out.push(b'.');
            out.extend_from_slice(&digits[1..sig_len]);
        }
        out.push(b'e');
        if exp >= 0 {
            out.push(b'+');
        } else {
            out.push(b'-');
        }
        let abs_exp = exp.unsigned_abs();
        if abs_exp < 10 {
            out.push(b'0');
            out.push(b'0' + (abs_exp as u8));
        } else if abs_exp < 100 {
            out.push(b'0' + ((abs_exp / 10) as u8));
            out.push(b'0' + ((abs_exp % 10) as u8));
        } else {
            out.extend_from_slice(abs_exp.to_string().as_bytes());
        }
    } else if exp < 0 {
        out.extend_from_slice(b"0.");
        let leading_zeros = (-1 - exp) as usize;
        for _ in 0..leading_zeros {
            out.push(b'0');
        }
        out.extend_from_slice(&digits[..sig_len]);
    } else {
        let int_digits = (exp + 1) as usize;
        if sig_len <= int_digits {
            out.extend_from_slice(&digits[..sig_len]);
            for _ in 0..(int_digits - sig_len) {
                out.push(b'0');
            }
        } else {
            out.extend_from_slice(&digits[..int_digits]);
            out.push(b'.');
            out.extend_from_slice(&digits[int_digits..sig_len]);
        }
    }
}

fn print_number(d: f64, out: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        out.extend_from_slice(b"null");
        return;
    }

    let valueint = saturated_valueint(d);
    if d == (valueint as f64) {
        out.extend_from_slice(valueint.to_string().as_bytes());
        return;
    }

    let mut buf15 = Vec::with_capacity(26);
    format_g(d, 15, &mut buf15);

    let ok15 = std::str::from_utf8(&buf15)
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .is_some_and(|test| compare_double(test, d));

    if ok15 {
        out.extend_from_slice(&buf15);
    } else {
        format_g(d, 17, out);
    }
}

fn print_string_ptr(input: &[u8], out: &mut Vec<u8>) {
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());
    let slice = &input[..end];

    let mut escape_characters = 0usize;
    for &b in slice {
        match b {
            b'"' | b'\\' | 0x08 | 0x0C | b'\n' | b'\r' | b'\t' => {
                escape_characters += 1;
            }
            _ if b < 32 => {
                escape_characters += 5;
            }
            _ => {}
        }
    }

    out.reserve(slice.len() + escape_characters + 2);
    out.push(b'"');

    if escape_characters == 0 {
        out.extend_from_slice(slice);
        out.push(b'"');
        return;
    }

    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
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
                    out.push(b'u');
                    out.push(b'0');
                    out.push(b'0');
                    out.push(HEX_DIGITS[((b >> 4) & 0x0F) as usize]);
                    out.push(HEX_DIGITS[(b & 0x0F) as usize]);
                }
            }
        }
    }
    out.push(b'"');
}

fn print_array(items: &[Value], out: &mut Vec<u8>) {
    out.push(b'[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(b',');
        }
        print_value(item, out);
    }
    out.push(b']');
}

fn print_object(members: &[(Vec<u8>, Value)], out: &mut Vec<u8>) {
    out.push(b'{');
    for (i, (key, val)) in members.iter().enumerate() {
        if i > 0 {
            out.push(b',');
        }
        print_string_ptr(key, out);
        out.push(b':');
        print_value(val, out);
    }
    out.push(b'}');
}
