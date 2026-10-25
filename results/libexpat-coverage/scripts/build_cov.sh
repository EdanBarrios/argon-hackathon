#!/bin/bash
# Coverage build of libexpat R_2_9_0 with the same sources/config as build_expat.sh,
# but clang source-based coverage instead of ASan/UBSan.
set -eu
H=/Users/edan/dev/argon-hackathon/.claude/worktrees/agent-a6c1abe1b0ca65d61/backend/harness/expat
V=/Users/edan/dev/argon-hackathon/backend/harness/expat/vendor/libexpat/expat/lib
OUT=/private/tmp/claude-501/-Users-edan-dev/61a45a4c-bc60-4a55-b6e6-24090fbddade/scratchpad/cov/build
mkdir -p "$OUT"
FLAGS="-g -O0 -fprofile-instr-generate -fcoverage-mapping"
for s in xmlparse.c xmlrole.c xmltok.c xcs.c random_dev_urandom.c random_arc4random_buf.c; do
  clang $FLAGS -std=c99 -I"$H/config" -I"$V" -c "$V/$s" -o "$OUT/${s%.c}.o"
done
clang $FLAGS -std=c99 -I"$H/config" -I"$V" -c "$H/drivers/expat_driver.c" -o "$OUT/expat_driver.o"
clang $FLAGS "$OUT"/*.o -o "$OUT/expat_driver_cov"
echo built "$OUT/expat_driver_cov"
