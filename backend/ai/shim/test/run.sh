#!/bin/bash
# Check the built shim: every contract symbol exported, then the C smoke test.
# Usage: backend/ai/shim/test/run.sh   (after build.sh; THESEUS_ROOT as for build.sh)
set -eu
T=$(cd "$(dirname "$0")" && pwd -P)
ROOT=$(cd "$T/../../../.." && pwd -P)
LIBDIR="${THESEUS_ROOT:-$ROOT}/runs/shim"
INC="${THESEUS_ROOT:-$ROOT}/backend/harness/expat/vendor/libexpat/expat/lib"
missing=0
exported=$(nm -gU "$LIBDIR/libtheseus_expat.dylib" | awk '{print $3}' | sed 's/^_//')
while read -r s; do
  echo "$exported" | grep -qx "$s" || { echo "MISSING $s"; missing=$((missing + 1)); }
done < "$T/symbols.txt"
echo "symbols: $(wc -l < "$T/symbols.txt" | tr -d ' ') required, $missing missing"
cc -std=c99 -Wall -Wno-deprecated-declarations -I"$INC" "$T/smoke.c" -L"$LIBDIR" -ltheseus_expat -Wl,-rpath,"$LIBDIR" -o "$LIBDIR/smoke"
"$LIBDIR/smoke"
[ "$missing" = 0 ]
