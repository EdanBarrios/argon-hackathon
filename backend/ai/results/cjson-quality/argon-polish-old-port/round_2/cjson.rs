//! cJSON v1.7.19 parse + print, translated to safe Rust.

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
        self.content
            .get(self.offset + index)
            .copied()
            .unwrap_or(0)
    }

    #[inline]
    fn slice_from_offset(&self) -> &[u8] {
        self.content.get(self.offset..self.length).unwrap_or(&[])
    }
}

/// Skip the UTF-8 BOM (byte order mark) if it is at the beginning of a buffer.
fn skip_utf8_bom(buffer: &mut ParseBuffer) {
    if buffer.offset != 0 {
        return;
    }
    if buffer.can_access_at_index(4) && buffer.slice_from_offset().starts_with(b"\xEF\xBB\xBF") {
        buffer.offset += 3;
    }
}

/// Utility to jump whitespace and cr/lf.
fn buffer_skip_whitespace(buffer: &mut ParseBuffer) {
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

/// Find the length of the longest leading prefix of `s` (which contains only
/// bytes from `0..=9`, `+`, `-`, `e`, `E`, `.`) that C's `strtod` consumes.
fn strtod_prefix_len(s: &[u8]) -> usize {
    let mut i = 0;
    if matches!(s.get(i), Some(b'+' | b'-')) {
        i += 1;
    }

    let mut digits = 0usize;
    while matches!(s.get(i), Some(b'0'..=b'9')) {
        digits += 1;
        i += 1;
    }

    if matches!(s.get(i), Some(b'.')) {
        i += 1;
        while matches!(s.get(i), Some(b'0'..=b'9')) {
            digits += 1;
            i += 1;
        }
    }

    if digits == 0 {
        return 0;
    }

    let after_significand = i;
    if matches!(s.get(i), Some(b'e' | b'E')) {
        i += 1;
        if matches!(s.get(i), Some(b'+' | b'-')) {
            i += 1;
        }
        let mut exp_digits = 0usize;
        while matches!(s.get(i), Some(b'0'..=b'9')) {
            exp_digits += 1;
            i += 1;
        }
        if exp_digits == 0 {
            return after_significand;
        }
    }

    i
}

/// Parse the input text to generate a number.
fn parse_number(input_buffer: &mut ParseBuffer) -> Option<Value> {
    let mut number_string_length = 0usize;

    while input_buffer.can_access_at_index(number_string_length) {
        match input_buffer.byte_at(number_string_length) {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
    }

    let start = input_buffer.offset;
    let candidate = input_buffer
        .content
        .get(start..start + number_string_length)?;
    let consumed = strtod_prefix_len(candidate);
    if consumed == 0 {
        return None;
    }

    let prefix_bytes = candidate.get(..consumed)?;
    let prefix_str = std::str::from_utf8(prefix_bytes).ok()?;
    let number: f64 = prefix_str.parse::<f64>().ok()?;

    input_buffer.offset += consumed;
    Some(Value::Number(number))
}

/// Parse 4 digit hexadecimal number. Mirrors cJSON's `parse_hex4` (returns 0 on invalid hex).
fn parse_hex4(input: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for (i, &b) in input.iter().enumerate().take(4) {
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

/// Converts a UTF-16 literal to UTF-8. A literal can be one or two sequences of the form `\uXXXX`.
/// Returns the number of input bytes consumed (6 or 12), or 0 on failure.
fn utf16_literal_to_utf8(input: &[u8], output: &mut Vec<u8>) -> usize {
    let Some(first_hex) = input.get(2..6) else {
        return 0;
    };

    let first_code = parse_hex4(first_hex);
    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let (sequence_length, codepoint) = if (0xD800..=0xDBFF).contains(&first_code) {
        if input.len() < 12 {
            return 0;
        }
        if input.get(6) != Some(&b'\\') || input.get(7) != Some(&b'u') {
            return 0;
        }
        let Some(second_hex) = input.get(8..12) else {
            return 0;
        };
        let second_code = parse_hex4(second_hex);
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
    let mut cp = codepoint;
    for pos in (1..utf8_length).rev() {
        if let Some(slot) = encoded.get_mut(pos) {
            *slot = ((cp | 0x80) & 0xBF) as u8;
        }
        cp >>= 6;
    }
    if utf8_length > 1 {
        encoded[0] = ((cp | (first_byte_mark as u32)) & 0xFF) as u8;
    } else {
        encoded[0] = (cp & 0x7F) as u8;
    }

    if let Some(slice) = encoded.get(..utf8_length) {
        output.extend_from_slice(slice);
    }
    sequence_length
}

/// Parse the input text into an unescaped byte vector.
fn parse_string_raw(input_buffer: &mut ParseBuffer) -> Option<Vec<u8>> {
    let mut input_pointer = input_buffer.offset + 1;
    let mut input_end = input_buffer.offset + 1;

    if input_buffer.byte_at(0) != b'"' {
        input_buffer.offset = input_pointer;
        return None;
    }

    let mut skipped_bytes = 0usize;
    while input_end < input_buffer.length && input_buffer.content.get(input_end) != Some(&b'"') {
        if input_buffer.content.get(input_end) == Some(&b'\\') {
            if input_end + 1 >= input_buffer.length {
                input_buffer.offset = input_pointer;
                return None;
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }

    if input_end >= input_buffer.length || input_buffer.content.get(input_end) != Some(&b'"') {
        input_buffer.offset = input_pointer;
        return None;
    }

    let allocation_length = (input_end - input_buffer.offset).saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(allocation_length);

    while input_pointer < input_end {
        let b = input_buffer
            .content
            .get(input_pointer)
            .copied()
            .unwrap_or(0);
        if b != b'\\' {
            output.push(b);
            input_pointer += 1;
        } else {
            if input_end - input_pointer < 2 {
                input_buffer.offset = input_pointer;
                return None;
            }
            let esc = input_buffer
                .content
                .get(input_pointer + 1)
                .copied()
                .unwrap_or(0);
            let mut sequence_length = 2usize;
            match esc {
                b'b' => output.push(0x08),
                b'f' => output.push(0x0C),
                b'n' => output.push(b'\n'),
                b'r' => output.push(b'\r'),
                b't' => output.push(b'\t'),
                b'"' | b'\\' | b'/' => output.push(esc),
                b'u' => {
                    let rem = input_buffer
                        .content
                        .get(input_pointer..input_end)
                        .unwrap_or(&[]);
                    sequence_length = utf16_literal_to_utf8(rem, &mut output);
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

/// Build an array from input text.
fn parse_array(input_buffer: &mut ParseBuffer) -> Option<Value> {
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

/// Build an object from the text.
fn parse_object(input_buffer: &mut ParseBuffer) -> Option<Value> {
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
        let key = parse_string_raw(input_buffer)?;
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
fn parse_value(input_buffer: &mut ParseBuffer) -> Option<Value> {
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
        return parse_string_raw(input_buffer).map(Value::String);
    }
    if input_buffer.can_access_at_index(0) {
        let b = input_buffer.byte_at(0);
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

/// Parse `input` with the exact semantics of
/// `cJSON_ParseWithLengthOpts(input + "\0", input.len() + 1, &end, 1)`.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut content = Vec::with_capacity(input.len() + 1);
    content.extend_from_slice(input);
    content.push(0);

    let length = content.len();
    let mut buffer = ParseBuffer {
        content,
        length,
        offset: 0,
        depth: 0,
    };

    skip_utf8_bom(&mut buffer);
    buffer_skip_whitespace(&mut buffer);

    if let Some(value) = parse_value(&mut buffer) {
        buffer_skip_whitespace(&mut buffer);
        if buffer.offset < buffer.length && buffer.byte_at(0) == 0 {
            return Ok(value);
        }
    }

    let position = if buffer.offset < buffer.length {
        buffer.offset
    } else {
        buffer.length.saturating_sub(1)
    };

    Err(ParseError {
        offset: Some(position),
    })
}

#[inline]
fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}

/// Emulate `sprintf(buf, "%1.*g", precision, d)` for a finite non-zero `f64`.
fn format_g(d: f64, precision: usize) -> Vec<u8> {
    let sci = format!("{:.*e}", precision - 1, d);
    let sci_bytes = sci.as_bytes();

    let (negative, rest) = if let Some(stripped) = sci_bytes.strip_prefix(b"-") {
        (true, stripped)
    } else {
        (false, sci_bytes)
    };

    let e_pos = rest.iter().position(|&b| b == b'e').unwrap_or(rest.len());
    let mantissa = rest.get(..e_pos).unwrap_or(&[]);
    let exp_bytes = rest.get(e_pos + 1..).unwrap_or(b"0");
    let exp_str = std::str::from_utf8(exp_bytes).unwrap_or("0");
    let exp: i32 = exp_str.parse::<i32>().unwrap_or(0);

    let mut digits = Vec::with_capacity(precision);
    for &b in mantissa {
        if b != b'.' {
            digits.push(b);
        }
    }

    while digits.len() > 1 && digits.last() == Some(&b'0') {
        digits.pop();
    }
    let sig_len = digits.len();

    let mut out = Vec::with_capacity(26);
    if negative {
        out.push(b'-');
    }

    let p_i32 = precision as i32;
    if exp < -4 || exp >= p_i32 {
        if let Some(&first) = digits.first() {
            out.push(first);
        }
        if sig_len > 1 {
            out.push(b'.');
            if let Some(tail) = digits.get(1..sig_len) {
                out.extend_from_slice(tail);
            }
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
        } else {
            out.extend_from_slice(abs_exp.to_string().as_bytes());
        }
    } else if exp < 0 {
        out.extend_from_slice(b"0.");
        let zeros = (-exp - 1) as usize;
        out.resize(out.len() + zeros, b'0');
        if let Some(sig) = digits.get(..sig_len) {
            out.extend_from_slice(sig);
        }
    } else {
        let int_digits = (exp + 1) as usize;
        if sig_len <= int_digits {
            if let Some(sig) = digits.get(..sig_len) {
                out.extend_from_slice(sig);
            }
            out.resize(out.len() + (int_digits - sig_len), b'0');
        } else {
            if let Some(int_part) = digits.get(..int_digits) {
                out.extend_from_slice(int_part);
            }
            out.push(b'.');
            if let Some(frac_part) = digits.get(int_digits..sig_len) {
                out.extend_from_slice(frac_part);
            }
        }
    }

    out
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

    let buf15 = format_g(d, 15);
    let parsed = std::str::from_utf8(&buf15)
        .ok()
        .and_then(|s| s.parse::<f64>().ok());

    match parsed {
        Some(test) if compare_double(test, d) => {
            out.extend_from_slice(&buf15);
        }
        _ => {
            let buf17 = format_g(d, 17);
            out.extend_from_slice(&buf17);
        }
    }
}

fn print_string_ptr(input: &[u8], out: &mut Vec<u8>) {
    let nul_pos = input.iter().position(|&b| b == 0).unwrap_or(input.len());
    let slice = input.get(..nul_pos).unwrap_or(input);

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
                b'\r' => out.push(b'\r'),
                b'\t' => out.push(b'\t'),
                _ => {
                    const HEX: [u8; 16] = *b"0123456789abcdef";
                    out.push(b'u');
                    out.push(b'0');
                    out.push(b'0');
                    let hi = HEX.get(((b >> 4) & 0x0F) as usize).copied().unwrap_or(b'0');
                    let lo = HEX.get((b & 0x0F) as usize).copied().unwrap_or(b'0');
                    out.push(hi);
                    out.push(lo);
                }
            }
        }
    }
    out.push(b'"');
}

fn print_value_unformatted(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::False => out.extend_from_slice(b"false"),
        Value::True => out.extend_from_slice(b"true"),
        Value::Number(d) => print_number(*d, out),
        Value::Raw(raw) => {
            let nul_pos = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            if let Some(slice) = raw.get(..nul_pos) {
                out.extend_from_slice(slice);
            }
        }
        Value::String(s) => print_string_ptr(s, out),
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                print_value_unformatted(item, out);
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
                print_value_unformatted(v, out);
            }
            out.push(b'}');
        }
    }
}

/// Render `value` without formatting, matching `cJSON_PrintUnformatted`.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    print_value_unformatted(value, &mut out);
    out
}
