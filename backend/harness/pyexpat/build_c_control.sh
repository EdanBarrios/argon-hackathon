#!/usr/bin/env bash
# Control: the vendored C libexpat 2.9.0 as an expat-ABI dylib, so the same build +
# test path can be checked with a known-good library. -> <out-dir>/libexpat_c_control.dylib
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
OUT="${1:?usage: build_c_control.sh <out-dir>}"; mkdir -p "$OUT"
EXP="${EXPAT_DIR:-}"
for d in "$HERE/../expat" /Users/edan/dev/argon-hackathon/backend/harness/expat; do
  [ -z "$EXP" ] && [ -s "$d/vendor/libexpat/expat/lib/expat.h" ] && EXP="$d"
done
LIB="$EXP/vendor/libexpat/expat/lib"
clang -dynamiclib -O2 -DHAVE_EXPAT_CONFIG_H -I"$EXP/config" -I"$LIB" \
  -install_name @rpath/libexpat_c_control.dylib \
  "$LIB/xmlparse.c" "$LIB/xmltok.c" "$LIB/xmlrole.c" "$LIB/xcs.c" "$LIB/random_dev_urandom.c" "$LIB/random_arc4random_buf.c" -o "$OUT/libexpat_c_control.dylib"
echo "$OUT/libexpat_c_control.dylib"
