#!/bin/bash
# Builds two C drivers from the same sources/config as backend/harness/expat/build_expat.sh:
#   expat_driver_edge: -O1 -fsanitize-coverage=trace-pc-guard (edge bitmap to $EDGE_OUT), used in the fuzz loop
#   expat_driver_cov:  -O0 source-based coverage, same flags as results/libexpat-coverage/scripts/build_cov.sh
# Usage: build_fuzz.sh OUTDIR
set -eu
OUT=$1; mkdir -p "$OUT"
HERE=$(cd "$(dirname "$0")" && pwd -P)
REPO=$(cd "$HERE/../../.." && pwd -P)
H=$REPO/backend/harness/expat
V=${EXPAT_LIB:-/Users/edan/dev/argon-hackathon/backend/harness/expat/vendor/libexpat/expat/lib}
SRCS="xmlparse.c xmlrole.c xmltok.c xcs.c random_dev_urandom.c random_arc4random_buf.c"
build() { # name flags...
  local name=$1; shift; local d="$OUT/$name.o"; mkdir -p "$d"
  for s in $SRCS; do clang "$@" -std=c99 -I"$H/config" -I"$V" -c "$V/$s" -o "$d/${s%.c}.o"; done
  clang "$@" -std=c99 -I"$H/config" -I"$V" -c "$H/drivers/expat_driver.c" -o "$d/expat_driver.o"
}
build edge -g -O1 -fsanitize-coverage=trace-pc-guard
clang -O1 -c "$HERE/edge_rt.c" -o "$OUT/edge_rt.o"
clang "$OUT"/edge.o/*.o "$OUT/edge_rt.o" -o "$OUT/expat_driver_edge"
build cov -g -O0 -fprofile-instr-generate -fcoverage-mapping
clang -fprofile-instr-generate -fcoverage-mapping "$OUT"/cov.o/*.o -o "$OUT/expat_driver_cov"
echo built "$OUT/expat_driver_edge" "$OUT/expat_driver_cov"
