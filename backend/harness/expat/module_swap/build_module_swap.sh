#!/bin/bash
# Build the "module swap" libexpat driver: the same drivers/expat_driver.c and
# the original C xmlparse.c + xmltok.c (+ xcs.c and the entropy shim), but with
# xmlrole.c replaced by the Rust staticlib in xmlrole_rs/. C parts get
# ASan + UBSan exactly as in build_expat.sh. Output: module_swap/build/.
#
# EXPAT_LIB overrides the libexpat lib/ dir (default ../vendor/libexpat/expat/lib).
# ARGON_RS (optional) re-extracts xmlrole_rs/src/argon_role.rs from that file.
# Bash 3.2 compatible.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd -P)
EXPAT=$(cd "$HERE/.." && pwd -P)
REPO=$(cd "$EXPAT/../../.." && pwd -P)
LIB="${EXPAT_LIB:-$EXPAT/vendor/libexpat/expat/lib}"
OUT="$HERE/build"
CC="${CC:-clang}"
FLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer"
INC="-I$EXPAT/config -I$LIB"

[ -f "$LIB/xmlparse.c" ] || { echo "ERROR: $LIB/xmlparse.c missing (set EXPAT_LIB or run fetch_expat.sh)" >&2; exit 1; }

if [ -n "${ARGON_RS:-}" ]; then
  python3 -I "$REPO/scripts/extract_argon_xmlrole.py" "$ARGON_RS" "$HERE/xmlrole_rs/src/argon_role.rs"
fi

(cd "$HERE/xmlrole_rs" && cargo build --release --offline --quiet)
RSLIB="$HERE/xmlrole_rs/target/release/libxmlrole_rs.a"

mkdir -p "$OUT"

# Check that Argon's XML_TOK_* / XML_ROLE_* numbers match the C headers, since
# C now consumes the role numbers Rust returns.
CHECK="$OUT/const_check.c"
{
  echo '#include <stdio.h>'
  echo '#include <stddef.h>'
  echo '#include "expat_config.h"'
  echo '#include "expat_external.h"'
  echo '#include "xmltok.h"'
  echo '#include "xmlrole.h"'
  echo 'int main(void) { int bad = 0;'
  # Layout assumed by the Rust #[repr(C)] mirrors in xmlrole_rs/src/ffi.rs.
  echo '  if (sizeof(PROLOG_STATE) != 32 || offsetof(PROLOG_STATE, inEntityValue) != 24) { printf("MISMATCH PROLOG_STATE layout\n"); bad = 1; }'
  echo '  if (offsetof(ENCODING, minBytesPerChar) != 16 * sizeof(void *) || offsetof(ENCODING, isUtf16) != 16 * sizeof(void *) + 5) { printf("MISMATCH ENCODING layout\n"); bad = 1; }'
  grep -E '^const (XML_TOK_|XML_ROLE_)[A-Z0-9_]+: i32 = -?[0-9]+;' "$HERE/xmlrole_rs/src/argon_role.rs" |
    sed -E 's/^const ([A-Z0-9_]+): i32 = (-?[0-9]+);.*/  if (\1 != (\2)) { printf("MISMATCH \1 C=%d Rust=%d\\n", (int)\1, \2); bad = 1; }/'
  echo '  if (!bad) printf("constants ok\n"); return bad; }'
} > "$CHECK"
# shellcheck disable=SC2086
"$CC" -std=c99 $INC "$CHECK" -o "$OUT/const_check"
"$OUT/const_check"

SOURCES="xmlparse.c xmltok.c xcs.c random_dev_urandom.c"
case "$(uname -s)" in
  Darwin) SOURCES="$SOURCES random_arc4random_buf.c" ;;
esac
OBJS=""
for src in $SOURCES; do
  [ -f "$LIB/$src" ] || continue
  obj="$OUT/${src%.c}.o"
  # shellcheck disable=SC2086
  "$CC" $FLAGS -std=c99 $INC -c "$LIB/$src" -o "$obj"
  OBJS="$OBJS $obj"
done
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c99 $INC -c "$EXPAT/drivers/expat_driver.c" -o "$OUT/expat_driver.o"
# shellcheck disable=SC2086
"$CC" $FLAGS $OBJS "$OUT/expat_driver.o" "$RSLIB" -o "$OUT/expat_driver_swap"

# The swap binary must not contain any of xmlrole.c's own symbols.
if nm "$OUT/expat_driver_swap" | grep -qE ' t _?(prolog0|doctype0|attlist0|element0)$'; then
  echo "ERROR: C xmlrole symbols present in swap binary" >&2; exit 1
fi
echo "built $OUT/expat_driver_swap"
