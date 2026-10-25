"""Generate differential-test inputs for the cJSON parse+print module.

Stdlib only, deterministic for a given --seed. Writes one file per behavior:
  seed_*   cJSON's own tests/inputs (no .expected files) and fuzzing/inputs
  edge_*   hand-written edge cases (numbers, strings, nesting, syntax)
  mut_*    mutations of the above until --count files exist

Usage: python3 -I gen_inputs.py [--seed 0] [--count 2000] [--out DIR] [--cjson DIR]
"""

import argparse
import hashlib
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PREFIXES = ("seed_", "edge_", "mut_")

INTERESTING_NUMBERS = [
    b"0", b"-0", b"1e999", b"-1e999", b"1e-400", b"123456789012345678901234567890",
    b"0.1", b"1.5e300", b"9007199254740993", b"9007199254740992", b"2147483647",
    b"2147483648", b"-2147483648", b"-2147483649", b"1E2", b"1e+2", b"1.0", b"-0.0",
    b"0.30000000000000004", b"5e-324", b"1.7976931348623157e308", b"4.9e-324",
    b"01", b"1.", b".5", b"+1", b"0x10", b"NaN", b"Infinity", b"-", b"1e", b"1e+",
]
INTERESTING_STRINGS = [
    b'"\\""', b'"\\\\"', b'"\\/"', b'"\\b\\f\\n\\r\\t"', b'"\\u0041"', b'"\\u0000"',
    b'"a\\u0000b"', b'"\\ud83d\\ude00"', b'"\\ud800"', b'"\\udc00"', b'"\\ud800x"',
    b'"\\ud800\\u0041"', b'"\\uDBFF\\uDFFF"', b'"\\u00e9"', b'"\\u20ac"', b'"\\x"',
    b'"\\u12"', b'"\\uZZZZ"', b'"\xc3\xa9"', b'"\xff"', b'"\xc3"', b'"\xed\xa0\x80"',
    b'"\xf0\x9f\x98\x80"', b'"\x01"', b'"\x1f"', b'"\x7f"', b'"\t"', b'"\n"', b'""',
    b'"unterminated',
]
TOKENS = [b"{", b"}", b"[", b"]", b",", b":", b'"', b"\\", b"null", b"true", b"false",
          b"nul", b"tru", b" ", b"\n", b"\x00", b"\xef\xbb\xbf", b"\\u", b"-", b"e", b"."]


def edge_cases():
    """Hand-written edge cases. Returns a list of (name, bytes)."""
    cases = []
    for i, n in enumerate(INTERESTING_NUMBERS):
        cases.append(("num_%02d" % i, b"[" + n + b"]"))
        cases.append(("num_bare_%02d" % i, n))
    for i, s in enumerate(INTERESTING_STRINGS):
        cases.append(("str_%02d" % i, s))
        cases.append(("str_key_%02d" % i, b"{" + s + b":1}"))
    for i in range(0x20):
        cases.append(("ctrl_%02x" % i, b'"a' + bytes([i]) + b'b"'))
    for depth in (999, 1000, 1001, 100000):
        cases.append(("depth_arr_%d" % depth, b"[" * depth + b"]" * depth))
        cases.append(("depth_obj_%d" % depth,
                      b'{"a":' * (depth - 1) + b"{}" + b"}" * (depth - 1)))
        cases.append(("depth_unclosed_%d" % depth, b"[" * depth))
    syntax = {
        "trailing_comma_arr": b"[1,2,]",
        "trailing_comma_obj": b'{"a":1,}',
        "leading_comma": b"[,1]",
        "double_comma": b"[1,,2]",
        "dup_keys": b'{"a":1,"a":2}',
        "dup_keys_nested": b'{"a":{"b":1},"a":[2]}',
        "trailing_garbage": b"[1] x",
        "trailing_garbage_obj": b'{"a":1}}',
        "two_values": b"1 2",
        "trailing_ws": b"[1]  \n\t\r ",
        "leading_ws": b" \n\t\r[1]",
        "empty": b"",
        "ws_only": b" \n\t\r ",
        "bom_value": b"\xef\xbb\xbf[1]",
        "bom_only": b"\xef\xbb\xbf",
        "nul_after_value": b"[1]\x00garbage",
        "nul_only": b"\x00",
        "single_quotes": b"['a']",
        "unquoted_key": b"{a:1}",
        "comment": b"[1 /* c */]",
        "bare_true": b"true", "bare_false": b"false", "bare_null": b"null",
        "bare_string": b'"x"', "bare_number": b"42",
        "empty_arr": b"[]", "empty_obj": b"{}", "nested_empty": b"[[],{},[{}]]",
        "key_not_string": b"{1:2}", "missing_colon": b'{"a" 1}', "missing_value": b'{"a":}',
        "unclosed_arr": b"[1,2", "unclosed_obj": b'{"a":1', "unclosed_str": b'["abc]',
        "capital_true": b"True", "long_string": b'"' + b"a" * 5000 + b'"',
        "wide_array": b"[" + b",".join(b"%d" % i for i in range(3000)) + b"]",
        "many_keys": b"{" + b",".join(b'"k%d":%d' % (i, i) for i in range(1000)) + b"}",
        "float_array": b"[0.1,0.2,0.30000000000000004,1e21,1e-7,123.456e-789]",
        "mixed": b'{"a":[1,-2.5e3,true,false,null,"s\\n"],"b":{"c":{}}}',
    }
    for name in sorted(syntax):
        cases.append(("syn_" + name, syntax[name]))
    return cases


def load_seed_files(cjson_dir):
    seeds = []
    for sub in ("tests/inputs", "fuzzing/inputs"):
        d = os.path.join(cjson_dir, sub)
        if not os.path.isdir(d):
            sys.exit("missing %s; run fetch_cjson.sh first" % d)
        tag = sub.split("/")[0]
        for name in sorted(os.listdir(d)):
            if not name.startswith("test") or name.endswith(".expected"):
                continue
            with open(os.path.join(d, name), "rb") as fh:
                data = fh.read()
            seeds.append(("%s_%s" % (tag, name.replace(".", "_")), data))
            # fuzzing inputs start with option flag bytes; also keep the JSON part
            if tag == "fuzzing":
                starts = [i for i in (data.find(b"{"), data.find(b"[")) if i > 0]
                if starts:
                    seeds.append(("%s_%s_body" % (tag, name.replace(".", "_")),
                                  data[min(starts):]))
    return seeds


def mutate(rng, data, pool):
    """Apply one random structured or byte-level mutation."""
    op = rng.randrange(11)
    n = len(data)
    pos = rng.randint(0, n)
    if op == 0 and n:                        # flip one byte
        i = rng.randrange(n)
        return data[:i] + bytes([data[i] ^ (1 << rng.randrange(8))]) + data[i + 1:]
    if op == 1:                              # insert a JSON-ish token
        return data[:pos] + rng.choice(TOKENS) + data[pos:]
    if op == 2 and n:                        # delete a span
        end = min(n, pos + rng.randint(1, 8))
        return data[:pos] + data[end:]
    if op == 3 and n:                        # duplicate a span
        a = rng.randrange(n)
        b = min(n, a + rng.randint(1, 16))
        return data[:pos] + data[a:b] + data[pos:]
    if op == 4:                              # splice two inputs
        other = rng.choice(pool)
        cut = rng.randint(0, len(other))
        return data[:pos] + other[cut:]
    if op == 5:                              # swap a number for an interesting one
        digits = [i for i, ch in enumerate(data) if 0x30 <= ch <= 0x39]
        if digits:
            i = rng.choice(digits)
            j = i
            while j < n and data[j:j + 1] in b"0123456789.eE+-":
                j += 1
            return data[:i] + rng.choice(INTERESTING_NUMBERS) + data[j:]
        return b"[" + rng.choice(INTERESTING_NUMBERS) + b"]"
    if op == 6:                              # drop in an interesting string
        return data[:pos] + rng.choice(INTERESTING_STRINGS) + data[pos:]
    if op == 7 and n:                        # truncate
        return data[:rng.randrange(n)]
    if op == 8:                              # wrap
        return rng.choice([b"[%s]", b'{"k":%s}', b"[%s,%s]", b" %s "]).replace(b"%s", data)
    if op == 9 and n:                        # random byte overwrite
        i = rng.randrange(n)
        return data[:i] + bytes([rng.randrange(256)]) + data[i + 1:]
    # nested array of a random small depth around a value
    depth = rng.randint(1, 40)
    return b"[" * depth + data + b"]" * depth


def main(argv=None):
    ap = argparse.ArgumentParser(description="Generate cJSON differential inputs")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--count", type=int, default=2000, help="total files to write")
    ap.add_argument("--out", default=os.path.join(HERE, "inputs", "cjson"))
    ap.add_argument("--cjson", default=os.path.join(HERE, "vendor", "cJSON"))
    ap.add_argument("--max-input-bytes", type=int, default=262144,
                    help="drop mutations larger than this")
    args = ap.parse_args(argv)

    rng = random.Random(args.seed)
    os.makedirs(args.out, exist_ok=True)
    for name in os.listdir(args.out):  # clear only files we generated earlier
        if name.startswith(PREFIXES):
            os.remove(os.path.join(args.out, name))

    seen = set()
    written = []

    def emit(name, data):
        digest = hashlib.sha256(data).digest()
        if digest in seen:
            return False
        seen.add(digest)
        with open(os.path.join(args.out, name), "wb") as fh:
            fh.write(data)
        written.append(data)
        return True

    for name, data in load_seed_files(args.cjson):
        emit("seed_%s" % name, data)
    for name, data in edge_cases():
        emit("edge_%s" % name, data)

    base = list(written)
    # mutation pool excludes the huge nesting cases to keep runs fast
    pool = [d for d in base if len(d) <= 20000] or base
    i, attempts = 0, 0
    while len(written) < args.count and attempts < args.count * 50:
        attempts += 1
        data = rng.choice(pool)
        for _ in range(rng.randint(1, 3)):
            data = mutate(rng, data, pool)
        if len(data) > args.max_input_bytes:
            continue
        if emit("mut_%05d" % i, data):
            i += 1
    print("wrote %d inputs to %s (%d seeds/edge cases, %d mutations, seed %d)"
          % (len(written), args.out, len(base), i, args.seed))
    return 0


if __name__ == "__main__":
    sys.exit(main())
