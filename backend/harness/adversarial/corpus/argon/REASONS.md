# Argon-written adversarial inputs

Model gemini-4-argon, thinking_level high, interaction v1_ChZCa3ZlYXRnNHpkVDZ3QS1wMVlENUFREhZCa3ZlYXRnNHpkVDZ3QS1wMVlENUFR, 682.3s wall. 225 returned, 225 unique, 0 with invalid hex dropped. Argon only generated these; the verifier judges them.

| file | bytes | preview | reason |
|---|---|---|---|
| argon_000.bin | 3 | `\xef\xbb\xbf` | skip_utf8_bom checks can_access_at_index(buffer, 4) which requires buffer_length >= 5, so a 3-byte BOM alone (buffer_length = 4 with appended NUL) is not skipped and fails at offset 0 instead of 3. |
| argon_001.bin | 4 | `\xef\xbb\xbf0` | skip_utf8_bom succeeds when 1 byte follows the 3-byte BOM because the driver's appended NUL makes buffer_length == 5 (can_access_at_index(buffer, 4) is true), parsing 0. |
| argon_002.bin | 4 | `\xef\xbb\xbfa` | skip_utf8_bom skips the 3-byte BOM when 1 invalid byte follows (buffer_length == 5), causing parse_value to fail at offset 3 instead of offset 0. |
| argon_003.bin | 4 | `\xef\xbb\xbf ` | skip_utf8_bom skips the BOM and buffer_skip_whitespace skips the space and trailing NUL before backing up one byte, failing at offset 4. |
| argon_004.bin | 3 | `\xef\xbb0` | skip_utf8_bom must not skip an incomplete 2-byte BOM prefix, causing parse_value to fail at offset 0. |
| argon_005.bin | 5 | ` \xef\xbb\xbf0` | cJSON_ParseWithLengthOpts calls skip_utf8_bom before buffer_skip_whitespace, so a leading space prevents BOM skipping and fails at offset 1. |
| argon_006.bin | 7 | `\xef\xbb\xbf\xef\xbb\xbf0` | skip_utf8_bom only skips a single BOM at offset 0, so a second BOM fails in parse_value at offset 3. |
| argon_007.bin | 8 | `\xef\xbb\xbf\x00true` | skip_utf8_bom skips the BOM and buffer_skip_whitespace skips the embedded NUL byte (0 <= 32) before parsing true. |
| argon_008.bin | 6 | `\xef\xbb\xbf"a"` | skip_utf8_bom skips the leading UTF-8 BOM before parse_string parses the root string literal. |
| argon_009.bin | 6 | `[\xef\xbb\xbf1]` | skip_utf8_bom is only called at offset 0 of the root buffer, so a BOM inside an array fails in parse_value at offset 1. |
| argon_010.bin | 0 | `` | buffer_skip_whitespace on an empty stdin skips the driver's appended NUL and decrements offset back to 0, failing at offset 0. |
| argon_011.bin | 4 | ` \t\n\r` | buffer_skip_whitespace on whitespace-only input consumes all whitespace plus the appended NUL and backs up to offset 4 (len), failing at offset 4 instead of 0. |
| argon_012.bin | 5 | `\x00true` | buffer_skip_whitespace treats NUL (0x00 <= 32) as whitespace, so a leading NUL byte before true is skipped and parsing succeeds. |
| argon_013.bin | 32 | `\x01\x02\x03\x04\x05\x06\x07\x08\x0b\x0c\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1ftrue` | buffer_skip_whitespace treats all ASCII control characters 0x01..0x1f (<= 32) as whitespace rather than only space, tab, CR, and LF. |
| argon_014.bin | 11 | `true\x01\x07\x08\x0b\x0c\x1b\x1f` | cJSON_ParseWithLengthOpts calls buffer_skip_whitespace for require_null_terminated, skipping trailing control bytes <= 32 up to the driver's appended NUL. |
| argon_015.bin | 7 | `true\x00  ` | buffer_skip_whitespace in the require_null_terminated check skips past an embedded NUL and trailing spaces to reach the final NUL, accepting the input. |
| argon_016.bin | 10 | `true\x00false` | buffer_skip_whitespace in require_null_terminated skips the NUL byte after true (since 0x00 <= 32) and fails at 'f' (offset 5) instead of accepting at the first NUL. |
| argon_017.bin | 9 | `[\x001\x01,\x0b2\x1f]` | parse_array uses buffer_skip_whitespace around elements and commas, allowing NUL and control characters <= 32 as array whitespace. |
| argon_018.bin | 11 | `{\x00"a"\x07:\x081\x0c}` | parse_object uses buffer_skip_whitespace around keys, colons, and values, allowing NUL and control characters <= 32 as object whitespace. |
| argon_019.bin | 5 | `\x7ftrue` | buffer_skip_whitespace does not skip DEL (0x7f > 32), failing at offset 0. |
| argon_020.bin | 5 | `true\x7f` | cJSON_ParseWithLengthOpts rejects trailing DEL (0x7f > 32) in the require_null_terminated check at offset 4. |
| argon_021.bin | 5 | `\x80true` | buffer_skip_whitespace compares unsigned char <= 32, so 0x80 is not treated as negative whitespace and fails at offset 0. |
| argon_022.bin | 5 | `\xfftrue` | buffer_skip_whitespace treats 0xff as unsigned 255 (> 32) rather than signed -1 (<= 32), failing at offset 0. |
| argon_023.bin | 5 | `true\x80` | buffer_skip_whitespace in require_null_terminated does not skip 0x80 as signed whitespace, failing at offset 4. |
| argon_024.bin | 6 | `\xc2\xa0true` | buffer_skip_whitespace only skips bytes <= 32 and does not treat UTF-8 non-breaking space (U+00A0) as whitespace, failing at offset 0. |
| argon_025.bin | 3 | `\x00\x00\x00` | buffer_skip_whitespace skips all three NUL bytes plus the driver's appended NUL and backs up one byte, failing at offset 3. |
| argon_026.bin | 6 | `\x00\x01\x1fxyz` | buffer_skip_whitespace skips leading NUL and control bytes so parse_value fails at 'x' (offset 3). |
| argon_027.bin | 8 | `true   @` | cJSON_ParseWithLengthOpts skips spaces after true and fails at '@' (offset 7) in the require_null_terminated check. |
| argon_028.bin | 4 | `[\x7f1]` | parse_array skips whitespace after '[' and fails in parse_value at 0x7f (offset 1). |
| argon_029.bin | 3 | `[\x00]` | parse_array skips an embedded NUL byte inside an empty array via buffer_skip_whitespace and outputs []. |
| argon_030.bin | 4 | `null` | parse_value parses null and print_value prints null. |
| argon_031.bin | 4 | `true` | parse_value parses true and print_value prints true. |
| argon_032.bin | 5 | `false` | parse_value parses false and print_value prints false. |
| argon_033.bin | 3 | `nul` | parse_value requires 4 bytes for null via can_read and leaves buffer.offset at 0 on truncated 'nul'. |
| argon_034.bin | 3 | `tru` | parse_value leaves buffer.offset at 0 on truncated 'tru'. |
| argon_035.bin | 4 | `fals` | parse_value requires 5 bytes for false and leaves buffer.offset at 0 on truncated 'fals'. |
| argon_036.bin | 4 | `Null` | parse_value uses case-sensitive strncmp for null, failing at offset 0 on 'Null'. |
| argon_037.bin | 7 | `nullify` | parse_value matches the 4-byte prefix 'null' with strncmp and advances offset to 4, so cJSON_ParseWithLengthOpts fails at offset 4 ('i') instead of 0. |
| argon_038.bin | 7 | `trueish` | parse_value matches the 4-byte prefix 'true' and advances offset to 4, failing at offset 4 ('i') instead of 0. |
| argon_039.bin | 9 | `falsehood` | parse_value matches the 5-byte prefix 'false' and advances offset to 5, failing at offset 5 ('h') instead of 0. |
| argon_040.bin | 7 | `[nulla]` | parse_value consumes 'null' inside an array and advances offset to 'a', so parse_array fails at offset 5 rather than offset 1. |
| argon_041.bin | 7 | `[true1]` | parse_value consumes 'true' and leaves '1' for parse_array, which fails expecting ',' or ']' at offset 5. |
| argon_042.bin | 8 | `[false_]` | parse_value consumes 'false' and leaves '_' for parse_array, which fails at offset 6. |
| argon_043.bin | 11 | `[true/*c*/]` | parse_value consumes 'true' and cJSON does not support comments in parse_array, failing at '/' (offset 5). |
| argon_044.bin | 14 | `//comment\ntrue` | parse_value does not skip '//' comments (only cJSON_Minify does), so parsing fails at offset 0. |
| argon_045.bin | 4 | `"abc` | parse_string fails in its first-pass scan for a closing quote before advancing input_pointer, setting the error offset to 1 (after '"') rather than 4. |
| argon_046.bin | 8 | `"a\\nb\\tc` | parse_string's first pass skips valid escapes but fails at EOF without moving input_pointer, reporting the error at offset 1. |
| argon_047.bin | 9 | `"abc\\xdef` | parse_string's first pass scans for '"' before validating escapes, so an unclosed string containing '\x' fails at offset 1 instead of offset 4. |
| argon_048.bin | 10 | `"abc\\xdef"` | parse_string's first pass succeeds on a closed string, and the second pass advances input_pointer to '\x' and fails at offset 4. |
| argon_049.bin | 22 | `["abc\\xdef","unclosed]` | parse_string fails in pass 2 on the first string's '\x' at offset 5 because the second string's opening quote was not needed to close the first string. |
| argon_050.bin | 10 | `{"a\\x:"b"}` | parse_string's pass 1 uses the quote before 'b' as the closing quote of the first string, so pass 2 runs and fails at '\x' (offset 3) instead of offset 2. |
| argon_051.bin | 8 | `{"a\\x:b}` | parse_string's pass 1 finds no closing quote in the buffer and fails at offset 2 ('a') without reaching '\x' in pass 2. |
| argon_052.bin | 5 | `"abc\\` | parse_string's pass 1 sees '\' at len-1, treats the driver's appended NUL at len as the escaped byte, and then fails at EOF with error offset 1. |
| argon_053.bin | 6 | `"abc\\"` | parse_string's pass 1 skips the escaped '\"' and fails at EOF, setting the error offset to 1. |
| argon_054.bin | 7 | `"abc\\\\"` | parse_string unescapes '\\' before the closing quote and print_string_ptr re-escapes it as '\\'. |
| argon_055.bin | 4 | `"\\a"` | parse_string rejects '\a' in pass 2 with error offset 1 (pointing to the backslash). |
| argon_056.bin | 4 | `"\\v"` | parse_string rejects vertical tab escape '\v' in pass 2 at offset 1. |
| argon_057.bin | 4 | `"\\0"` | parse_string rejects octal/null escape '\0' in pass 2 at offset 1. |
| argon_058.bin | 4 | `"\\'"` | parse_string rejects single-quote escape '\'' in pass 2 at offset 1. |
| argon_059.bin | 6 | `"a\\/b"` | parse_string unescapes '\/' to '/' and print_string_ptr outputs '/' without escaping it. |
| argon_060.bin | 18 | `"\\"\\\\\\b\\f\\n\\r\\t\\/"` | parse_string and print_string_ptr round-trip all eight JSON single-character escapes (normalizing '\/' to '/'). |
| argon_061.bin | 33 | `"\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f"` | parse_string accepts unescaped control characters 0x01..0x1f in string literals, and print_string_ptr formats them using '\b','\t','\n','\f','\r' or lowercase '\u00xx'. |
| argon_062.bin | 3 | `"\x7f"` | parse_string and print_string_ptr pass DEL (0x7f > 31) through unescaped. |
| argon_063.bin | 9 | `"abc\x00def"` | parse_string copies raw embedded NUL bytes into output, and print_string_ptr stops at the first NUL byte via *input_pointer != '\0', truncating the output to "abc". |
| argon_064.bin | 10 | `"abc\x00\ndef"` | print_string_ptr stops scanning for escape_characters at the embedded NUL before '\n', taking the escape_characters == 0 memcpy path and outputting "abc". |
| argon_065.bin | 10 | `"a\nbc\x00def"` | print_string_ptr takes the escaping loop because of '\n' before the embedded NUL and terminates the loop at the NUL byte, outputting "a\nbc". |
| argon_066.bin | 14 | `"abc\\u0000def"` | utf16_literal_to_utf8 encodes \u0000 as a raw 0x00 byte, causing print_string_ptr to truncate the printed string at "abc". |
| argon_067.bin | 8 | `"\\u0000"` | utf16_literal_to_utf8 writes 0x00 at index 0 for "\u0000", which print_string_ptr prints as an empty string "". |
| argon_068.bin | 14 | `"\x80\xff\xfe\x81\xc0\xaf\xed\xa0\x80\xed\xbf\xbf"` | parse_string and print_string_ptr copy arbitrary non-UTF-8 byte sequences (lone continuations, overlongs, UTF-8 surrogates, 0xfe/0xff) without validation. |
| argon_069.bin | 9 | `"\\\\u0041"` | parse_string treats '\\u0041' as an escaped backslash followed by literal 'u0041', not as U+0041. |
| argon_070.bin | 2 | `""` | parse_string and print_string_ptr handle an empty string literal "" via the zero-length fast path. |
| argon_071.bin | 8 | `"\\uZZZZ"` | parse_hex4 returns 0 instead of an error when given non-hex characters 'ZZZZ', so utf16_literal_to_utf8 treats \uZZZZ as U+0000 and succeeds, printing "". |
| argon_072.bin | 14 | `"abc\\u123Gdef"` | parse_hex4 returns 0 when the fourth hex digit 'G' is invalid, so utf16_literal_to_utf8 writes a NUL byte after 'abc' and print_string_ptr prints "abc". |
| argon_073.bin | 10 | `"x\\uD80Zy"` | parse_hex4 returns 0 when the fourth nibble of high-surrogate prefix \uD80Z is invalid, bypassing surrogate validation and truncating the printed string to "x". |
| argon_074.bin | 10 | `"\\u00\\"41"` | parse_string's pass 1 skips '\"' inside "\u00\"41", and pass 2's utf16_literal_to_utf8 consumes 6 bytes ('\u00\"') with parse_hex4 returning 0, leaving '41' and succeeding. |
| argon_075.bin | 16 | `"\\u0041\\u00\\"42"` | utf16_literal_to_utf8 decodes \u0041 to 'A' and then consumes '\u00\"' as a 6-byte sequence producing NUL, so print_string_ptr outputs "A". |
| argon_076.bin | 10 | `"\\u000\\\\n"` | utf16_literal_to_utf8 consumes the 6 bytes '\u000\' (returning 0 -> NUL byte), leaving the second '\' to pair with 'n' as '\n' so parse_string succeeds. |
| argon_077.bin | 10 | `"\\u000\\\\a"` | utf16_literal_to_utf8 consumes the 6 bytes '\u000\', leaving '\a' at offset 7 where parse_string fails instead of failing at '\u' (offset 1). |
| argon_078.bin | 4 | `"\\u"` | utf16_literal_to_utf8 fails when (input_end - first_sequence) < 6 on "\u", reporting error offset 1. |
| argon_079.bin | 5 | `"\\u1"` | utf16_literal_to_utf8 fails when only 1 hex digit follows '\u' before the closing quote, reporting error offset 1. |
| argon_080.bin | 6 | `"\\u12"` | utf16_literal_to_utf8 fails when only 2 hex digits follow '\u' before the closing quote, reporting error offset 1. |
| argon_081.bin | 7 | `"\\u123"` | utf16_literal_to_utf8 fails when only 3 hex digits follow '\u' before the closing quote, reporting error offset 1. |
| argon_082.bin | 9 | `"abc\\u123` | parse_string fails in pass 1 on an unclosed string with a truncated '\u123', reporting error offset 1 instead of offset 4. |
| argon_083.bin | 10 | `"abc\\u123"` | parse_string reaches pass 2 on a closed string with a truncated '\u123', reporting error offset 4. |
| argon_084.bin | 26 | `"\\u0001\\u001f\\u0020\\u007f"` | utf16_literal_to_utf8 encodes 1-byte UTF-8 codepoints U+0001, U+001F, U+0020, and U+007F, and print_string_ptr re-escapes only those < 32. |
| argon_085.bin | 14 | `"\\u0080\\u07ff"` | utf16_literal_to_utf8 encodes the 2-byte UTF-8 boundary codepoints U+0080 (0xc2 0x80) and U+07FF (0xdf 0xbf). |
| argon_086.bin | 26 | `"\\u0800\\ud7ff\\ue000\\uffff"` | utf16_literal_to_utf8 encodes the 3-byte UTF-8 boundary codepoints U+0800, U+D7FF, U+E000, and U+FFFF. |
| argon_087.bin | 26 | `"\\ud800\\udc00\\udbff\\udfff"` | utf16_literal_to_utf8 decodes the minimum (U+10000) and maximum (U+10FFFF) surrogate pairs into 4-byte UTF-8 sequences. |
| argon_088.bin | 14 | `"\\uD83D\\ude00"` | parse_hex4 handles mixed uppercase and lowercase hex digits in a surrogate pair (U+1F600). |
| argon_089.bin | 8 | `"\\udc00"` | utf16_literal_to_utf8 rejects an unpaired low surrogate \udc00 at offset 1. |
| argon_090.bin | 8 | `"\\udfff"` | utf16_literal_to_utf8 rejects the maximum unpaired low surrogate \udfff at offset 1. |
| argon_091.bin | 8 | `"\\ud800"` | utf16_literal_to_utf8 rejects a lone high surrogate \ud800 at the end of a string at offset 1. |
| argon_092.bin | 15 | `"a\\ud800bcdefg"` | utf16_literal_to_utf8 rejects a high surrogate followed by non-escape characters and leaves input_pointer at the first '\' (offset 2). |
| argon_093.bin | 16 | `"a\\ud800\\nbcdef"` | utf16_literal_to_utf8 rejects a high surrogate followed by '\n' and reports the error at the high surrogate's '\' (offset 2) rather than at '\n' (offset 8). |
| argon_094.bin | 14 | `"a\\ud800\\udc0"` | utf16_literal_to_utf8 rejects a high surrogate followed by a 5-byte truncated '\udc0' and reports the error at the first surrogate's '\' (offset 2). |
| argon_095.bin | 15 | `"a\\ud800\\ud800"` | utf16_literal_to_utf8 rejects two consecutive high surrogates and reports the error at the first surrogate's '\' (offset 2). |
| argon_096.bin | 15 | `"a\\ud800\\u0041"` | utf16_literal_to_utf8 rejects a high surrogate followed by a BMP escape \u0041 and reports the error at offset 2. |
| argon_097.bin | 15 | `"a\\ud800\\udc0g"` | parse_hex4 returns 0 on invalid hex in the second surrogate \udc0g, which fails the second_code < 0xDC00 check and reports the error at offset 2. |
| argon_098.bin | 15 | `"a\\udbff\\ue000"` | utf16_literal_to_utf8 rejects a high surrogate followed by \ue000 (just above 0xDFFF) and reports the error at offset 2. |
| argon_099.bin | 2 | `00` | parse_number delegates to strtod, which accepts numbers with leading zeros like 00 and prints 0. |
| argon_100.bin | 2 | `01` | parse_number accepts leading zeros in '01' via strtod and prints 1. |
| argon_101.bin | 4 | `-007` | parse_number accepts negative numbers with leading zeros '-007' via strtod and prints -7. |
| argon_102.bin | 5 | `00.25` | parse_number accepts leading zeros before a decimal point '00.25' via strtod and prints 0.25. |
| argon_103.bin | 2 | `1.` | parse_number accepts a trailing decimal point '1.' via strtod and prints 1. |
| argon_104.bin | 3 | `-0.` | parse_number accepts '-0.' via strtod and print_number outputs 0. |
| argon_105.bin | 5 | `42.e1` | parse_number accepts a decimal point immediately before an exponent '42.e1' via strtod and prints 420. |
| argon_106.bin | 3 | `-.5` | parse_value dispatches on leading '-' to parse_number, and strtod accepts '-.5' without a leading zero, printing -0.5. |
| argon_107.bin | 3 | `-.0` | parse_number accepts '-.0' via strtod and print_number outputs 0. |
| argon_108.bin | 6 | `-.25e2` | parse_number accepts '-.25e2' via strtod and print_number outputs -25. |
| argon_109.bin | 2 | `.5` | parse_value only enters parse_number on '-' or '0'..'9', so '.5' (unlike '-.5') is rejected at offset 0. |
| argon_110.bin | 2 | `+1` | parse_value does not dispatch '+' to parse_number, rejecting '+1' at offset 0. |
| argon_111.bin | 2 | `1e` | parse_number copies '1e' into number_c_string, strtod consumes '1' and sets after_end to 'e', so parse_number succeeds and cJSON_ParseWithLengthOpts fails at offset 1. |
| argon_112.bin | 3 | `1e+` | parse_number copies '1e+', strtod consumes only '1', and cJSON_ParseWithLengthOpts fails at offset 1 ('e') rather than offset 0. |
| argon_113.bin | 3 | `1e-` | parse_number copies '1e-', strtod consumes only '1', and cJSON_ParseWithLengthOpts fails at offset 1 ('e'). |
| argon_114.bin | 5 | `1.2.3` | parse_number copies '1.2.3', strtod consumes '1.2', and cJSON_ParseWithLengthOpts fails at the second '.' (offset 3). |
| argon_115.bin | 3 | `1+2` | parse_number copies '1+2', strtod consumes '1', and cJSON_ParseWithLengthOpts fails at '+' (offset 1). |
| argon_116.bin | 3 | `1-2` | parse_number copies '1-2', strtod consumes '1', and cJSON_ParseWithLengthOpts fails at '-' (offset 1). |
| argon_117.bin | 4 | `0x10` | parse_number stops copying before 'x' (so strtod never sees hex '0x10'), consumes '0', and fails at 'x' (offset 1). |
| argon_118.bin | 4 | `[1e]` | parse_number consumes '1' from '1e' inside an array, causing parse_array to fail at 'e' (offset 2) instead of offset 1. |
| argon_119.bin | 1 | `-` | parse_number calls strtod on '-', which sets after_end == number_c_string and fails at offset 0. |
| argon_120.bin | 2 | `-.` | parse_number calls strtod on '-.', which consumes 0 bytes and fails at offset 0. |
| argon_121.bin | 3 | `--1` | parse_number calls strtod on '--1', which consumes 0 bytes and fails at offset 0. |
| argon_122.bin | 3 | `-+1` | parse_number calls strtod on '-+1', which consumes 0 bytes and fails at offset 0. |
| argon_123.bin | 3 | `-e1` | parse_number calls strtod on '-e1', which consumes 0 bytes and fails at offset 0. |
| argon_124.bin | 2 | `-0` | print_number checks d == (double)item->valueint, which is true for -0.0 == 0.0, so '-0' is printed via '%d' as '0' without a minus sign. |
| argon_125.bin | 4 | `-0.0` | print_number formats '-0.0' as '0' because -0.0 == (double)0 takes the integer '%d' branch. |
| argon_126.bin | 7 | `-1e-400` | parse_number underflows '-1e-400' to -0.0, and print_number outputs '0' via the item->valueint equality check. |
| argon_127.bin | 10 | `2147483647` | parse_number saturates INT_MAX (2147483647) into valueint, and print_number takes the '%d' branch. |
| argon_128.bin | 10 | `2147483648` | parse_number saturates 2147483648 (INT_MAX + 1) to valueint = INT_MAX, so d == (double)item->valueint is false and print_number uses '%1.15g'. |
| argon_129.bin | 11 | `-2147483648` | parse_number sets valueint = INT_MIN (-2147483648), and print_number prints it via the '%d' branch. |
| argon_130.bin | 11 | `-2147483649` | parse_number saturates -2147483649 (INT_MIN - 1) to valueint = INT_MIN, so print_number falls through to '%1.15g'. |
| argon_131.bin | 12 | `2147483647.5` | parse_number saturates 2147483647.5 to valueint = INT_MAX, and print_number prints 2147483647.5 because d != (double)valueint. |
| argon_132.bin | 13 | `-2147483648.5` | parse_number saturates -2147483648.5 to valueint = INT_MIN, and print_number prints -2147483648.5 via '%1.15g'. |
| argon_133.bin | 5 | `1e309` | parse_number accepts strtod overflow to +INFINITY on 1e309, and print_number outputs 'null' because isinf(d) is true. |
| argon_134.bin | 6 | `-1e309` | parse_number accepts strtod overflow to -INFINITY on -1e309, and print_number outputs 'null'. |
| argon_135.bin | 16 | `[1e9999,-1e9999]` | print_number prints both positive and negative overflowed numbers inside an array as [null,null]. |
| argon_136.bin | 15 | `999999999999999` | print_number formats 10^15 - 1 (15 digits) with '%1.15g' in fixed-point notation as 999999999999999. |
| argon_137.bin | 16 | `1000000000000000` | print_number formats 10^15 with '%1.15g' in scientific notation as '1e+15' because the exponent (15) equals the precision (15). |
| argon_138.bin | 16 | `1000000000000001` | print_number's '%1.15g' attempt ('1e+15') fails compare_double on 10^15 + 1, so it switches to '%1.17g' where exponent 15 < 17 prints fixed-point '1000000000000001'. |
| argon_139.bin | 20 | `1000000000000000.125` | print_number formats 1000000000000000.125 with '%1.15g' as '1e+15', and compare_double returns true because 0.125 <= 1e15 * DBL_EPSILON (0.22204). |
| argon_140.bin | 19 | `1000000000000000.25` | print_number's '%1.15g' fails compare_double on 1000000000000000.25 (0.25 > 0.22204), switching to '%1.17g' which rounds half-to-even to '1000000000000000.2'. |
| argon_141.bin | 18 | `100000000000000000` | print_number formats 10^17 as '1e+17' because the exponent is >= 17. |
| argon_142.bin | 18 | `1.0000000000000002` | print_number formats 1.0 + DBL_EPSILON (1.0000000000000002) with '%1.15g' as '1', and compare_double(1.0, d) returns true because fabs(1.0 - d) <= d * DBL_EPSILON. |
| argon_143.bin | 18 | `1.0000000000000004` | print_number's '%1.15g' ('1') fails compare_double on 1.0 + 2*DBL_EPSILON (1.0000000000000004), switching to '%1.17g' to output '1.0000000000000004'. |
| argon_144.bin | 18 | `1.9999999999999996` | print_number formats 2.0 - 2*2^-52 (1.9999999999999996) with '%1.15g' as '2', and compare_double(2.0, d) returns true because 2*2^-52 <= 2.0 * DBL_EPSILON. |
| argon_145.bin | 18 | `1.9999999999999993` | print_number's '%1.15g' ('2') fails compare_double on 2.0 - 3*2^-52 (1.9999999999999993), switching to '%1.17g' to output '1.9999999999999993'. |
| argon_146.bin | 19 | `0.30000000000000004` | print_number formats 0.30000000000000004 with '%1.15g' as '0.3', and compare_double returns true because the 1-ULP difference (2^-54) is <= 0.3 * DBL_EPSILON. |
| argon_147.bin | 18 | `0.1234567890123456` | print_number's '%1.15g' loses precision on 16 significant digits (0.1234567890123456), failing compare_double and switching to '%1.17g'. |
| argon_148.bin | 6 | `0.0001` | print_number formats 1e-4 with '%1.15g' in fixed-point notation as '0.0001' because exponent -4 is not < -4. |
| argon_149.bin | 7 | `0.00001` | print_number formats 1e-5 with '%1.15g' in scientific notation with a two-digit exponent as '1e-05' because exponent -5 < -4. |
| argon_150.bin | 22 | `0.00009999999999999999` | print_number's '%1.15g' rounds 0.00009999999999999999 up to 0.0001 (exponent -4 -> '0.0001'), and compare_double accepts it. |
| argon_151.bin | 23 | `1.7976931348623157e+308` | print_number's '%1.15g' rounds DBL_MAX to '1.79769313486232e+308', which overflows sscanf to +INF, and compare_double(+INF, DBL_MAX) evaluates +INF <= +INF as true. |
| argon_152.bin | 23 | `2.2250738585072014e-308` | print_number formats DBL_MIN (2.2250738585072014e-308) with '%1.15g' as '2.2250738585072e-308' because compare_double accepts the 15-digit rounding. |
| argon_153.bin | 6 | `5e-324` | print_number formats the minimum positive subnormal double (5e-324) with '%1.15g' as '4.94065645841247e-324'. |
| argon_154.bin | 7 | `-5e-324` | print_number formats the minimum negative subnormal double (-5e-324) with '%1.15g' as '-4.94065645841247e-324'. |
| argon_155.bin | 16 | `9007199254740992` | print_number formats 2^53 (9007199254740992, 16 digits) via '%1.17g' as fixed-point '9007199254740992' after '%1.15g' ('9.00719925474099e+15') fails compare_double. |
| argon_156.bin | 10 | `123.456e78` | print_number formats large exponents with '%1.15g' using a plus sign as '1.23456e+80'. |
| argon_157.bin | 2 | `[]` | parse_array and print_array handle an empty array []. |
| argon_158.bin | 7 | `[ \t\r\n\x00]` | parse_array handles whitespace and embedded NUL inside an empty array. |
| argon_159.bin | 1 | `[` | parse_array advances past '[' and fails in parse_value at the trailing NUL (offset 1). |
| argon_160.bin | 4 | `[   ` | parse_array skips whitespace after '[' to the trailing NUL, steps back and forward, and fails in parse_value at offset 4. |
| argon_161.bin | 3 | `[,]` | parse_array fails in parse_value on a leading comma at offset 1. |
| argon_162.bin | 4 | `[1,]` | parse_array fails in parse_value on a trailing comma when seeing ']' at offset 3. |
| argon_163.bin | 6 | `[1,,2]` | parse_array fails in parse_value on consecutive commas at offset 3. |
| argon_164.bin | 5 | `[1 2]` | parse_array fails when expecting ',' or ']' after the first element at offset 3 ('2'). |
| argon_165.bin | 2 | `[1` | parse_array fails when EOF is reached after an element without ']', reporting offset 2. |
| argon_166.bin | 3 | `[1,` | parse_array fails in parse_value when EOF is reached immediately after a comma, reporting offset 3. |
| argon_167.bin | 2 | `{}` | parse_object and print_object handle an empty object {}. |
| argon_168.bin | 7 | `{ \t\r\n\x00}` | parse_object handles whitespace and embedded NUL inside an empty object. |
| argon_169.bin | 1 | `{` | parse_object calls parse_string at offset 1 ('\0'), which sets offset to 2 (== buffer.length), and cJSON_ParseWithLengthOpts clamps the error position to buffer.length - 1 (offset 1). |
| argon_170.bin | 4 | `{   ` | parse_object skips spaces after '{' and fails at offset 4. |
| argon_171.bin | 5 | `{1:2}` | parse_object calls parse_string on non-string key '1' (offset 1), and parse_string initializes input_pointer = offset + 1 before checking for '"', advancing the error offset to 2 (':')! |
| argon_172.bin | 3 | `{,}` | parse_object calls parse_string at ',' (offset 1), and parse_string's fail block sets input_buffer->offset to input_pointer (offset 2, at '}'). |
| argon_173.bin | 8 | `{"a":1,}` | parse_object calls parse_string after ',' at '}' (offset 7), and parse_string's input_pointer + 1 initialization advances the error offset to 8 (one past '}'). |
| argon_174.bin | 7 | `{"a":1,` | parse_object calls parse_string after ',' at the appended NUL (offset 7), which sets offset to 8 and gets clamped by cJSON_ParseWithLengthOpts to offset 7. |
| argon_175.bin | 12 | `{"a":1, 2:3}` | parse_object calls parse_string on the second key '2' (offset 8), which fails on non-'"' and sets the error offset to 9 (':'). |
| argon_176.bin | 7 | `{"a" 1}` | parse_object fails when ':' is missing after the key string at offset 5 ('1'). |
| argon_177.bin | 4 | `{"a"` | parse_object fails when EOF is reached right after a key string without ':', reporting offset 4. |
| argon_178.bin | 6 | `{"a":}` | parse_object fails in parse_value when '}' immediately follows ':' at offset 5. |
| argon_179.bin | 5 | `{"a":` | parse_object fails in parse_value when EOF immediately follows ':' at offset 5. |
| argon_180.bin | 13 | `{"a":1 "b":2}` | parse_object fails when ',' is missing between key-value pairs at offset 7 ('"'). |
| argon_181.bin | 6 | `{"a":1` | parse_object fails when '}' is missing after a key-value pair at offset 6. |
| argon_182.bin | 25 | `{"z":1,"a":2,"z":3,"m":4}` | parse_object stores children in a linked list, preserving insertion order and duplicate keys in print_object rather than sorting or deduplicating. |
| argon_183.bin | 27 | `{"a\\u0000b":1,"a\\u0000c":2}` | print_object calls print_string_ptr on object keys, truncating both keys at their decoded NUL byte to produce {"a":1,"a":2}. |
| argon_184.bin | 35 | `{"":[{"":[]},null,true,false,0,""]}` | print_object and print_array format nested empty keys, arrays, booleans, null, number, and string without extra whitespace. |
| argon_185.bin | 6 | `[1][2]` | cJSON_ParseWithLengthOpts with require_null_terminated rejects a second concatenated JSON array at offset 3. |
| argon_186.bin | 6 | `"a""b"` | cJSON_ParseWithLengthOpts with require_null_terminated rejects a second concatenated string at offset 3. |
| argon_187.bin | 4 | `{}{}` | cJSON_ParseWithLengthOpts with require_null_terminated rejects a second concatenated object at offset 2. |
| argon_188.bin | 255 | `"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` | print_string_ptr on a 253-byte string requests 256 bytes in ensure, exactly matching print's initial default_buffer_size = 256 without reallocating. |
| argon_189.bin | 256 | `"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` | print_string_ptr on a 254-byte string requests 257 bytes in ensure, crossing print's default_buffer_size = 256 and triggering buffer reallocation. |
| argon_190.bin | 102 | `"\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01` | print_string_ptr expands 100 raw 0x01 bytes into 600 bytes of '\u0001' escapes, testing escape_characters += 5 sizing and ensure reallocation. |
| argon_191.bin | 286 | `[1000,1001,1002,1003,1004,1005,1006,1007` | print_array prints 57 four-digit numbers (> 256 bytes) across printbuffer growth while updating offsets between elements. |
| argon_192.bin | 1940 | `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[` | parse_array checks input_buffer->depth >= CJSON_NESTING_LIMIT (1000) before incrementing depth, so exactly 1000 nested arrays succeed. |
| argon_193.bin | 2002 | `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[` | parse_array rejects 1001 nested arrays when input_buffer->depth == 1000 >= CJSON_NESTING_LIMIT, failing at offset 1000. |
| argon_194.bin | 2002 | `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[` | parse_array decrements input_buffer->depth on success, so two sibling arrays at depth 1000 both succeed instead of counting cumulative depth. |
| argon_195.bin | 3516 | `{"":{"":{"":{"":{"":{"":{"":{"":{"":{"":` | parse_object and parse_array share input_buffer->depth, allowing 500 nested objects containing 500 nested arrays (total depth 1000). |
| argon_196.bin | 3450 | `{"":{"":{"":{"":{"":{"":{"":{"":{"":{"":` | parse_object rejects an innermost object at combined object+array depth 1001 (500 objects + 500 arrays + 1 object), failing at offset 2500. |
| argon_197.bin | 3 | `'a'` | parse_value rejects a single-quoted string at offset 0. |
| argon_198.bin | 3 | `NaN` | parse_value does not dispatch 'N' to strtod, rejecting bare NaN at offset 0. |
| argon_199.bin | 8 | `Infinity` | parse_value does not dispatch 'I' to strtod, rejecting bare Infinity at offset 0. |
| argon_200.bin | 9 | `-Infinity` | parse_number only copies chars in '0'-'9','+','-','e','E','.', so '-Infinity' copies only '-' into number_c_string and strtod fails at offset 0. |
| argon_201.bin | 4 | `-NaN` | parse_number only copies '-' from '-NaN' into number_c_string rather than passing '-NaN' to strtod, failing at offset 0. |
| argon_202.bin | 12 | `1e0000000000` | parse_number parses '1e0000000000' as 1.0 (with valueint = 1) and print_number outputs '1'. |
| argon_203.bin | 23 | `0.000000000000000000000` | parse_number parses '0.000000000000000000000' as 0.0 (valueint = 0) and print_number outputs '0' via '%d'. |
| argon_204.bin | 13 | `-2147483648.0` | parse_number sets valueint = INT_MIN for '-2147483648.0', and print_number prints '-2147483648' without '.0' because d == (double)item->valueint. |
| argon_205.bin | 12 | `2147483647.0` | parse_number sets valueint = INT_MAX for '2147483647.0', and print_number prints '2147483647' without '.0' because d == (double)item->valueint. |
| argon_206.bin | 12 | `2147483648.0` | parse_number saturates '2147483648.0' to valueint = INT_MAX, and print_number prints '2147483648' without '.0' because '%1.15g' strips trailing zeros. |
| argon_207.bin | 44 | `"\\u0008\\u0009\\u000a\\u000c\\u000d\\u0022\\u0` | utf16_literal_to_utf8 decodes \u0008,\u0009,\u000a,\u000c,\u000d,\u0022,\u005c to raw ASCII bytes, which print_string_ptr re-escapes as \b,\t,\n,\f,\r,\",\\. |
| argon_208.bin | 23 | `["a\\u0000b","c\\u0000d"]` | print_array calls update_offset after each print_string_ptr, which uses strlen up to the decoded NUL byte so subsequent array elements are placed right after each truncated string. |
| argon_209.bin | 36 | `{"a":1,"b":[2,3,{"c":"d"}],"e":-0.0}` | cJSON_ParseWithLengthOpts and cJSON_PrintUnformatted round-trip a composite object/array structure while normalizing -0.0 to 0. |
| argon_210.bin | 4 | `"\\?"` | parse_string rejects C trigraph/question-mark escape '\?' in pass 2 at offset 1. |
| argon_211.bin | 4 | `"\\1"` | parse_string rejects numeric escape '\1' in pass 2 at offset 1. |
| argon_212.bin | 1 | `!` | buffer_skip_whitespace stops at 0x21 ('!', the first byte > 32), and parse_value fails at offset 0. |
| argon_213.bin | 1 | `n` | parse_value fails on single-byte 'n' when can_read(input_buffer, 4) is false, reporting offset 0. |
| argon_214.bin | 1 | `t` | parse_value fails on single-byte 't' when can_read(input_buffer, 4) is false, reporting offset 0. |
| argon_215.bin | 1 | `f` | parse_value fails on single-byte 'f' when can_read(input_buffer, 5) is false, reporting offset 0. |
| argon_216.bin | 4 | `TRUE` | parse_value uses case-sensitive strncmp and rejects uppercase 'TRUE' at offset 0. |
| argon_217.bin | 5 | `False` | parse_value uses case-sensitive strncmp and rejects mixed-case 'False' at offset 0. |
| argon_218.bin | 7 | `123eabc` | parse_number copies '123e' (stopping before 'a'), strtod consumes only '123', and cJSON_ParseWithLengthOpts fails at 'e' (offset 3) rather than 'a' (offset 4). |
| argon_219.bin | 9 | `-0.125e-3` | parse_number and print_number round-trip '-0.125e-3' to '-0.000125' (exponent -4 uses fixed-point in '%1.15g'). |
| argon_220.bin | 9 | `-0.125e-4` | parse_number and print_number round-trip '-0.125e-4' to '-1.25e-05' (exponent -5 uses scientific notation in '%1.15g'). |
| argon_221.bin | 17 | `100000000000000.5` | print_number's '%1.15g' rounds 100000000000000.5 (16 significant digits) to '1e+14', which fails compare_double and switches to '%1.17g' ('100000000000000.5'). |
| argon_222.bin | 27 | `2147483646.9999999999999999` | parse_number rounds 2147483646.9999999999999999 to (double)INT_MAX in strtod, saturates valueint = INT_MAX, and print_number outputs 2147483647. |
| argon_223.bin | 11 | `[1,2]  \x00  3` | cJSON_ParseWithLengthOpts with require_null_terminated skips spaces and an embedded NUL after [1,2] via buffer_skip_whitespace and fails at '3' (offset 10). |
| argon_224.bin | 17 | `{"a":"b\\x","c":1}` | parse_object fails inside parse_string's pass 2 on '\x' in the first value string at offset 7. |
