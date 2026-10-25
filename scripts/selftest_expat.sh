#!/bin/bash
# libexpat harness self-test. Three checks, all reported:
#   1. canary: one known document must print the exact expected event lines
#      (C against C matches trivially even if the driver rejects everything,
#      see docs/FRICTION-LOG.md 10:50)
#   2. C against C over the generated inputs: match_rate must be 1.0
#   3. C against the stub expat.rs (rejects everything): match_rate must be
#      near 0, proving the verifier can tell implementations apart
# Exits nonzero if any check fails. Bash 3.2 compatible.
#
# Usage: scripts/selftest_expat.sh [--seed N] [--count N]
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
H="$ROOT/backend/harness"
E="$H/expat"
SEED=0
COUNT=2000
STUB_MAX=0.05
while [ $# -gt 0 ]; do
  case "$1" in
    --seed) SEED="$2"; shift 2 ;;
    --count) COUNT="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

"$E/fetch_expat.sh" >/dev/null 2>&1
"$E/build_expat.sh" >/dev/null 2>&1
python3 -I "$E/gen_inputs_expat.py" --seed "$SEED" --count "$COUNT"
DRIVER="$E/build/expat_driver"

# 1. Canary
GOT=$(printf '<?xml version="1.0"?>\n<!DOCTYPE d [<!ENTITY e "x&amp;y">]>\n<d b="&lt;" a="1"><!--c--><?p q?>hi &e; <![CDATA[<z>]]>\303\251<e/></d>' | "$DRIVER")
WANT='XMLDECL "1.0" null -1
DOCTYPE_START "d" null null 1
ENTITY "e" 0 VALUE "x&amp;y"
DOCTYPE_END
START "d" "b"="<" "a"="1"
COMMENT "c"
PI "p" "q"
TEXT "hi x&y "
CDATA_START
TEXT "<z>"
CDATA_END
TEXT "\xc3\xa9"
START "e"
END "e"
END "d"
OK'
if [ "$GOT" != "$WANT" ]; then
  echo "SELFTEST FAILED: canary printed:" >&2
  echo "$GOT" >&2
  exit 1
fi
GOT_ERR=$(printf '<a>\n<b></a>' | "$DRIVER")
if [ "$GOT_ERR" != "ERROR 7 line 2 col 5" ]; then
  echo "SELFTEST FAILED: error canary printed '$GOT_ERR'" >&2
  exit 1
fi
echo "canary: ok (event lines and the error line match exactly)"

# 2. C against C
STAMP=$(date +%Y%m%d-%H%M%S)
OUT_CC="$ROOT/runs/selftest-expat-cc-$STAMP.json"
"$H/verify.sh" --c "$DRIVER" --rust "$DRIVER" --inputs "$E/inputs" --timeout 20 --out "$OUT_CC" >/dev/null

# 3. C against the stub
BUILD=$("$ROOT/scripts/build_rust_expat.sh" "$E/rust_crate/src/expat.rs" "selftest-stub-$STAMP")
STUB_BIN=$(python3 -I -c 'import json,sys; d=json.loads(sys.argv[1]); print(d.get("binary","")) if d["ok"] else sys.exit("stub build failed: "+d["errors"])' "$BUILD")
OUT_STUB="$ROOT/runs/selftest-expat-stub-$STAMP.json"
"$H/verify.sh" --c "$DRIVER" --rust "$STUB_BIN" --inputs "$E/inputs" --timeout 20 --out "$OUT_STUB" >/dev/null

python3 -I - "$OUT_CC" "$OUT_STUB" "$STUB_MAX" <<'PY'
import json, sys
cc, stub = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
limit = float(sys.argv[3])
for label, r, path in (("C vs C", cc, sys.argv[1]), ("C vs stub", stub, sys.argv[2])):
    print("%s: match_rate=%s behaviors_tested=%d counts=%s report=%s"
          % (label, r["match_rate"], r["behaviors_tested"], r["counts"], path))
ok = True
if not (cc["match_rate"] == 1.0 and cc["counts"]["divergence"] == 0 and cc["behaviors_tested"] > 0):
    print("SELFTEST FAILED: C vs C must be a perfect match", file=sys.stderr)
    ok = False
if stub["match_rate"] is None or stub["match_rate"] > limit:
    print("SELFTEST FAILED: the stub must score at most %s" % limit, file=sys.stderr)
    ok = False
sys.exit(0 if ok else 1)
PY
