#!/bin/bash
# In-scope line estimate, reusing results/libexpat-coverage/scripts/{funcs,scope}.py unchanged.
# Usage: in_scope.sh COV_BIN WORKDIR   (WORKDIR has merged.profdata from measure_cov.sh)
set -eu
BIN=$1; W=$2
HERE=$(cd "$(dirname "$0")" && pwd -P)
CS=$HERE/../../libexpat-coverage/scripts
L=${EXPAT_LIB:-/Users/edan/dev/argon-hackathon/backend/harness/expat/vendor/libexpat/expat/lib}
FILES="$L/xmlparse.c $L/xmltok.c $L/xmltok_impl.c $L/xmltok_ns.c $L/xmlrole.c"
xcrun llvm-cov export "$BIN" -instr-profile="$W/merged.profdata" $FILES > "$W/full_export.json"
xcrun llvm-cov export -summary-only "$BIN" -instr-profile="$W/merged.profdata" $FILES > "$W/summary_export.json"
python3 -I "$CS/funcs.py" "$W/full_export.json" "$W/funcs.json" > /dev/null
for f in xmlparse.c xmltok.c xmltok_impl.c xmlrole.c xmltok_ns.c; do
  xcrun llvm-cov show "$BIN" -instr-profile="$W/merged.profdata" -show-instantiations=false "$L/$f" > "$W/show_$f.txt"
done
python3 -I "$CS/scope.py" "$W" "$W/scope.json"
