#!/usr/bin/env bash
# Build the C stub shim: build_stub.sh <out-dir>  ->  <out-dir>/libtheseus_expat_stub.dylib
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
OUT="${1:?usage: build_stub.sh <out-dir>}"; mkdir -p "$OUT"
EXPAT_INC="${EXPAT_INC:-}"
if [ -z "$EXPAT_INC" ]; then
  for d in "$HERE/../expat/vendor/libexpat/expat/lib"; do
    [ -s "$d/expat.h" ] && { EXPAT_INC="$d"; break; }
  done
fi
clang -dynamiclib -O1 -Wall -I"$EXPAT_INC" -DXML_DTD -DXML_NS -DXML_GE=1 \
  -install_name @rpath/libtheseus_expat_stub.dylib \
  "$HERE/stub_shim.c" -o "$OUT/libtheseus_expat_stub.dylib"
echo "$OUT/libtheseus_expat_stub.dylib"
