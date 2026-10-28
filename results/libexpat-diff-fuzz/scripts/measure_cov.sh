#!/bin/bash
# Source-based line/branch coverage of the five parser files, same method as
# results/libexpat-coverage (clang -O0 profile build, one profile per input,
# llvm-profdata merge -sparse, llvm-cov report).
# Usage: measure_cov.sh COV_BIN WORKDIR INPUT_DIR [INPUT_DIR...]
set -u
BIN=$1; W=$2; shift 2
L=${EXPAT_LIB:-/Users/edan/dev/argon-hackathon/backend/harness/expat/vendor/libexpat/expat/lib}
rm -rf "$W/prof"; mkdir -p "$W/prof"
n=0
for d in "$@"; do
  for f in $(find "$d" -type f); do
    LLVM_PROFILE_FILE="$W/prof/%p.profraw" "$BIN" < "$f" > /dev/null 2>&1
    n=$((n+1))
  done
done
echo "ran $n inputs"
xcrun llvm-profdata merge -sparse "$W"/prof/*.profraw -o "$W/merged.profdata"
xcrun llvm-cov report "$BIN" -instr-profile="$W/merged.profdata" \
  "$L/xmlparse.c" "$L/xmltok.c" "$L/xmltok_impl.c" "$L/xmltok_ns.c" "$L/xmlrole.c" > "$W/report.txt"
cat "$W/report.txt"
