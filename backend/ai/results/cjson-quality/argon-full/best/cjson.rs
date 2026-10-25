//! cJSON v1.7.19 parse + print translated to Rust.

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
    fn can_read(&self, size: usize) -> bool {
        self.offset.saturating_add(size) <= self.length
    }

    fn can_access_at_index(&self, index: usize) -> bool {
        self.offset.saturating_add(index) < self.length
    }

    fn cannot_access_at_index(&self, index: usize) -> bool {
        !self.can_access_at_index(index)
    }

    fn byte_at(&self, index: usize) -> Option<u8> {
        let pos = self.offset.checked_add(index)?;
        if pos < self.length {
            self.content.get(pos).copied()
        } else {
            None
        }
    }

    fn slice_from_offset(&self, len: usize) -> Option<&'a [u8]> {
        let end = self.offset.checked_add(len)?;
        if end <= self.length {
            self.content.get(self.offset..end)
        } else {
            None
        }
    }

    fn skip_utf8_bom(&mut self) {
        if self.offset != 0 {
            return;
        }
        if self.can_access_at_index(4) && self.slice_from_offset(3) == Some(b"\xEF\xBB\xBF") {
            self.offset += 3;
        }
    }

    fn skip_whitespace(&mut self) {
        if self.cannot_access_at_index(0) {
            return;
        }
        while let Some(b) = self.byte_at(0) {
            if b <= 32 {
                self.offset += 1;
            } else {
                break;
            }
        }
        if self.offset == self.length {
            self.offset = self.offset.saturating_sub(1);
        }
    }
}

fn fail_error(buffer: &ParseBuffer<'_>) -> ParseError {
    let position = if buffer.offset < buffer.length {
        buffer.offset
    } else {
        buffer.length.saturating_sub(1)
    };
    ParseError {
        offset: Some(position),
    }
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h = 0u32;
    for i in 0..4usize {
        let b = match input.get(i).copied() {
            Some(v) => v,
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

fn utf16_literal_to_utf8(first_sequence: &[u8], output: &mut Vec<u8>) -> usize {
    if first_sequence.len() < 6 {
        return 0;
    }

    let first_hex = match first_sequence.get(2..6) {
        Some(s) => s,
        None => return 0,
    };
    let first_code = parse_hex4(first_hex);

    if (0xDC00..=0xDFFF).contains(&first_code) {
        return 0;
    }

    let (sequence_length, mut codepoint) = if (0xD800..=0xDBFF).contains(&first_code) {
        let second_sequence = match first_sequence.get(6..) {
            Some(s) => s,
            None => return 0,
        };
        if second_sequence.len() < 6 {
            return 0;
        }
        if second_sequence.first().copied() != Some(b'\\')
            || second_sequence.get(1).copied() != Some(b'u')
        {
            return 0;
        }
        let second_hex = match second_sequence.get(2..6) {
            Some(s) => s,
            None => return 0,
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
    let mut utf8_position = utf8_length.saturating_sub(1);
    while utf8_position > 0 {
        if let Some(slot) = encoded.get_mut(utf8_position) {
            *slot = ((codepoint | 0x80) & 0xBF) as u8;
        }
        codepoint >>= 6;
        utf8_position -= 1;
    }

    if let Some(slot0) = encoded.get_mut(0) {
        if utf8_length > 1 {
            *slot0 = ((codepoint | first_byte_mark) & 0xFF) as u8;
        } else {
            *slot0 = (codepoint & 0x7F) as u8;
        }
    }

    if let Some(bytes) = encoded.get(..utf8_length) {
        output.extend_from_slice(bytes);
    }

    sequence_length
}

fn parse_string(buffer: &mut ParseBuffer<'_>) -> Option<Vec<u8>> {
    let mut input_pointer = buffer.offset.saturating_add(1);
    let mut input_end = buffer.offset.saturating_add(1);

    if buffer.byte_at(0) != Some(b'"') {
        buffer.offset = input_pointer;
        return None;
    }

    let mut skipped_bytes = 0usize;
    while input_end < buffer.length && buffer.content.get(input_end).copied() != Some(b'"') {
        if buffer.content.get(input_end).copied() == Some(b'\\') {
            if input_end.saturating_add(1) >= buffer.length {
                buffer.offset = input_pointer;
                return None;
            }
            skipped_bytes = skipped_bytes.saturating_add(1);
            input_end = input_end.saturating_add(1);
        }
        input_end = input_end.saturating_add(1);
    }
    if input_end >= buffer.length || buffer.content.get(input_end).copied() != Some(b'"') {
        buffer.offset = input_pointer;
        return None;
    }

    let alloc_len = input_end
        .saturating_sub(buffer.offset)
        .saturating_sub(skipped_bytes);
    let mut output = Vec::with_capacity(alloc_len);

    while input_pointer < input_end {
        let ch = match buffer.content.get(input_pointer).copied() {
            Some(c) => c,
            None => {
                buffer.offset = input_pointer;
                return None;
            }
        };
        if ch != b'\\' {
            output.push(ch);
            input_pointer = input_pointer.saturating_add(1);
        } else {
            if input_end.saturating_sub(input_pointer) < 1 {
                buffer.offset = input_pointer;
                return None;
            }
            let esc = match buffer.content.get(input_pointer.saturating_add(1)).copied() {
                Some(c) => c,
                None => {
                    buffer.offset = input_pointer;
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
                    let slice = match buffer.content.get(input_pointer..input_end) {
                        Some(s) => s,
                        None => {
                            buffer.offset = input_pointer;
                            return None;
                        }
                    };
                    sequence_length = utf16_literal_to_utf8(slice, &mut output);
                    if sequence_length == 0 {
                        buffer.offset = input_pointer;
                        return None;
                    }
                }
                _ => {
                    buffer.offset = input_pointer;
                    return None;
                }
            }
            input_pointer = input_pointer.saturating_add(sequence_length);
        }
    }

    buffer.offset = input_end.saturating_add(1);
    Some(output)
}

fn strtod_prefix_len(bytes: &[u8]) -> usize {
    let mut i = 0usize;
    if matches!(bytes.get(i), Some(b'+' | b'-')) {
        i += 1;
    }
    let mut digits = 0usize;
    while matches!(bytes.get(i), Some(b'0'..=b'9')) {
        i += 1;
        digits += 1;
    }
    if matches!(bytes.get(i), Some(b'.')) {
        i += 1;
        while matches!(bytes.get(i), Some(b'0'..=b'9')) {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0;
    }
    let after_sig = i;
    if matches!(bytes.get(i), Some(b'e' | b'E')) {
        i += 1;
        if matches!(bytes.get(i), Some(b'+' | b'-')) {
            i += 1;
        }
        let mut exp_digits = 0usize;
        while matches!(bytes.get(i), Some(b'0'..=b'9')) {
            i += 1;
            exp_digits += 1;
        }
        if exp_digits == 0 {
            return after_sig;
        }
    }
    i
}

fn parse_number(buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    let mut number_string_length = 0usize;
    while let Some(b) = buffer.byte_at(number_string_length) {
        match b {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                number_string_length += 1;
            }
            _ => break,
        }
    }

    let num_slice = buffer.slice_from_offset(number_string_length)?;
    let consumed = strtod_prefix_len(num_slice);
    if consumed == 0 {
        return None;
    }

    let valid_slice = num_slice.get(..consumed)?;
    let valid_str = std::str::from_utf8(valid_slice).ok()?;
    let number = valid_str.parse::<f64>().ok()?;

    buffer.offset = buffer.offset.saturating_add(consumed);
    Some(Value::Number(number))
}

fn parse_array(buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    buffer.depth += 1;

    if buffer.byte_at(0) != Some(b'[') {
        return None;
    }

    buffer.offset += 1;
    buffer.skip_whitespace();
    if buffer.byte_at(0) == Some(b']') {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Some(Value::Array(Vec::new()));
    }

    if buffer.cannot_access_at_index(0) {
        buffer.offset = buffer.offset.saturating_sub(1);
        return None;
    }

    buffer.offset = buffer.offset.saturating_sub(1);
    let mut elements = Vec::new();
    loop {
        buffer.offset += 1;
        buffer.skip_whitespace();
        let val = parse_value(buffer)?;
        elements.push(val);
        buffer.skip_whitespace();
        if buffer.byte_at(0) != Some(b',') {
            break;
        }
    }

    if buffer.byte_at(0) != Some(b']') {
        return None;
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Some(Value::Array(elements))
}

fn parse_object(buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if buffer.depth >= CJSON_NESTING_LIMIT {
        return None;
    }
    buffer.depth += 1;

    if buffer.byte_at(0) != Some(b'{') {
        return None;
    }

    buffer.offset += 1;
    buffer.skip_whitespace();
    if buffer.byte_at(0) == Some(b'}') {
        buffer.depth -= 1;
        buffer.offset += 1;
        return Some(Value::Object(Vec::new()));
    }

    if buffer.cannot_access_at_index(0) {
        buffer.offset = buffer.offset.saturating_sub(1);
        return None;
    }

    buffer.offset = buffer.offset.saturating_sub(1);
    let mut members = Vec::new();
    loop {
        if buffer.cannot_access_at_index(1) {
            return None;
        }

        buffer.offset += 1;
        buffer.skip_whitespace();
        let key = parse_string(buffer)?;
        buffer.skip_whitespace();

        if buffer.byte_at(0) != Some(b':') {
            return None;
        }

        buffer.offset += 1;
        buffer.skip_whitespace();
        let val = parse_value(buffer)?;
        members.push((key, val));
        buffer.skip_whitespace();

        if buffer.byte_at(0) != Some(b',') {
            break;
        }
    }

    if buffer.byte_at(0) != Some(b'}') {
        return None;
    }

    buffer.depth -= 1;
    buffer.offset += 1;
    Some(Value::Object(members))
}

fn parse_value(buffer: &mut ParseBuffer<'_>) -> Option<Value> {
    if buffer.can_read(4) && buffer.slice_from_offset(4) == Some(b"null") {
        buffer.offset += 4;
        return Some(Value::Null);
    }
    if buffer.can_read(5) && buffer.slice_from_offset(5) == Some(b"false") {
        buffer.offset += 5;
        return Some(Value::False);
    }
    if buffer.can_read(4) && buffer.slice_from_offset(4) == Some(b"true") {
        buffer.offset += 4;
        return Some(Value::True);
    }
    if let Some(first) = buffer.byte_at(0) {
        if first == b'"' {
            return parse_string(buffer).map(Value::String);
        }
        if first == b'-' || first.is_ascii_digit() {
            return parse_number(buffer);
        }
        if first == b'[' {
            return parse_array(buffer);
        }
        if first == b'{' {
            return parse_object(buffer);
        }
    }
    None
}

/// Parses a JSON byte slice using cJSON's `cJSON_ParseWithLengthOpts(input + "\0", len + 1, &end, 1)` semantics.
pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut content = Vec::with_capacity(input.len().saturating_add(1));
    content.extend_from_slice(input);
    content.push(0);

    let mut buffer = ParseBuffer {
        content: &content,
        length: content.len(),
        offset: 0,
        depth: 0,
    };

    buffer.skip_utf8_bom();
    buffer.skip_whitespace();

    let value = match parse_value(&mut buffer) {
        Some(v) => v,
        None => return Err(fail_error(&buffer)),
    };

    buffer.skip_whitespace();
    if buffer.offset >= buffer.length || buffer.byte_at(0) != Some(0) {
        return Err(fail_error(&buffer));
    }

    Ok(value)
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    (a - b).abs() <= max_val * f64::EPSILON
}

fn format_g(d: f64, precision: usize) -> Vec<u8> {
    let negative = d.is_sign_negative();
    let abs_d = d.abs();
    let formatted = format!("{:.*e}", precision.saturating_sub(1), abs_d);
    let (mantissa_str, exp_str) = match formatted.split_once('e') {
        Some(pair) => pair,
        None => return formatted.into_bytes(),
    };
    let exp = exp_str.parse::<i32>().unwrap_or(0);

    let mut digits: Vec<u8> = mantissa_str.bytes().filter(|&b| b != b'.').collect();
    while digits.len() > 1 && digits.last() == Some(&b'0') {
        digits.pop();
    }

    let mut out = Vec::with_capacity(26);
    if negative {
        out.push(b'-');
    }

    let p_i32 = precision as i32;
    if exp < -4 || exp >= p_i32 {
        if let Some((&first, rest)) = digits.split_first() {
            out.push(first);
            if !rest.is_empty() {
                out.push(b'.');
                out.extend_from_slice(rest);
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
        }
        out.extend_from_slice(abs_exp.to_string().as_bytes());
    } else if exp < 0 {
        out.extend_from_slice(b"0.");
        let zeros = (-exp - 1) as usize;
        out.extend(std::iter::repeat_n(b'0', zeros));
        out.extend_from_slice(&digits);
    } else {
        let int_digits = (exp + 1) as usize;
        if digits.len() <= int_digits {
            out.extend_from_slice(&digits);
            let trailing_zeros = int_digits.saturating_sub(digits.len());
            out.extend(std::iter::repeat_n(b'0', trailing_zeros));
        } else if let (Some(int_part), Some(frac_part)) =
            (digits.get(..int_digits), digits.get(int_digits..))
        {
            out.extend_from_slice(int_part);
            out.push(b'.');
            out.extend_from_slice(frac_part);
        }
    }

    out
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
    } else {
        let buf15 = format_g(d, 15);
        let round_trip_ok = std::str::from_utf8(&buf15)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .is_some_and(|test| compare_double(test, d));
        if round_trip_ok {
            out.extend_from_slice(&buf15);
        } else {
            let buf17 = format_g(d, 17);
            out.extend_from_slice(&buf17);
        }
    }
}

fn print_string_ptr(input: &[u8], out: &mut Vec<u8>) {
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());
    let slice = input.get(..end).unwrap_or(&[]);

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

fn print_value(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::False => out.extend_from_slice(b"false"),
        Value::True => out.extend_from_slice(b"true"),
        Value::Number(d) => print_number(*d, out),
        Value::Raw(bytes) => {
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            if let Some(slice) = bytes.get(..end) {
                out.extend_from_slice(slice);
            }
        }
        Value::String(bytes) => print_string_ptr(bytes, out),
        Value::Array(items) => {
            out.push(b'[');
            let len = items.len();
            for (idx, item) in items.iter().enumerate() {
                print_value(item, out);
                if idx + 1 < len {
                    out.push(b',');
                }
            }
            out.push(b']');
        }
        Value::Object(members) => {
            out.push(b'{');
            let len = members.len();
            for (idx, (key, val)) in members.iter().enumerate() {
                print_string_ptr(key, out);
                out.push(b':');
                print_value(val, out);
                if idx + 1 < len {
                    out.push(b',');
                }
            }
            out.push(b'}');
        }
    }
}

/// Renders a `Value` to unformatted JSON bytes, matching `cJSON_PrintUnformatted`.
pub fn print_unformatted(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    print_value(value, &mut out);
    out
}
