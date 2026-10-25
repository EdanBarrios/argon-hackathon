#!/bin/bash
# Checks the banned-API guard in scripts/build_rust.sh.
# Every cheating fixture must be rejected with a "banned API" reason; the stub
# and every committed real model output under results/ must still build.
# Usage: scripts/test_build_guard.sh. Bash 3.2 compatible.
set -u
ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
FAILED=0

for f in "$ROOT"/backend/harness/rust_crate/fixtures/cheat_*.rs; do
  OUT=$("$ROOT/scripts/build_rust.sh" "$f" "guardtest-$(basename "$f" .rs)")
  case "$OUT" in
    *'"ok":false'*'banned API'*) echo "PASS rejected $(basename "$f")" ;;
    *) echo "FAIL not rejected: $f: $OUT"; FAILED=1 ;;
  esac
done

for f in "$ROOT/backend/harness/rust_crate/src/cjson.rs" "$ROOT"/results/*/*/cjson.rs; do
  [ -f "$f" ] || continue
  NAME=$(echo "$f" | sed "s|^$ROOT/||; s|[^A-Za-z0-9._-]|_|g")
  OUT=$("$ROOT/scripts/build_rust.sh" "$f" "guardtest-$NAME")
  case "$OUT" in
    *'"ok":true'*) echo "PASS built ${f#$ROOT/}" ;;
    *) echo "FAIL rejected or broke: $f: $OUT"; FAILED=1 ;;
  esac
done

[ "$FAILED" -eq 0 ] && echo "build guard: all checks passed"
exit "$FAILED"
