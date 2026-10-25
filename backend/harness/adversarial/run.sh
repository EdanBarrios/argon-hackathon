#!/usr/bin/env bash
# Run the adversarial corpora (C driver vs one Rust binary) through verify.sh.
#
# Usage: backend/harness/adversarial/run.sh <rust_binary> <label> [corpus ...]
#   corpus defaults to: targeted argon human
# Reports go to runs/adversarial/<label>-<corpus>.json.
#
# verify.py feeds every non-dot file in a directory to both sides, so each
# corpus is staged into a temp dir first: corpus/argon has REASONS.md and
# corpus/human has a README, and those must not be judged as inputs.
# Human files may have any name; everything except README* is staged.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
RUST="${1:?rust binary}"
LABEL="${2:?label}"
shift 2
CORPORA=("$@")
[ ${#CORPORA[@]} -eq 0 ] && CORPORA=(targeted argon human)
mkdir -p "$ROOT/runs/adversarial"
for corpus in "${CORPORA[@]}"; do
  src="$HERE/corpus/$corpus"
  stage="$(mktemp -d)"
  find "$src" -maxdepth 1 -type f ! -name 'README*' ! -name 'REASONS.md' ! -name '.*' \
    -exec cp {} "$stage/" \;
  n=$(find "$stage" -type f | wc -l | tr -d ' ')
  if [ "$n" -eq 0 ]; then
    echo "$corpus: no inputs, skipped"
    rm -rf "$stage"
    continue
  fi
  out="$ROOT/runs/adversarial/$LABEL-$corpus.json"
  "$ROOT/backend/harness/verify.sh" --c "$ROOT/backend/harness/build/cjson_driver" \
    --rust "$RUST" --inputs "$stage" --timeout 20 --out "$out" >/dev/null || true
  python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); print(sys.argv[2], r['behaviors_tested'], r['counts'], r['match_rate'])" "$out" "$corpus"
  rm -rf "$stage"
done
