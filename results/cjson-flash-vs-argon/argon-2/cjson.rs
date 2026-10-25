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

struct ParseBuffer<'a> {
    content: &'a [u8],
    length: usize,
    offset: usize,
    depth: usize,
}

impl<'a> ParseBuffer<'a> {
    #[inline]
    fn can_read(&self, size: usize) -> bool {
        self.offset.saturating_add(size) <= self.length
    }

    #[inline]
    fn can_access_at_index(&self, index: usize) -> bool {
        self.offset.saturating_add(index) < self.length
    }

    #[inline]
    fn cannot_access_at_index(&self, index: usize) -> bool {
        !self.can_access_at_index(index)
    }

    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        let pos = self.offset.saturating_add(index);
        self.content.get(pos).copied().unwrap_or(0)
    }

    #[inline]
    fn slice_from_offset(&self) -> &'a [u8] {
        if self.offset <= self.content.len() {
            &self.content[self.offset..]
        } else {
            &[]
        }
    }
}

/// Skip the UTF-8 BOM (byte order mark) if it is at the beginning of a buffer.
fn skip_utf8_bom(buffer: &mut ParseBuffer<'_>) {
    if buffer.offset != 0 {
        return;
    }
    if buffer.can_access_at_index(4) && buffer.slice_from_offset().starts_with(b"\xEF\xBB\xBF") {
        buffer.offset += 3;
    }
}

/// Utility to jump whitespace and cr/lf.
fn buffer_skip_whitespace(buffer: &mut ParseBuffer<'_>) {
    if buffer.cannot_access_at_index(0) {
        return;
    }

    while buffer.can_access_at_index(0) && buffer.byte_at(0) <= 32 {
        buffer.offset += 1;
    }

    if buffer.offset == buffer.length {
        buffer.offset -= 1;
    }
}

/// Parse 4 digit hexadecimal number.
/// Matches cJSON's `parse_hex4`: returns 0 on any non-hex character.
fn parse_hex4(input: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for i in 0..4 {
        let b = match input.get(i) {
            Some(&b) => b,
            None => return 0,
        };
        if b.is_ascii_digit() {
            h += (b - b'0') as u32;
        } else if (b'A'..=b'F').contains(&b) {
            h += 10 + (b - b'A') as u32;
        } else if (b'a'..=b'f').contains(&b) {
            h += 10 + (b - b'a') as u32;
        } else {
            return 0;
        }

        if i < 3 {
            h <<= 4;
        }
    }
    h
}

/// Converts a UTF-16 literal to UTF-8.
/// A literal can be one or two sequences of the form `\uXXXX`.
/// Returns the consumed sequence length (6 or 12), or 0 on failure.
fn utf16_literal_to_utf8(input_slice: &[u8], output: &mut Vec<u8>) -> usize {
    if input_slice.len() < 6 {
        return 0;
    }

    let first_code = parse_hex4(&input_slice[2..6]);

    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let (sequence_length, mut codepoint): (usize, u32) = if (0xD800..=0xDBFF).contains(&first_code)
    {
        if input_slice.len() < 12 {
            return 0;
        }
        let second_sequence = &input_slice[6..12];
        if second_sequence[0] != b'\\' || second_sequence[1] != b'u' {
            return 0;
        }
        let second_code = parse_hex4(&second_sequence[2..6]);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (12, cp)
    } else {
        (6, first_code)
    };

    let (utf8_length, first_byte_mark): (usize, u8) = if codepoint < 0x80 {
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
    let mut utf8_position = utf8_length - 1;
    while utf8_position > 0 {
        encoded[utf8_position] = ((codepoint | 0x80) & 0xBF) as u8;
        codepoint >>= 6;
        utf8_position -= 1;
    }

    if utf8_length > 1 {
        encoded[0] = ((codepoint | (first_byte_mark as u32)) & 0xFF) as u8;
    } else {
        encoded[0] = (codepoint & 0x7F) as u8;
    }

    output.extend_from_slice(&encoded[..utf8_length]);
    sequence_length
}

/// Parse the input text into an unescaped byte vector.
fn parse_string(input_buffer: &mut ParseBuffer<'_>) -> Option<Vec<u8>> {
    let start_offset = input_buffer.offset;
    let mut input_pointer = start_offset + 1;
    let mut input_end = start_offset + 1;

    if input_buffer.byte_at(0) != b'\"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let mut skipped_bytes: usize = 0;
    while input_end < input_buffer.length && input_buffer.content[input_end] != b'\"' {
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

    if input_end >= input_buffer.length || input_buffer.content[input_end] != b'\"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let allocation_length = (input_end - start_offset).saturating_sub(skipped_bytes);
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
            let esc = input_buffer.content[input_pointer + 1];
            let mut sequence_length: usize = 2;
            match esc {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'\"' | b'\\' | b'/' => output.push(esc),
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

/// Find the length of the valid `strtod` prefix of `s`, where `s` only contains
/// characters from `['0'-'9', '+', '-', 'e', 'E', '.']`.
fn strtod_prefix_len(s: &[u8]) -> usize {
    let len = s.len();
    let mut idx = 0;

    if idx < len && (s[idx] == b'+' || s[idx] == b'-') {
        idx += 1;
    }

    let mut int_digits = 0;
    while idx < len && s[idx].is_ascii_digit() {
        idx += 1;
        int_digits += 1;
    }

    let mut frac_digits = 0;
    if idx < len && s[idx] == b'.' {
        idx += 1;
        while idx < len && s[idx].is_ascii_digit() {
            idx += 1;
            frac_digits += 1;
        }
    }

    if int_digits == 0 && frac_digits == 0 {
        return 0;
    }

    let mut valid_len = idx;

    if idx < len && (s[idx] == b'e' || s[idx] == b'E') {
        let mut exp_idx = idx + 1;
        if exp_idx < len && (s[exp_idx] == b'+' || s[exp_idx] == b'-') {
            exp_idx += 1;
        }
        let mut exp_digits = 0;
        while exp_idx < len && s[exp_idx].is_ascii_digit() {
            exp_idx += 1;
            exp_digits += 1;
        }
        if exp_digits > 0 {
            valid_len = exp_idx;
        }
    }

    valid_len
}

/// Parse the input text to generate a number.
fn parse_number(input_buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    let mut number_string_length = 0;
    while input_buffer.can_access_at_index(number_string_length) {
        match input_buffer.byte_at(number_string_length) {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
    }

    let start = input_buffer.offset;
    let candidate = &input_buffer.content[start..start + number_string_length];
    let consumed = strtod_prefix_len(candidate);
    if consumed == 0 {
        return None;
    }

    let num_str = std::str::from_utf8(&candidate[..consumed]).ok()?;
    let number = num_str.parse::<f64>().ok()?;

    input_buffer.offset += consumed;
    Some(Value::Number(number))
}

/// Build an array from input text.
fn parse_array(input_buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    input_buffer.depth += 1;

    if input_buffer.byte_at(0) != b'[' {
        return None;
    }

    input_buffer.offset += 1;
    buffer_skip_whitespace(input_buffer);
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b']' {
        input_buffer.depth -= 1;
        input_buffer.offset += 1;
        return Some(Value::Array(Vec::new()));
    }

    if input_buffer.cannot_access_at_index(0) {
        input_buffer.offset -= 1;
        return None;
    }

    input_buffer.offset -= 1;
    let mut elements = Vec::new();
    loop {
        input_buffer.offset += 1;
        buffer_skip_whitespace(input_buffer);
        let elem = parse_value(input_buffer)?;
        elements.push(elem);
        buffer_skip_whitespace(input_buffer);

        if !(input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b',') {
            break;
        }
    }

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b']' {
        return None;
    }

    input_buffer.depth -= 1;
    input_buffer.offset += 1;
    Some(Value::Array(elements))
}

/// Build an object from input text.
fn parse_object(input_buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    input_buffer.depth += 1;

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b'{' {
        return None;
    }

    input_buffer.offset += 1;
    buffer_skip_whitespace(input_buffer);
    if input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b'}' {
        input_buffer.depth -= 1;
        input_buffer.offset += 1;
        return Some(Value::Object(Vec::new()));
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
        buffer_skip_whitespace(input_buffer);
        let key = parse_string(input_buffer)?;
        buffer_skip_whitespace(input_buffer);

        if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b':' {
            return None;
        }

        input_buffer.offset += 1;
        buffer_skip_whitespace(input_buffer);
        let val = parse_value(input_buffer)?;
        members.push((key, val));
        buffer_skip_whitespace(input_buffer);

        if !(input_buffer.can_access_at_index(0) && input_buffer.byte_at(0) == b',') {
            break;
        }
    }

    if input_buffer.cannot_access_at_index(0) || input_buffer.byte_at(0) != b'}' {
        return None;
    }

    input_buffer.depth -= 1;
    input_buffer.offset += 1;
    Some(Value::Object(members))
}

/// Parser core - when encountering text, process appropriately.
fn parse_value(input_buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if input_buffer.can_read(4) && input_buffer.slice_from_offset().starts_with(b"null") {
        input_buffer.offset += 4;
        return Some(Value::Null);
    }
    if input_buffer.can_read(5) && input_buffer.slice_from_offset().starts_with(b"false") {
        input_buffer.offset += 5;
        return Some(Value::False);
    }
    if input_buffer.can_read(4) && input_buffer.slice_from_offset().starts_with(b"true") {
        input_buffer.offset += 4;
        return Some(Value::True);
    }
    if input_buffer.can_access_at_index(0) {
        let b = input_buffer.byte_at(0);
        if b == b'\"' {
            return parse_string(input_buffer).map(Value::String);
        }
        if b == b'-' || b.is_ascii_digit() {
            return parse_number(input_buffer);
        }
        if b == b'[' {
            return parse_array(input_buffer);
        }
        if b == b'{' {
            return parse_object(input_buffer);
        }
    }

    None
}

/// Parse a JSON byte slice with `cJSON_ParseWithLengthOpts(input + "\0", len + 1, &end, 1)` semantics.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut content = Vec::with_capacity(input.len() + 1);
    content.extend_from_slice(input);
    content.push(0);

    let buffer_length = content.len();
    let mut buffer = ParseBuffer {
        content: &content,
        length: buffer_length,
        offset: 0,
        depth: 0,
    };

    skip_utf8_bom(&mut buffer);
    buffer_skip_whitespace(&mut buffer);

    if let Some(item) = parse_value(&mut buffer) {
        buffer_skip_whitespace(&mut buffer);
        if buffer.offset < buffer.length && buffer.byte_at(0) == b'\0' {
            return Ok(item);
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

#[inline]
fn double_to_valueint(number: f64) -> i32 {
    if number >= (i32::MAX as f64) {
        i32::MAX
    } else if number <= (i32::MIN as f64) {
        i32::MIN
    } else {
        number as i32
    }
}

#[inline]
fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}

/// Emulate `sprintf(buf, "%1.*g", precision, d)` for a finite non-zero `f64`.
fn format_g(d: f64, precision: usize) -> String {
    let exp_formatted = format!("{:.*e}", precision - 1, d.abs());
    let (mantissa, exp_str) = match exp_formatted.split_once('e') {
        Some(parts) => parts,
        None => return exp_formatted,
    };
    let exp: i32 = exp_str.parse::<i32>().unwrap_or(0);

    let mut digits = Vec::with_capacity(precision);
    for b in mantissa.bytes() {
        if b != b'.' {
            digits.push(b);
        }
    }

    while digits.len() > 1 && digits.last() == Some(&b'0') {
        digits.pop();
    }

    let mut out = Vec::with_capacity(26);
    if d.is_sign_negative() {
        out.push(b'-');
    }

    if exp < -4 || exp >= (precision as i32) {
        out.push(digits[0]);
        if digits.len() > 1 {
            out.push(b'.');
            out.extend_from_slice(&digits[1..]);
        }
        out.push(b'e');
        if exp >= 0 {
            out.push(b'+');
        } else {
            out.push(b'-');
        }
        let abs_exp = exp.unsigned_abs();
        let exp_digits = format!("{:02}", abs_exp);
        out.extend_from_slice(exp_digits.as_bytes());
    } else if exp < 0 {
        out.extend_from_slice(b"0.");
        let zeros = (-exp - 1) as usize;
        out.resize(out.len() + zeros, b'0');
        out.extend_from_slice(&digits);
    } else {
        let int_digits = (exp + 1) as usize;
        if digits.len() <= int_digits {
            out.extend_from_slice(&digits);
            let trailing_zeros = int_digits - digits.len();
            out.resize(out.len() + trailing_zeros, b'0');
        } else {
            out.extend_from_slice(&digits[..int_digits]);
            out.push(b'.');
            out.extend_from_slice(&digits[int_digits..]);
        }
    }

    String::from_utf8(out).unwrap_or_default()
}

/// Render the number from the given item into the output buffer.
fn print_number(d: f64, output_buffer: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        output_buffer.extend_from_slice(b"null");
        return;
    }

    let valueint = double_to_valueint(d);
    if d == (valueint as f64) {
        output_buffer.extend_from_slice(valueint.to_string().as_bytes());
        return;
    }

    let s15 = format_g(d, 15);
    if let Ok(test) = s15.parse::<f64>() {
        if compare_double(test, d) {
            output_buffer.extend_from_slice(s15.as_bytes());
            return;
        }
    }

    let s17 = format_g(d, 17);
    output_buffer.extend_from_slice(s17.as_bytes());
}

/// Render a byte string (stopping at first NUL byte, like C string) to an escaped JSON string.
fn print_string_ptr(input: &[u8], output_buffer: &mut Vec<u8>) {
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());
    let c_slice = &input[..end];

    let mut escape_characters: usize = 0;
    for &b in c_slice {
        match b {
            b'\"' | b'\\' | 0x08 | 0x0C | b'\n' | b'\r' | b'\t' => {
                escape_characters += 1;
            }
            _ if b < 32 => {
                escape_characters += 5;
            }
            _ => {}
        }
    }

    output_buffer.reserve(c_slice.len() + escape_characters + 2);
    output_buffer.push(b'\"');

    if escape_characters == 0 {
        output_buffer.extend_from_slice(c_slice);
        output_buffer.push(b'\"');
        return;
    }

    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    for &b in c_slice {
        if b > 31 && b != b'\"' && b != b'\\' {
            output_buffer.push(b);
        } else {
            output_buffer.push(b'\\');
            match b {
                b'\\' => output_buffer.push(b'\\'),
                b'\"' => output_buffer.push(b'\"'),
                0x08 => output_buffer.push(b'b'),
                0x0C => output_buffer.push(b'f'),
                b'\n' => output_buffer.push(b'n'),
                b'\r' => output_buffer.push(b'r'),
                b'\t' => output_buffer.push(b't'),
                _ => {
                    output_buffer.push(b'u');
                    output_buffer.push(b'0');
                    output_buffer.push(b'0');
                    output_buffer.push(HEX_DIGITS[((b >> 4) & 0x0F) as usize]);
                    output_buffer.push(HEX_DIGITS[(b & 0x0F) as usize]);
                }
            }
        }
    }

    output_buffer.push(b'\"');
}

fn print_array(elements: &[Value], output_buffer: &mut Vec<u8>) {
    output_buffer.push(b'[');
    for (i, elem) in elements.iter().enumerate() {
        if i > 0 {
            output_buffer.push(b',');
        }
        print_value(elem, output_buffer);
    }
    output_buffer.push(b']');
}

fn print_object(members: &[(Vec<u8>, Value)], output_buffer: &mut Vec<u8>) {
    output_buffer.push(b'{');
    for (i, (key, val)) in members.iter().enumerate() {
        if i > 0 {
            output_buffer.push(b',');
        }
        print_string_ptr(key, output_buffer);
        output_buffer.push(b':');
        print_value(val, output_buffer);
    }
    output_buffer.push(b'}');
}

fn print_value(item: &Value, output_buffer: &mut Vec<u8>) {
    match item {
        Value::Null => output_buffer.extend_from_slice(b"null"),
        Value::False => output_buffer.extend_from_slice(b"false"),
        Value::True => output_buffer.extend_from_slice(b"true"),
        Value::Number(d) => print_number(*d, output_buffer),
        Value::Raw(raw) => {
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            output_buffer.extend_from_slice(&raw[..end]);
        }
        Value::String(s) => print_string_ptr(s, output_buffer),
        Value::Array(arr) => print_array(arr, output_buffer),
        Value::Object(obj) => print_object(obj, output_buffer),
    }
}

/// Render a `Value` to unformatted JSON bytes, matching `cJSON_PrintUnformatted`.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut output_buffer = Vec::with_capacity(256);
    print_value(value, &mut output_buffer);
    output_buffer
}
