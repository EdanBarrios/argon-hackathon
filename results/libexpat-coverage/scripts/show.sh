#!/bin/bash
D=/private/tmp/claude-501/-Users-edan-dev/61a45a4c-bc60-4a55-b6e6-24090fbddade/scratchpad/cov
L=/Users/edan/dev/argon-hackathon/backend/harness/expat/vendor/libexpat/expat/lib
for f in xmlparse.c xmltok.c xmltok_impl.c xmlrole.c xmltok_ns.c; do
  xcrun llvm-cov show "$D/build/expat_driver_cov" -instr-profile="$D/merged.profdata" -show-instantiations=false "$L/$f" > "$D/show_$f.txt"
done
