#!/bin/bash
# Fetch cJSON at a pinned tag into backend/harness/vendor/cJSON (gitignored).
# Idempotent: if the checkout already exists at the pinned commit, do nothing.
# Bash 3.2 compatible.
set -eu

CJSON_URL="https://github.com/DaveGamble/cJSON"
CJSON_TAG="v1.7.19"
CJSON_COMMIT="c859b25da02955fef659d658b8f324b5cde87be3"

HERE=$(cd "$(dirname "$0")" && pwd -P)
DEST="$HERE/vendor/cJSON"

current_commit() {
  git -C "$DEST" rev-parse HEAD 2>/dev/null || true
}

if [ -d "$DEST/.git" ] && [ "$(current_commit)" = "$CJSON_COMMIT" ]; then
  echo "cJSON $CJSON_TAG already present at $DEST"
  exit 0
fi

rm -rf "$DEST"
mkdir -p "$HERE/vendor"
git -c advice.detachedHead=false clone --quiet --depth 1 --branch "$CJSON_TAG" "$CJSON_URL" "$DEST"

GOT=$(current_commit)
if [ "$GOT" != "$CJSON_COMMIT" ]; then
  echo "ERROR: $CJSON_TAG resolved to $GOT, expected $CJSON_COMMIT" >&2
  rm -rf "$DEST"
  exit 1
fi
echo "cJSON $CJSON_TAG fetched at $DEST ($GOT)"
