"""C mutation sensitivity check: does the verifier catch a real bug?

Stdlib only. Run with `python3 -I mutants.py --inputs DIR`.

The C-vs-C self-test proves the verifier reports no false divergences. This
proves the other direction. For each mutation below we copy the vendored
cJSON.c, apply ONE exact-string substitution at a known site in the parse or
print path of cJSON v1.7.19, build a driver from it with build_cjson.sh (same
driver source and flags as the real one), and run verify.sh with the
unmutated driver as the C side and the mutant as the other side.

  KILLED    at least one input did not match (divergence, crash or timeout)
  SURVIVED  every input matched: the generated inputs never exercise the
            mutated behaviour. Survivors point at generator blind spots.

Prints one JSON report on stdout, lists survivors on stderr, and exits 1 if
any mutant survived (2 for a usage or setup error).
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
CJSON_C = os.path.join(HERE, "vendor", "cJSON", "cJSON.c")

# (id, description, old text, new text). Each old text must occur exactly
# once in cJSON.c v1.7.19, so every mutant changes exactly one site.
MUTATIONS = [
    ("array_depth_off_by_one", "parse_array: nesting check >= becomes > (allows depth 1001)",
     "/* head of the linked list */\n    cJSON *current_item = NULL;\n\n"
     "    if (input_buffer->depth >= CJSON_NESTING_LIMIT)",
     "/* head of the linked list */\n    cJSON *current_item = NULL;\n\n"
     "    if (input_buffer->depth > CJSON_NESTING_LIMIT)"),
    ("object_depth_limit_lowered", "parse_object: nesting limit lowered by one (999)",
     "/* linked list head */\n    cJSON *current_item = NULL;\n\n"
     "    if (input_buffer->depth >= CJSON_NESTING_LIMIT)",
     "/* linked list head */\n    cJSON *current_item = NULL;\n\n"
     "    if (input_buffer->depth >= CJSON_NESTING_LIMIT - 1)"),
    ("print_precision_15_to_14", "print_number: first attempt %1.15g becomes %1.14g",
     '"%1.15g"', '"%1.14g"'),
    ("print_precision_17_to_16", "print_number: round-trip fallback %1.17g becomes %1.16g",
     '"%1.17g"', '"%1.16g"'),
    ("print_inf_not_null", "print_number: infinity no longer printed as null",
     "isnan(d) || isinf(d)", "isnan(d)"),
    ("print_escape_f_upper", "print_string_ptr: \\f printed as \\F",
     "*output_pointer = 'f';", "*output_pointer = 'F';"),
    ("print_control_hex_upper", "print_string_ptr: control chars printed as \\u%04X",
     '"u%04x"', '"u%04X"'),
    ("whitespace_excludes_space", "skip_whitespace: <= 32 becomes < 32 (space not whitespace)",
     "[0] <= 32", "[0] < 32"),
    ("bom_bound_off_by_one", "skip_utf8_bom: needs index 3 instead of 4 to be readable",
     "can_access_at_index(buffer, 4)", "can_access_at_index(buffer, 3)"),
    ("lone_low_surrogate_range", "utf16_literal_to_utf8: lone low surrogate check stops at DFFE",
     "(first_code <= 0xDFFF)", "(first_code <= 0xDFFE)"),
    ("second_surrogate_range", "utf16_literal_to_utf8: second surrogate DFFF rejected",
     "(second_code > 0xDFFF)", "(second_code >= 0xDFFF)"),
    ("utf8_two_byte_bound", "utf16_literal_to_utf8: U+0800 encoded with 2 bytes",
     "codepoint < 0x800", "codepoint <= 0x800"),
    ("hex_lower_f_rejected", "parse_hex4: lowercase 'f' no longer a hex digit",
     "(input[i] <= 'f')", "(input[i] <= 'e')"),
    ("number_drops_upper_e", "parse_number: 'E' exponent not part of a number",
     "            case 'e':\n            case 'E':\n", "            case 'e':\n"),
    ("parse_escape_drops_slash", "parse_string: \\/ escape rejected",
     "                case '\\\\':\n                case '/':\n",
     "                case '\\\\':\n"),
]


def fail(message):
    print("mutants.py: " + message, file=sys.stderr)
    sys.exit(2)


def check_sites(source):
    """Every mutation must match exactly once; name the ones that do not."""
    bad = ["%s (matches %d times)" % (mid, source.count(old))
           for mid, _desc, old, _new in MUTATIONS if source.count(old) != 1]
    if bad:
        fail("mutation sites do not match cJSON.c exactly once: " + ", ".join(bad))


def build(target, cjson_c):
    env = dict(os.environ, CJSON_C=cjson_c)
    proc = subprocess.run([os.path.join(HERE, "build_cjson.sh"), target], env=env,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        fail("build failed for %s:\n%s" % (target, proc.stderr.decode("utf-8", "replace")))


def verify(c_driver, other_driver, args):
    cmd = [os.path.join(HERE, "verify.sh"), "--c", c_driver, "--rust", other_driver,
           "--inputs", args.inputs, "--timeout", str(args.timeout)]
    if args.jobs:
        cmd += ["--jobs", str(args.jobs)]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, check=True)
    return json.loads(proc.stdout)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--inputs", required=True, help="directory of input files")
    ap.add_argument("--timeout", type=float, default=5.0, help="seconds per run")
    ap.add_argument("--jobs", type=int, help="passed to verify.sh")
    args = ap.parse_args(argv)
    if not os.path.isdir(args.inputs):
        ap.error("--inputs is not a directory: %s" % args.inputs)
    if not os.path.isfile(CJSON_C):
        ap.error("%s missing; run fetch_cjson.sh first" % CJSON_C)
    args.inputs = os.path.abspath(args.inputs)

    with open(CJSON_C, encoding="utf-8") as fh:
        source = fh.read()
    check_sites(source)

    work = tempfile.mkdtemp(prefix="cjson-mutants-")
    try:
        original = os.path.join(work, "original", "cjson_driver")
        build(original, CJSON_C)
        results = []
        for mid, desc, old, new in MUTATIONS:
            mdir = os.path.join(work, mid)
            os.makedirs(mdir)
            mutant_c = os.path.join(mdir, "cJSON.c")
            with open(mutant_c, "w", encoding="utf-8") as fh:
                fh.write(source.replace(old, new, 1))
            driver = os.path.join(mdir, "cjson_driver")
            build(driver, mutant_c)
            report = verify(original, driver, args)
            killed = report["counts"]["match"] != report["behaviors_tested"]
            first = [e["file"] for k in ("divergences", "crashes", "timeouts")
                     for e in report[k]][:3]
            results.append({
                "id": mid, "description": desc,
                "verdict": "KILLED" if killed else "SURVIVED",
                "counts": {k: v for k, v in report["counts"].items() if v},
                "example_inputs": first,
            })
            print("%-28s %s" % (mid, results[-1]["verdict"]), file=sys.stderr)
    finally:
        shutil.rmtree(work, ignore_errors=True)

    survived = [{"id": r["id"], "description": r["description"]}
                for r in results if r["verdict"] == "SURVIVED"]
    killed = len(results) - len(survived)
    out = {
        "mutants": len(results),
        "killed": killed,
        "survived": survived,
        "kill_rate": killed / len(results),
        "inputs": args.inputs,
        "behaviors_tested": report["behaviors_tested"],
        "results": results,
    }
    sys.stdout.write(json.dumps(out, indent=2) + "\n")
    for s in survived:
        print("SURVIVED: %s: %s" % (s["id"], s["description"]), file=sys.stderr)
    return 1 if survived else 0


if __name__ == "__main__":
    sys.exit(main())
