#!/bin/bash
# Build the OLD (vulnerable) cJSON v1.7.17 driver with ASan + UBSan.
# Reproducible: pins the tag and verifies the commit. Bash 3.2 compatible.
# Nothing here is tracked by git except this script (vendor/ and build/ are
# gitignored).
set -eu

CJSON_URL="https://github.com/DaveGamble/cJSON"
CJSON_TAG="v1.7.17"
CJSON_COMMIT="87d8f0961a01bf09bef98ff89bae9fdec42181ee"

HERE=$(cd "$(dirname "$0")" && pwd -P)
SRC="$HERE/vendor/cJSON"
OUT="$HERE/build"
CC="${CC:-clang}"
# Task-specified flags: -O0 for this build (baseline driver uses -O1).
FLAGS="-g -O0 -fsanitize=address,undefined -fno-omit-frame-pointer"

current_commit() {
  git -C "$SRC" rev-parse HEAD 2>/dev/null || true
}

if [ ! -d "$SRC/.git" ] || [ "$(current_commit)" != "$CJSON_COMMIT" ]; then
  rm -rf "$SRC"
  mkdir -p "$HERE/vendor"
  git -c advice.detachedHead=false clone --quiet --depth 1 --branch "$CJSON_TAG" "$CJSON_URL" "$SRC"
fi

GOT=$(current_commit)
if [ "$GOT" != "$CJSON_COMMIT" ]; then
  echo "ERROR: $CJSON_TAG resolved to $GOT, expected $CJSON_COMMIT" >&2
  exit 1
fi

mkdir -p "$OUT"
# shellcheck disable=SC2086
"$CC" $FLAGS -std=c89 -I"$SRC" -c "$SRC/cJSON.c" -o "$OUT/cJSON_old.o"
# shellcheck disable=SC2086
"$CC" $FLAGS -I"$SRC" -c "$HERE/cjson_driver_old.c" -o "$OUT/cjson_driver_old.o"
# shellcheck disable=SC2086
"$CC" $FLAGS "$OUT/cJSON_old.o" "$OUT/cjson_driver_old.o" -o "$OUT/cjson_driver_old"
echo "built $OUT/cjson_driver_old (cJSON $CJSON_TAG @ $GOT)"
