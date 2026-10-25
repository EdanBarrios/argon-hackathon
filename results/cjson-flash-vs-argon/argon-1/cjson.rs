//! cJSON v1.7.19 parse + print translated to safe Rust.

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
        self.offset
            .checked_add(size)
            .is_some_and(|end| end <= self.length)
    }

    #[inline]
    fn can_access_at_index(&self, index: usize) -> bool {
        self.offset
            .checked_add(index)
            .is_some_and(|pos| pos < self.length)
    }

    #[inline]
    fn cannot_access_at_index(&self, index: usize) -> bool {
        !self.can_access_at_index(index)
    }

    #[inline]
    fn byte_at(&self, index: usize) -> u8 {
        self.offset
            .checked_add(index)
            .and_then(|pos| self.content.get(pos).copied())
            .unwrap_or(0)
    }

    #[inline]
    fn slice_from_offset(&self) -> &'a [u8] {
        if self.offset <= self.content.len() {
            &self.content[self.offset..]
        } else {
            &[]
        }
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

/// Parse `input` with the exact semantics of
/// `cJSON_ParseWithLengthOpts(input + "\0", input.len() + 1, &end, 1)`.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut owned = Vec::with_capacity(input.len().saturating_add(1));
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
    let position = if buffer.offset < buffer.length {
        buffer.offset
    } else if buffer.length > 0 {
        buffer.length - 1
    } else {
        0
    };
    ParseError {
        offset: Some(position),
    }
}

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

#[inline(never)]
fn parse_number(input_buffer: &mut ParseBuffer<'_>) -> Option<f64> {
    let mut number_string_length = 0usize;
    while input_buffer.can_access_at_index(number_string_length) {
        match input_buffer.byte_at(number_string_length) {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
    }

    let num_slice = &input_buffer.slice_from_offset()[..number_string_length];
    let consumed = strtod_prefix_len(num_slice);
    if consumed == 0 {
        return None;
    }

    let valid_str = std::str::from_utf8(&num_slice[..consumed]).ok()?;
    let number = valid_str.parse::<f64>().ok()?;

    input_buffer.offset += consumed;
    Some(number)
}

/// Computes the exact number of bytes that C's `strtod` consumes from a slice
/// containing only bytes in `[0-9+\-eE.]`.
fn strtod_prefix_len(bytes: &[u8]) -> usize {
    let mut idx = 0usize;

    if idx < bytes.len() && (bytes[idx] == b'-' || bytes[idx] == b'+') {
        idx += 1;
    }

    let mut int_digits = 0usize;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
        int_digits += 1;
    }

    let mut frac_digits = 0usize;
    if idx < bytes.len() && bytes[idx] == b'.' {
        idx += 1;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            idx += 1;
            frac_digits += 1;
        }
    }

    if int_digits == 0 && frac_digits == 0 {
        return 0;
    }

    if idx < bytes.len() && (bytes[idx] == b'e' || bytes[idx] == b'E') {
        let exp_start = idx;
        idx += 1;
        if idx < bytes.len() && (bytes[idx] == b'+' || bytes[idx] == b'-') {
            idx += 1;
        }
        let mut exp_digits = 0usize;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            idx += 1;
            exp_digits += 1;
        }
        if exp_digits == 0 {
            idx = exp_start;
        }
    }

    idx
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h = 0u32;
    for i in 0..4 {
        let b = match input.get(i) {
            Some(&b) => b,
            None => return 0,
        };
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

fn utf16_literal_to_utf8(input: &[u8], output: &mut Vec<u8>) -> usize {
    if input.len() < 6 {
        return 0;
    }

    let first_code = parse_hex4(&input[2..6]);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let (sequence_length, mut codepoint) = if (0xD800..=0xDBFF).contains(&first_code) {
        if input.len() < 12 {
            return 0;
        }
        if input[6] != b'\\' || input[7] != b'u' {
            return 0;
        }
        let second_code = parse_hex4(&input[8..12]);
        if !(0xDC00..=0xDFFF).contains(&second_code) {
            return 0;
        }
        let cp = 0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF));
        (12usize, cp)
    } else {
        (6usize, first_code)
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
    for pos in (1..utf8_length).rev() {
        encoded[pos] = ((codepoint as u8) | 0x80) & 0xBF;
        codepoint >>= 6;
    }
    if utf8_length > 1 {
        encoded[0] = (codepoint as u8) | first_byte_mark;
    } else {
        encoded[0] = (codepoint as u8) & 0x7F;
    }

    output.extend_from_slice(&encoded[..utf8_length]);
    sequence_length
}

#[inline(never)]
fn parse_string(input_buffer: &mut ParseBuffer<'_>) -> Option<Vec<u8>> {
    let mut input_pointer = input_buffer.offset.saturating_add(1);
    let mut input_end = input_buffer.offset.saturating_add(1);

    if input_buffer.byte_at(0) != b'"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let mut skipped_bytes = 0usize;
    while input_end < input_buffer.length && input_buffer.content[input_end] != b'"' {
        if input_buffer.content[input_end] == b'\\' {
            if input_end.saturating_add(1) >= input_buffer.length {
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
            if input_end - input_pointer < 1 {
                input_buffer.offset = input_pointer;
                return None;
            }
            let esc = match input_buffer.content.get(input_pointer + 1) {
                Some(&c) => c,
                None => {
                    input_buffer.offset = input_pointer;
                    return None;
                }
            };
            let mut sequence_length = 2usize;
            match esc {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'"' | b'\\' | b'/' => output.push(esc),
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
        let elem = parse_value(input_buffer)?;
        elements.push(elem);
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

/// Render `value` to its unformatted JSON byte representation, matching `cJSON_PrintUnformatted`.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut output = Vec::with_capacity(256);
    print_value(value, &mut output);
    output
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
        Value::Array(items) => {
            output_buffer.push(b'[');
            for (i, elem) in items.iter().enumerate() {
                if i > 0 {
                    output_buffer.push(b',');
                }
                print_value(elem, output_buffer);
            }
            output_buffer.push(b']');
        }
        Value::Object(entries) => {
            output_buffer.push(b'{');
            for (i, (key, val)) in entries.iter().enumerate() {
                if i > 0 {
                    output_buffer.push(b',');
                }
                print_string_ptr(key, output_buffer);
                output_buffer.push(b':');
                print_value(val, output_buffer);
            }
            output_buffer.push(b'}');
        }
    }
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

#[inline(never)]
fn print_number(d: f64, output_buffer: &mut Vec<u8>) {
    if d.is_nan() || d.is_infinite() {
        output_buffer.extend_from_slice(b"null");
        return;
    }

    let valueint = double_to_valueint(d);
    if d == f64::from(valueint) {
        output_buffer.extend_from_slice(valueint.to_string().as_bytes());
        return;
    }

    let mut number_buffer = Vec::with_capacity(26);
    format_printf_g(d, 15, &mut number_buffer);

    let round_trip_ok = std::str::from_utf8(&number_buffer)
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .is_some_and(|test| compare_double(test, d));

    if !round_trip_ok {
        number_buffer.clear();
        format_printf_g(d, 17, &mut number_buffer);
    }

    output_buffer.extend_from_slice(&number_buffer);
}

/// Formats `d` according to C's `sprintf("%1.<precision>g", d)`.
fn format_printf_g(d: f64, precision: usize, out: &mut Vec<u8>) {
    if d == 0.0 {
        if d.is_sign_negative() {
            out.push(b'-');
        }
        out.push(b'0');
        return;
    }

    let exp_prec = precision.saturating_sub(1);
    let formatted = format!("{:.*e}", exp_prec, d);

    let (negative, rest) = match formatted.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, formatted.as_str()),
    };

    let (mantissa, exp_str) = match rest.split_once('e') {
        Some(pair) => pair,
        None => return,
    };

    let exp: i32 = exp_str.parse().unwrap_or(0);

    let mut digits = Vec::with_capacity(precision);
    for &b in mantissa.as_bytes() {
        if b != b'.' {
            digits.push(b);
        }
    }
    while digits.len() > 1 && digits.last() == Some(&b'0') {
        digits.pop();
    }

    if negative {
        out.push(b'-');
    }

    let p = precision as i32;
    if exp >= -4 && exp < p {
        if exp < 0 {
            out.extend_from_slice(b"0.");
            for _ in 0..(-1 - exp) {
                out.push(b'0');
            }
            out.extend_from_slice(&digits);
        } else {
            let int_len = (exp + 1) as usize;
            if digits.len() <= int_len {
                out.extend_from_slice(&digits);
                for _ in 0..(int_len - digits.len()) {
                    out.push(b'0');
                }
            } else {
                out.extend_from_slice(&digits[..int_len]);
                out.push(b'.');
                out.extend_from_slice(&digits[int_len..]);
            }
        }
    } else {
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
        if abs_exp < 10 {
            out.push(b'0');
            out.push(b'0' + (abs_exp as u8));
        } else if abs_exp < 100 {
            out.push(b'0' + ((abs_exp / 10) as u8));
            out.push(b'0' + ((abs_exp % 10) as u8));
        } else {
            out.extend_from_slice(abs_exp.to_string().as_bytes());
        }
    }
}

#[inline(never)]
fn print_string_ptr(input: &[u8], output_buffer: &mut Vec<u8>) {
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

    output_buffer.reserve(slice.len() + escape_characters + 2);
    output_buffer.push(b'"');

    if escape_characters == 0 {
        output_buffer.extend_from_slice(slice);
        output_buffer.push(b'"');
        return;
    }

    const HEX: &[u8; 16] = b"0123456789abcdef";
    for &b in slice {
        if b > 31 && b != b'"' && b != b'\\' {
            output_buffer.push(b);
        } else {
            output_buffer.push(b'\\');
            match b {
                b'\\' => output_buffer.push(b'\\'),
                b'"' => output_buffer.push(b'"'),
                0x08 => output_buffer.push(b'b'),
                0x0C => output_buffer.push(b'f'),
                b'\n' => output_buffer.push(b'n'),
                b'\r' => output_buffer.push(b'r'),
                b'\t' => output_buffer.push(b't'),
                _ => {
                    output_buffer.push(b'u');
                    output_buffer.push(b'0');
                    output_buffer.push(b'0');
                    output_buffer.push(HEX[usize::from(b >> 4)]);
                    output_buffer.push(HEX[usize::from(b & 0x0F)]);
                }
            }
        }
    }

    output_buffer.push(b'"');
}
