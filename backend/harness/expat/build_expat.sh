#!/bin/bash
# Build the libexpat driver with ASan + UBSan into backend/harness/expat/build
# (gitignored). Compiles the library sources directly with clang and our
# hand-written config/expat_config.h; no cmake. Run fetch_expat.sh first.
# Bash 3.2 compatible.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd -P)
LIB="$HERE/vendor/libexpat/expat/lib"
OUT="$HERE/build"
CC="${CC:-clang}"
FLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer"
INC="-I$HERE/config -I$LIB"

if [ ! -f "$LIB/xmlparse.c" ]; then
  echo "ERROR: $LIB/xmlparse.c missing; run fetch_expat.sh first" >&2
  exit 1
fi

SOURCES="xmlparse.c xmlrole.c xmltok.c xcs.c random_dev_urandom.c"
case "$(uname -s)" in
  Darwin) SOURCES="$SOURCES random_arc4random_buf.c" ;;
esac

# Skip the (slow, ASan) rebuild when the binary is newer than every input.
up_to_date() {
  [ -x "$OUT/expat_driver" ] || return 1
  for f in "$HERE/drivers/expat_driver.c" "$HERE/config/expat_config.h" "$0" "$LIB"/*.c "$LIB"/*.h; do
    [ "$f" -nt "$OUT/expat_driver" ] && return 1
  done
  return 0
}
if [ "${FORCE:-0}" != "1" ] && up_to_date; then
  echo "up to date: $OUT/expat_driver"
  exit 0
fi

mkdir -p "$OUT"
OBJS=""
for src in $SOURCES; do
  obj="$OUT/${src%.c}.o"
  # shellcheck disable=SC2086
  "$CC" $FLAGS -std=c99 $INC -c "$LIB/$src" -o "$obj"
  OBJS="$OBJS $obj"
done
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c99 $INC -c "$HERE/drivers/expat_driver.c" -o "$OUT/expat_driver.o"
# shellcheck disable=SC2086
"$CC" $FLAGS $OBJS "$OUT/expat_driver.o" -o "$OUT/expat_driver"
echo "built $OUT/expat_driver"
