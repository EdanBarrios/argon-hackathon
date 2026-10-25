#!/bin/bash
# Build the cJSON driver with ASan + UBSan into backend/harness/build (gitignored).
# Run fetch_cjson.sh first. Bash 3.2 compatible.
#
# Usage: build_cjson.sh [OUTPUT]      (default OUTPUT: build/cjson_driver)
# Environment:
#   OPT      optimisation flag, default -O1 (selftest.sh also builds -O0)
#   CJSON_C  cJSON.c to compile, default the vendored one (mutants.py points
#            this at a mutated copy; cJSON.h always comes from vendor)
set -eu

HERE=$(cd "$(dirname "$0")" && pwd -P)
SRC="$HERE/vendor/cJSON"
CC="${CC:-clang}"
OPT="${OPT:--O1}"
CJSON_C="${CJSON_C:-$SRC/cJSON.c}"
TARGET="${1:-$HERE/build/cjson_driver}"
FLAGS="-g $OPT -fsanitize=address,undefined -fno-omit-frame-pointer"

if [ ! -f "$CJSON_C" ]; then
  echo "ERROR: $CJSON_C missing; run fetch_cjson.sh first" >&2
  exit 1
fi

# Object files go next to the target, prefixed with its name, so several
# drivers (-O1, -O0, mutants) can be built side by side.
mkdir -p "$(dirname "$TARGET")"
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c89 -I"$SRC" -c "$CJSON_C" -o "$TARGET.cJSON.o"
# shellcheck disable=SC2086
"$CC" $FLAGS -I"$SRC" -c "$HERE/drivers/cjson_driver.c" -o "$TARGET.driver.o"
# shellcheck disable=SC2086
"$CC" $FLAGS "$TARGET.cJSON.o" "$TARGET.driver.o" -o "$TARGET"
echo "built $TARGET"
