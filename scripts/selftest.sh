#!/bin/bash
# Verifier self-test: run the cJSON C driver against ITSELF.
# Fetches cJSON, builds the ASan/UBSan driver, generates inputs, runs
# verify.sh with C on both sides, and exits nonzero unless match_rate is 1.0
# with zero divergences. If this fails, the harness is broken, not the code
# under test. Bash 3.2 compatible.
#
# Usage: scripts/selftest.sh [--seed N] [--count N]
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
H="$ROOT/backend/harness"
SEED=0
COUNT=2000
while [ $# -gt 0 ]; do
  case "$1" in
    --seed) SEED="$2"; shift 2 ;;
    --count) COUNT="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

"$H/fetch_cjson.sh"
"$H/build_cjson.sh" 2>/dev/null
python3 -I "$H/gen_inputs.py" --seed "$SEED" --count "$COUNT"

DRIVER="$H/build/cjson_driver"

# Canary: C against C matches trivially even if the driver rejects every
# input (this happened: a wrong buffer length made everything PARSE_ERROR).
# So first prove the driver really parses and prints.
GOT=$(printf '{"a": [1, 2.5, "x"], "b": null}' | "$DRIVER")
WANT=$(printf 'OK\t{"a":[1,2.5,"x"],"b":null}')
if [ "$GOT" != "$WANT" ]; then
  echo "SELFTEST FAILED: driver canary printed '$GOT'" >&2
  exit 1
fi

STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/runs/selftest-$STAMP.json"
"$H/verify.sh" --c "$DRIVER" --rust "$DRIVER" --inputs "$H/inputs/cjson" --out "$OUT" >/dev/null

python3 -I - "$OUT" <<'PY'
import json, sys
r = json.load(open(sys.argv[1]))
c = r["counts"]
print("match_rate=%s behaviors_tested=%d counts=%s" % (r["match_rate"], r["behaviors_tested"], c))
print("report: %s" % sys.argv[1])
ok = r["match_rate"] == 1.0 and c["divergence"] == 0 and r["behaviors_tested"] > 0
if not ok:
    print("SELFTEST FAILED: C vs C must be a perfect match", file=sys.stderr)
sys.exit(0 if ok else 1)
PY
