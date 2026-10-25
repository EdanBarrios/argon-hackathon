"""Targeted adversarial inputs for cJSON parse + PrintUnformatted.

Stdlib only, deterministic (no randomness). One input per file, written to
corpus/targeted/<group>_<name>.bin. Each group aims at semantics a port is
likely to get subtly wrong: number formatting, string escapes and UTF-8/UTF-16,
nesting limits, whitespace, BOM, trailing bytes, NUL handling, literals.

Usage: python3 -I gen_targeted.py [--out DIR]
"""

import argparse
import os
import struct

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_OUT = os.path.join(HERE, "corpus", "targeted")


def f64(bits):
    return struct.unpack("<d", struct.pack("<Q", bits))[0]


def numbers():
    out = {}
    lit = [
        # round-trip boundary of %1.15g vs %1.17g
        "0.1", "0.2", "0.3", "0.30000000000000004", "0.1000000000000001",
        "1.0000000000000002", "0.9999999999999999", "123456789012345.6",
        "1234567890123456.7", "12345678901234567", "3.141592653589793",
        "2.718281828459045", "1e15", "1e16", "1e17", "1e21", "1e22", "1e-5",
        "1e-6", "1e-7", "100000000000000000000", "0.000001", "0.0000001",
        "1.5", "2.5", "-1.5", "123.456", "1e-10", "9.999999999999999e22",
        "4.35", "0.07", "1.005", "8.41e21",
        # 2^53 +- 1..3
        "9007199254740989", "9007199254740990", "9007199254740991",
        "9007199254740992", "9007199254740993", "9007199254740994",
        "9007199254740995", "-9007199254740993",
        # INT_MAX / INT_MIN +- 1 (valueint saturation)
        "2147483646", "2147483647", "2147483648", "-2147483647", "-2147483648",
        "-2147483649", "2147483647.5", "-2147483648.5", "4294967296",
        # 2^63, 2^64
        "9223372036854775807", "9223372036854775808", "-9223372036854775808",
        "18446744073709551616",
        # denormals
        "5e-324", "4.9406564584124654e-324", "2.2250738585072014e-308",
        "2.2250738585072009e-308", "1e-310", "2.225e-311", "1e-323",
        # zeros
        "0", "-0", "-0.0", "0.0", "0e5", "0E-5", "-0e0", "0.000",
        # extremes
        "1e308", "1.7976931348623157e308", "1.7976931348623158e308",
        "1.7976931348623159e308", "1e309", "-1e309", "1e-324", "2e-324",
        "3e-324", "1e999", "-1e999", "1e-999",
        # exponent spellings
        "1E5", "1e5", "1e+5", "1E+5", "1e-05", "1E-05", "1e005", "1e+005",
        "1.5E+10", "1.e5", "1e0", "1e-0", "1e+0", "10e-1",
        # malformed-ish
        "01", "00", "-01", "-", "+1", ".5", "5.", "1e", "1e+", "1e-", "1E",
        "-.5", "--1", "1..2", "1.2.3", "1ee5", "0x1F", "0X10", "1_000",
        "Infinity", "-Infinity", "NaN", "nan", "inf", "-inf", "1,5",
        # strtod accepts hex floats and inf/nan; cJSON copies only number chars
        "0x1p3", "1e5x", "1.5f", "1d5", "1E5E5",
    ]
    for i, s in enumerate(lit):
        b = s.encode()
        out["num_bare_%03d" % i] = b
        out["num_arr_%03d" % i] = b"[" + b + b"]"
    # very long digit strings
    out["num_long_int_400"] = b"1" * 400
    out["num_long_frac_400"] = b"0." + b"1" * 400
    out["num_long_int_70"] = b"[" + b"9" * 70 + b"]"
    out["num_long_zeros_frac"] = b"[0." + b"0" * 330 + b"1]"
    out["num_long_exp"] = b"[1e" + b"0" * 300 + b"5]"
    out["num_long_mantissa_1e308"] = b"[" + b"1" + b"0" * 308 + b"]"
    out["num_long_mantissa_1e309"] = b"[" + b"1" + b"0" * 309 + b"]"
    out["num_64_chars"] = b"[" + b"1" * 63 + b"]"
    out["num_long_2000"] = b"[" + b"7" * 2000 + b".5]"
    # doubles at bit level near formatting boundaries
    for name, bits in [("min_subnormal", 1), ("max_subnormal", 0x000FFFFFFFFFFFFF),
                       ("min_normal", 0x0010000000000000), ("max", 0x7FEFFFFFFFFFFFFF),
                       ("one_plus_ulp", 0x3FF0000000000001), ("one_minus_ulp", 0x3FEFFFFFFFFFFFFF)]:
        out["num_bits_" + name] = b"[" + repr(f64(bits)).encode() + b"]"
        out["num_bits17_" + name] = b"[" + ("%.17g" % f64(bits)).encode() + b"]"
    # mixed numbers in one array
    out["num_mixed"] = b"[0.1,-0,1e308,5e-324,2147483648,-2147483649,1e400,0.30000000000000004]"
    return out


ESCAPES = [b'\\"', b"\\\\", b"\\/", b"\\b", b"\\f", b"\\n", b"\\r", b"\\t"]


def strings():
    out = {}
    for c in range(0x20):
        out["str_ctrl_raw_%02x" % c] = b'"a' + bytes([c]) + b'b"'
        out["str_ctrl_u_%02x" % c] = b'"a\\u%04xb"' % c
        out["str_ctrl_uhex_upper_%02x" % c] = b'"\\u%04X"' % c
    out["str_del_raw"] = b'"\x7f"'
    out["str_del_u"] = b'"\\u007f"'
    for i, e in enumerate(ESCAPES):
        out["str_esc_%d" % i] = b'"' + e + b'"'
        out["str_esc_key_%d" % i] = b'{"' + e + b'":' + b'"' + e + b'"}'
    out["str_esc_all"] = b'"' + b"".join(ESCAPES) + b'"'
    for i, bad in enumerate([b"\\x41", b"\\a", b"\\v", b"\\0", b"\\'", b"\\u12", b"\\u", b"\\u123",
                             b"\\uZZZZ", b"\\u12G4", b"\\U0041", b"\\", b"\\ ", b"\\\n",
                             b"\\u-123", b"\\u+123", b"\\u 123", b"\\e"]):
        out["str_badesc_%02d" % i] = b'"' + bad + b'"'
        out["str_badesc_tail_%02d" % i] = b'["ok","' + bad + b'"]'
    # unicode escapes
    for name, esc in [
        ("u0000", b"\\u0000"), ("a_u0000_b", b"a\\u0000b"), ("u0001", b"\\u0001"),
        ("u007f", b"\\u007f"), ("u0080", b"\\u0080"), ("u00ff", b"\\u00ff"),
        ("u07ff", b"\\u07ff"), ("u0800", b"\\u0800"), ("uffff", b"\\uffff"),
        ("ufffe", b"\\ufffe"), ("ufeff", b"\\ufeff"), ("u2028", b"\\u2028"),
        ("mixed_case", b"\\uAbCd"),
        ("pair_d800_dc00", b"\\ud800\\udc00"), ("pair_d800_dfff", b"\\ud800\\udfff"),
        ("pair_dbff_dc00", b"\\udbff\\udc00"), ("pair_dbff_dfff", b"\\udbff\\udfff"),
        ("pair_d83d_de00", b"\\uD83D\\uDE00"),
        ("lone_high_d800", b"\\ud800"), ("lone_high_dbff", b"\\udbff"),
        ("lone_low_dc00", b"\\udc00"), ("lone_low_dfff", b"\\udfff"),
        ("reversed", b"\\udc00\\ud800"), ("high_high", b"\\ud800\\ud800"),
        ("high_then_ascii", b"\\ud800\\u0041"), ("high_then_char", b"\\ud800x"),
        ("high_then_n", b"\\ud800\\n"), ("high_then_short", b"\\ud800\\u12"),
        ("high_then_end", b"x\\ud800"), ("high_then_backslash", b"\\ud800\\"),
        ("high_then_u_bad", b"\\ud800\\uzzzz"), ("high_then_U", b"\\ud800\\Udc00"),
        ("pair_then_text", b"\\ud800\\udc00abc"), ("two_pairs", b"\\ud800\\udc00\\udbff\\udfff"),
        ("high_then_dbff", b"\\ud800\\udbff"), ("low_d7ff_dc00", b"\\ud7ff\\udc00"),
        ("high_then_e000", b"\\ud800\\ue000"),
    ]:
        out["str_u_" + name] = b'"' + esc + b'"'
        out["str_ukey_" + name] = b'{"' + esc + b'":1}'
    # raw UTF-8 and invalid bytes (cJSON passes bytes through)
    for name, raw in [
        ("utf8_2", b"\xc3\xa9"), ("utf8_3", b"\xe2\x82\xac"), ("utf8_4", b"\xf0\x9f\x98\x80"),
        ("utf8_4_max", b"\xf4\x8f\xbf\xbf"), ("utf8_above_max", b"\xf4\x90\x80\x80"),
        ("overlong_slash", b"\xc0\xaf"), ("overlong_nul", b"\xc0\x80"),
        ("overlong_3", b"\xe0\x80\xaf"), ("overlong_4", b"\xf0\x80\x80\xaf"),
        ("surrogate_raw", b"\xed\xa0\x80"), ("lone_cont", b"\x80"), ("bad_cont", b"\xc3\x28"),
        ("trunc_2", b"\xc3"), ("trunc_4", b"\xf0\x9f\x98"), ("ff", b"\xff"), ("fe", b"\xfe"),
        ("f8", b"\xf8\x88\x80\x80\x80"), ("all_high",
                                         bytes(range(0x80, 0x100))),
        ("bom_in_string", b"\xef\xbb\xbf"), ("quote_after_lead", b"\xc3"),
    ]:
        out["str_raw_" + name] = b'"' + raw + b'"'
    out["str_raw_highbytes_key"] = b'{"\xff\xfe":"\x80"}'
    out["str_empty"] = b'""'
    out["str_unterminated"] = b'"abc'
    out["str_unterminated_esc"] = b'"abc\\'
    out["str_unterminated_esc_quote"] = b'"abc\\"'
    out["str_long_8k"] = b'"' + b"x" * 8000 + b'"'
    out["str_long_escapes"] = b'"' + b"\\n" * 2000 + b'"'
    out["str_long_ctrl"] = b'"' + b"\\u0001" * 1000 + b'"'
    out["str_long_unicode"] = b'"' + b"\\ud83d\\ude00" * 500 + b'"'
    # \u with invalid hex swallows a backslash the first (length) pass paired
    # differently, so the second pass reads the closing quote as an escape
    # target. C accepts (its check is `input_end - input_pointer < 1`).
    # Found by scratch fuzzing: gemini-3.8-flash's port rejects these.
    for i, body in enumerate([b"\\uZ\\t\\\\", b"\\uZ\\\\\\\\", b"\\u\xd4\\t\\\\", b"\\uZ\\\"\\\\",
                              b"\\u000\\\\", b"\\u00\\n\\\\", b"a\\uZZZ\\\\"]):
        out["str_straddle_%d" % i] = b'"' + body + b'"'
        out["str_straddle_arr_%d" % i] = b'["' + body + b'",1]'
        out["str_straddle_key_%d" % i] = b'{"' + body + b'":1}'
    out["str_straddle_obj_fuzz_min"] = (b'{' + b'"":"",' * 8 + b'"\\u\xd4\\t\\\\"')
    out["str_single_quote"] = b"'a'"
    out["str_nul_raw_mid"] = b'"a\x00b"'
    return out


def structure():
    out = {}
    for depth in range(997, 1004):
        out["nest_arr_%d" % depth] = b"[" * depth + b"]" * depth
        out["nest_obj_%d" % depth] = b'{"a":' * (depth - 1) + b"{}" + b"}" * (depth - 1)
        out["nest_mix_%d" % depth] = (b'[{"a":' * (depth // 2) + (b"[]" if depth % 2 == 0 else b"[[]]")
                                      + b"}]" * (depth // 2))
        out["nest_obj_in_arr_tail_%d" % depth] = b"[" * (depth - 1) + b"{}" + b"]" * (depth - 1)
        out["nest_unclosed_%d" % depth] = b"[" * depth
        out["nest_arr_val_%d" % depth] = b"[" * depth + b"1" + b"]" * depth
    out["nest_10000"] = b"[" * 10000 + b"]" * 10000
    out["nest_obj_bad_at_1001"] = b'{"a":' * 1001 + b"1" + b"}" * 1001
    out["key_empty"] = b'{"":1}'
    out["key_empty_dup"] = b'{"":1,"":2}'
    out["key_dup"] = b'{"a":1,"a":2,"a":3}'
    out["key_dup_case"] = b'{"a":1,"A":2}'
    out["key_nul_escape"] = b'{"a\\u0000b":1,"a":2}'
    out["array_10k"] = b"[" + b",".join(b"%d" % (i % 10) for i in range(10000)) + b"]"
    out["array_10k_null"] = b"[" + b",".join([b"null"] * 10000) + b"]"
    out["object_2k"] = b"{" + b",".join(b'"k%d":%d' % (i, i) for i in range(2000)) + b"}"
    # whitespace variants
    ws = {"sp": b" ", "tab": b"\t", "lf": b"\n", "cr": b"\r", "crlf": b"\r\n",
          "vt": b"\x0b", "ff": b"\x0c", "nul": b"\x00", "nbsp": b"\xc2\xa0",
          "x01": b"\x01", "x1f": b"\x1f", "x7f": b"\x7f", "x80": b"\x80", "x20x0b": b" \x0b"}
    template = [b"{", b'"a"', b":", b"[", b"1", b",", b"true", b"]", b",", b'"b"', b":", b"null", b"}"]
    for name, w in ws.items():
        out["ws_all_" + name] = w + w.join(template) + w
        out["ws_lead_" + name] = w + b"[1]"
        out["ws_trail_" + name] = b"[1]" + w
        out["ws_colon_" + name] = b'{"a"' + w + b":1}"
        out["ws_comma_" + name] = b"[1" + w + b",2]"
        out["ws_after_open_" + name] = b"[" + w + b"1]"
        out["ws_before_close_" + name] = b'{"a":1' + w + b"}"
    out["ws_only_lots"] = b" \t\r\n" * 50
    # BOM
    out["bom_lead"] = b"\xef\xbb\xbf[1]"
    out["bom_lead_ws"] = b"\xef\xbb\xbf  {}"
    out["bom_ws_lead"] = b" \xef\xbb\xbf[1]"
    out["bom_double"] = b"\xef\xbb\xbf\xef\xbb\xbf[1]"
    out["bom_mid"] = b"[1,\xef\xbb\xbf2]"
    out["bom_trail"] = b"[1]\xef\xbb\xbf"
    out["bom_only"] = b"\xef\xbb\xbf"
    out["bom_partial"] = b"\xef\xbb[1]"
    out["bom_partial1"] = b"\xef"
    out["bom_utf16le"] = b"\xff\xfe[\x001\x00]\x00"
    out["bom_scalar"] = b"\xef\xbb\xbf42"
    # trailing garbage
    for name, tail in [("x", b"x"), ("space_x", b" x"), ("comma", b","), ("close", b"]"),
                       ("value", b"1"), ("obj", b"{}"), ("nl_x", b"\nx"), ("nul_x", b"\x00x"),
                       ("nul", b"\x00"), ("nul_nul", b"\x00\x00"), ("ws_nul", b" \x00")]:
        out["trail_arr_" + name] = b"[1]" + tail
        out["trail_num_" + name] = b"12" + tail
        out["trail_lit_" + name] = b"true" + tail
        out["trail_str_" + name] = b'"s"' + tail
    # NULs and empties
    out["nul_only"] = b"\x00"
    out["nul_many"] = b"\x00" * 8
    out["nul_mid_arr"] = b"[1,\x002]"
    out["nul_mid_obj"] = b'{"a":\x001}'
    out["nul_mid_key"] = b'{"a\x00b":1}'
    out["nul_before_value"] = b"\x00[1]"
    out["empty"] = b""
    out["ws_space"] = b" "
    # structural errors at varied offsets
    for name, data in [
        ("trailing_comma_arr", b"[1,2,]"), ("trailing_comma_obj", b'{"a":1,}'),
        ("lead_comma", b"[,1]"), ("double_comma", b"[1,,2]"), ("missing_colon", b'{"a" 1}'),
        ("missing_value", b'{"a":}'), ("key_number", b"{1:1}"), ("key_null", b"{null:1}"),
        ("unclosed_arr", b"[1,2"), ("unclosed_obj", b'{"a":1'), ("unclosed_obj_key", b'{"a"'),
        ("unclosed_obj_colon", b'{"a":'), ("unclosed_obj_comma", b'{"a":1,'),
        ("unclosed_arr_comma", b"[1,"), ("mismatch1", b"[}"), ("mismatch2", b"{]"),
        ("only_close", b"]"), ("only_open", b"["), ("only_open_obj", b"{"), ("colon", b":"),
        ("comma", b","), ("arr_colon", b"[1:2]"), ("obj_arr_member", b'{"a",1}'),
        ("comment_line", b"[1]//c"), ("comment_block", b"/*c*/[1]"),
    ]:
        out["err_" + name] = data
    return out


def literals():
    out = {}
    for w in [b"true", b"false", b"null"]:
        out["lit_%s" % w.decode()] = w
        out["lit_arr_%s" % w.decode()] = b"[" + w + b"]"
        out["lit_upper_%s" % w.decode()] = w.upper()
        out["lit_cap_%s" % w.decode()] = w.capitalize()
        out["lit_mixed_%s" % w.decode()] = w[:2].upper() + w[2:]
        for k in range(1, len(w)):
            out["lit_trunc_%s_%d" % (w.decode(), k)] = w[:k]
            out["lit_trunc_arr_%s_%d" % (w.decode(), k)] = b"[" + w[:k] + b"]"
        out["lit_ext_%s" % w.decode()] = w + b"x"
        out["lit_ext_arr_%s" % w.decode()] = b"[" + w + b"x]"
        out["lit_glued_%s" % w.decode()] = b"[" + w + w + b"]"
        out["lit_key_%s" % w.decode()] = b'{"' + w + b'":' + w + b"}"
    out["lit_nul_inside"] = b"tr\x00ue"
    out["lit_undefined"] = b"undefined"
    return out


def batches():
    """Many cases per file, so one process start checks many behaviors.

    Seeded random.Random, so the output is still deterministic. A batch that
    diverges is then bisected by hand.
    """
    import random
    rng = random.Random(1719)
    out = {}
    for f in range(40):
        vals = []
        for _ in range(100):
            kind = rng.randrange(7)
            if kind == 0:   # random finite bit pattern
                while True:
                    d = f64(rng.getrandbits(64))
                    if d == d and abs(d) != float("inf"):
                        break
                vals.append(rng.choice(["%r", "%.15g", "%.16g", "%.17g", "%.3g"]) % d)
            elif kind == 1:  # short decimal
                vals.append("%d.%0*d" % (rng.randrange(-1000, 1000), rng.randrange(1, 8),
                                         rng.randrange(0, 10 ** 7)))
            elif kind == 2:  # near int32 / 2^53 / 2^63 boundaries
                base = rng.choice([2 ** 31, -2 ** 31, 2 ** 53, -2 ** 53, 2 ** 63, 2 ** 32])
                vals.append(str(base + rng.randrange(-4, 5)))
            elif kind == 3:  # exact 17-digit ties: odd m / 4 with m in [4e15, 9e15)
                m = rng.randrange(4 * 10 ** 15, 9 * 10 ** 15) | 1
                vals.append(repr(m / 4))
            elif kind == 4:  # random exponent spellings
                vals.append("%d%s%s%d" % (rng.randrange(0, 100000), rng.choice("eE"),
                                          rng.choice(["", "+", "-"]), rng.randrange(0, 330)))
            elif kind == 5:  # subnormal range
                vals.append("%.*e" % (rng.randrange(0, 20), f64(rng.getrandbits(52))))
            else:            # integer-valued doubles of all sizes
                vals.append("%d" % rng.randrange(-10 ** rng.randrange(1, 25), 10 ** rng.randrange(1, 25)))
        out["batch_num_%02d" % f] = ("[" + ",".join(vals) + "]").encode()
    pieces = [b'\\"', b"\\\\", b"\\/", b"\\b", b"\\f", b"\\n", b"\\r", b"\\t"]
    for f in range(40):
        strs = []
        for _ in range(50):
            s = b""
            for _ in range(rng.randrange(0, 12)):
                kind = rng.randrange(6)
                if kind == 0:
                    s += rng.choice(pieces)
                elif kind == 1:  # BMP escape, no surrogates
                    cp = rng.choice([rng.randrange(0, 0x80), rng.randrange(0x80, 0x800),
                                     rng.randrange(0x800, 0xD800), rng.randrange(0xE000, 0x10000)])
                    s += (b"\\u%04x" if rng.random() < 0.5 else b"\\u%04X") % cp
                elif kind == 2:  # valid surrogate pair
                    cp = rng.randrange(0x10000, 0x110000) - 0x10000
                    s += b"\\u%04x\\u%04x" % (0xD800 + (cp >> 10), 0xDC00 + (cp & 0x3FF))
                elif kind == 3:  # raw high byte
                    s += bytes([rng.randrange(0x80, 0x100)])
                else:            # printable ASCII
                    c = rng.randrange(0x20, 0x7F)
                    s += bytes([c]) if c not in (0x22, 0x5C) else b"x"
            strs.append(b'"' + s + b'"')
        if f % 2:
            body = b"{" + b",".join(b"%s:%s" % (strs[i], strs[i + 1]) for i in range(0, 50, 2)) + b"}"
        else:
            body = b"[" + b",".join(strs) + b"]"
        out["batch_str_%02d" % f] = body
    # one escape per file with random hex, surrogates allowed, so errors are hit
    for f in range(60):
        hi = rng.choice([0xD800, 0xDBFF, 0xDC00, 0xDFFF, rng.randrange(0xD800, 0xE000)])
        lo = rng.choice([0xDC00, 0xDFFF, 0xD800, rng.randrange(0, 0x10000), rng.randrange(0xDC00, 0xE000)])
        sep = rng.choice([b"", b"", b"a", b"\\n", b"\\"])
        out["rand_surrogate_%02d" % f] = b'["p",' + b'"\\u%04x%s\\u%04x"]' % (hi, sep, lo)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=DEFAULT_OUT)
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)
    for name in os.listdir(args.out):
        if name.endswith(".bin"):
            os.remove(os.path.join(args.out, name))
    cases = {}
    for group in (numbers, strings, structure, literals, batches):
        for name, data in group().items():
            assert name not in cases, name
            assert name.lower() not in {k.lower() for k in cases}, "case clash: " + name
            cases[name] = data
    for name in sorted(cases):
        with open(os.path.join(args.out, name + ".bin"), "wb") as fh:
            fh.write(cases[name])
    total = sum(len(v) for v in cases.values())
    print("wrote %d inputs (%d bytes) to %s" % (len(cases), total, args.out))


if __name__ == "__main__":
    main()
