#!/bin/bash
# Build the cJSON driver with ASan + UBSan into backend/harness/build (gitignored).
# Run fetch_cjson.sh first. Bash 3.2 compatible.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd -P)
SRC="$HERE/vendor/cJSON"
OUT="$HERE/build"
CC="${CC:-clang}"
FLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer"

if [ ! -f "$SRC/cJSON.c" ]; then
  echo "ERROR: $SRC/cJSON.c missing; run fetch_cjson.sh first" >&2
  exit 1
fi

mkdir -p "$OUT"
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c89 -I"$SRC" -c "$SRC/cJSON.c" -o "$OUT/cJSON.o"
# shellcheck disable=SC2086
"$CC" $FLAGS -I"$SRC" -c "$HERE/drivers/cjson_driver.c" -o "$OUT/cjson_driver.o"
# shellcheck disable=SC2086
"$CC" $FLAGS "$OUT/cJSON.o" "$OUT/cjson_driver.o" -o "$OUT/cjson_driver"
echo "built $OUT/cjson_driver"
