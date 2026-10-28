#!/bin/bash
# Reproduce the headline libexpat result with no API key and no model call.
# Builds libexpat R_2_9_0 (ASan + UBSan) and the committed Argon port, then
# runs both on 2,000 inputs (seed 4242) through verify.sh and prints the counts.
#
# Needs: clang, cargo, git, curl, python3, network access (fetches libexpat and
# the W3C XML conformance suite at pinned versions). Bash 3.2 compatible.
# Usage: scripts/reproduce.sh [--seed N] [--count N]
# Only macOS x86_64 has been tested.
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
H="$ROOT/backend/harness"
E="$H/expat"
PORT="$ROOT/backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs"
PORT_SHA="418570361191d0019294b981d80e3185c7a732e0d4c02a627107f60b5af72c0d"
SEED=4242
COUNT=2000
while [ $# -gt 0 ]; do
  case "$1" in
    --seed) SEED="$2"; shift 2 ;;
    --count) COUNT="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

for tool in clang cargo git curl python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "ERROR: $tool not found" >&2; exit 2; }
done

GOT=$(shasum -a 256 "$PORT" | cut -d' ' -f1)
[ "$GOT" = "$PORT_SHA" ] || { echo "ERROR: $PORT does not match the recorded sha256" >&2; exit 1; }

echo "1/4 fetching libexpat and the W3C suite"
"$E/fetch_expat.sh" >/dev/null
echo "2/4 building the C driver (ASan + UBSan)"
"$E/build_expat.sh" >/dev/null
echo "3/4 building the Rust port (unsafe and banned-API guard included)"
BUILD=$("$ROOT/scripts/build_rust_expat.sh" "$PORT" reproduce)
BIN=$(printf '%s' "$BUILD" | python3 -I -c 'import json,sys; d=json.load(sys.stdin); print(d["binary"] if d["ok"] else "")')
[ -n "$BIN" ] || { echo "ERROR: Rust build failed: $BUILD" >&2; exit 1; }

INPUTS="$ROOT/runs/reproduce-inputs-s$SEED"
rm -rf "$INPUTS"
python3 -I "$E/gen_inputs_expat.py" --seed "$SEED" --count "$COUNT" --out "$INPUTS" >/dev/null
echo "4/4 verifying $COUNT inputs (seed $SEED); this takes several minutes"
OUT="$ROOT/runs/reproduce-verify-s$SEED.json"
"$H/verify.sh" --c "$E/build/expat_driver" --rust "$BIN" --inputs "$INPUTS" \
  --timeout 20 --jobs "$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)" > "$OUT"

python3 -I - "$OUT" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
c = d["counts"]
print(f"match_rate {d['match_rate']}  behaviors_tested {d['behaviors_tested']}")
print("  " + ", ".join(f"{k} {v}" for k, v in c.items()))
sys.exit(0 if d["match_rate"] == 1.0 else 1)
PY
