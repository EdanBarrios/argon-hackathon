#!/bin/bash
# C coverage of cJSON.c reached by a directory of inputs. Reported only,
# never a pass/fail gate: it shows which parts of the parse/print path the
# generated inputs exercise. macOS (xcrun llvm-profdata / llvm-cov).
# Bash 3.2 compatible.
#
# Builds an instrumented driver (no sanitizers) into
# backend/harness/build/coverage, runs every input file through it, merges
# the profiles and prints one JSON line: line/branch/function/region coverage
# of cJSON.c plus per-function region and branch coverage for the
# parse/print functions.
#
# Usage: scripts/coverage.sh [--inputs DIR]   (default backend/harness/inputs/cjson)
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
H="$ROOT/backend/harness"
INPUTS="$H/inputs/cjson"
while [ $# -gt 0 ]; do
  case "$1" in
    --inputs) INPUTS="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
if [ ! -d "$INPUTS" ]; then
  echo "ERROR: inputs directory $INPUTS missing; run gen_inputs.py first" >&2
  exit 1
fi

"$H/fetch_cjson.sh" >&2
SRC="$H/vendor/cJSON"
OUT="$H/build/coverage"
CC="${CC:-clang}"
FLAGS="-g -O0 -fprofile-instr-generate -fcoverage-mapping"
mkdir -p "$OUT"
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c89 -I"$SRC" -c "$SRC/cJSON.c" -o "$OUT/cJSON.o"
# shellcheck disable=SC2086
"$CC" $FLAGS -I"$SRC" -c "$H/drivers/cjson_driver.c" -o "$OUT/cjson_driver.o"
# shellcheck disable=SC2086
"$CC" $FLAGS "$OUT/cJSON.o" "$OUT/cjson_driver.o" -o "$OUT/cjson_driver"

PROF=$(mktemp -d "${TMPDIR:-/tmp}/cjson-cov.XXXXXX")
trap 'rm -rf "$PROF"' EXIT

N=0
for f in "$INPUTS"/*; do
  [ -f "$f" ] || continue
  N=$((N + 1))
  # A nonzero exit still writes a profile; coverage is not a verdict.
  LLVM_PROFILE_FILE="$PROF/$N.profraw" LC_ALL=C "$OUT/cjson_driver" < "$f" > /dev/null 2>&1 || true
done
if [ "$N" -eq 0 ]; then
  echo "ERROR: no input files in $INPUTS" >&2
  exit 1
fi

xcrun llvm-profdata merge -sparse "$PROF"/*.profraw -o "$PROF/merged.profdata"
xcrun llvm-cov export -summary-only -instr-profile "$PROF/merged.profdata" \
  "$OUT/cjson_driver" "$SRC/cJSON.c" > "$PROF/summary.json"
xcrun llvm-cov export -instr-profile "$PROF/merged.profdata" \
  "$OUT/cjson_driver" "$SRC/cJSON.c" > "$PROF/full.json"

python3 -I - "$PROF/summary.json" "$PROF/full.json" "$INPUTS" "$N" <<'PY'
import json, os, sys
summary_path, full_path, inputs, runs = sys.argv[1:5]

def pick(s):
    return {"covered": s["covered"], "count": s["count"], "percent": round(s["percent"], 2)}

files = [f for f in json.load(open(summary_path))["data"][0]["files"]
         if f["filename"].endswith("/cJSON.c")]
if len(files) != 1:
    sys.exit("coverage.sh: expected exactly one cJSON.c in the llvm-cov export")
s = files[0]["summary"]

FUNCS = ("parse_value", "parse_number", "parse_string", "parse_array", "parse_object",
         "parse_hex4", "utf16_literal_to_utf8", "skip_utf8_bom", "buffer_skip_whitespace",
         "print_value", "print_number", "print_string_ptr", "print_array", "print_object")
per_function = {}
for fn in json.load(open(full_path))["data"][0]["functions"]:
    name = fn["name"].split(":")[-1]  # static functions are "cJSON.c:name"
    if name not in FUNCS:
        continue
    code = [r for r in fn["regions"] if r[7] == 0]          # kind 0 = code region
    outcomes = [x > 0 for b in fn["branches"] for x in (b[4], b[5])]  # true, false
    per_function[name] = {
        "regions": "%d/%d" % (sum(r[4] > 0 for r in code), len(code)),
        "branches": "%d/%d" % (sum(outcomes), len(outcomes)),
    }

print(json.dumps({
    "file": "cJSON.c",
    "inputs": os.path.abspath(inputs),
    "runs": int(runs),
    "lines": pick(s["lines"]),
    "branches": pick(s["branches"]),
    "functions": pick(s["functions"]),
    "regions": pick(s["regions"]),
    "per_function": per_function,
}, sort_keys=False))
PY
