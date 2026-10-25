#!/bin/bash
set -u
D=/private/tmp/claude-501/-Users-edan-dev/61a45a4c-bc60-4a55-b6e6-24090fbddade/scratchpad/cov
rm -f "$D"/prof/*.profraw
n=0; bad=0
for f in "$D"/inputs/*; do
  LLVM_PROFILE_FILE="$D/prof/%p.profraw" "$D/build/expat_driver_cov" < "$f" > /dev/null 2>&1 || bad=$((bad+1))
  n=$((n+1))
done
echo "ran $n inputs, nonzero exits $bad"
xcrun llvm-profdata merge -sparse "$D"/prof/*.profraw -o "$D/merged.profdata"
ls "$D"/prof | wc -l
