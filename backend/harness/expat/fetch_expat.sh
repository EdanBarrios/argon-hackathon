#!/bin/bash
# Fetch libexpat at a pinned release tag into backend/harness/expat/vendor/libexpat
# and the W3C XML conformance suite (xmlts20130923, input seeds) into
# vendor/xmlts. Both gitignored. Idempotent: pinned commit and sha256 are
# checked, and existing good copies are kept. Bash 3.2 compatible.
#
# Prints the size of the library (expat/lib/*.c and *.h): lines, bytes, and a
# token estimate. With --count-tokens it also asks the Gemini count_tokens API
# (needs GEMINI_API_KEY in the repo .env; the key is never printed).
set -eu

EXPAT_URL="https://github.com/libexpat/libexpat"
EXPAT_TAG="R_2_9_0"
EXPAT_COMMIT="a76b1174c1ea52abb639a4c69d52b630a8a106a2"
XMLTS_URL="https://www.w3.org/XML/Test/xmlts20130923.tar.gz"
XMLTS_SHA256="9b61db9f5dbffa545f4b8d78422167083a8568c59bd1129f94138f936cf6fc1f"

HERE=$(cd "$(dirname "$0")" && pwd -P)
DEST="$HERE/vendor/libexpat"
COUNT_TOKENS=0
[ "${1:-}" = "--count-tokens" ] && COUNT_TOKENS=1

current_commit() {
  git -C "$DEST" rev-parse HEAD 2>/dev/null || true
}

if [ -d "$DEST/.git" ] && [ "$(current_commit)" = "$EXPAT_COMMIT" ]; then
  echo "libexpat $EXPAT_TAG already present at $DEST"
else
  rm -rf "$DEST"
  mkdir -p "$HERE/vendor"
  git -c advice.detachedHead=false clone --quiet --no-tags --depth 1 --branch "$EXPAT_TAG" "$EXPAT_URL" "$DEST"
  GOT=$(current_commit)
  if [ "$GOT" != "$EXPAT_COMMIT" ]; then
    echo "ERROR: $EXPAT_TAG resolved to $GOT, expected $EXPAT_COMMIT" >&2
    rm -rf "$DEST"
    exit 1
  fi
  echo "libexpat $EXPAT_TAG fetched at $DEST ($GOT)"
fi

XMLTS_TGZ="$HERE/vendor/xmlts20130923.tar.gz"
if [ ! -f "$HERE/vendor/xmlts/xmlconf/xmlconf.xml" ]; then
  [ -f "$XMLTS_TGZ" ] || curl -sSLf -o "$XMLTS_TGZ" "$XMLTS_URL"
  GOT=$(shasum -a 256 "$XMLTS_TGZ" | awk '{print $1}')
  if [ "$GOT" != "$XMLTS_SHA256" ]; then
    echo "ERROR: xmlts sha256 $GOT, expected $XMLTS_SHA256" >&2
    rm -f "$XMLTS_TGZ"
    exit 1
  fi
  rm -rf "$HERE/vendor/xmlts"
  mkdir -p "$HERE/vendor/xmlts"
  tar -xzf "$XMLTS_TGZ" -C "$HERE/vendor/xmlts"
  echo "xmlts20130923 extracted at $HERE/vendor/xmlts"
else
  echo "xmlts20130923 already present at $HERE/vendor/xmlts"
fi

LIB="$DEST/expat/lib"
echo "size of $LIB (*.c, *.h):"
( cd "$LIB" && wc -l ./*.c ./*.h | sed 's#\./##' )
BYTES=$(cat "$LIB"/*.c "$LIB"/*.h | wc -c | tr -d ' ')
echo "bytes: $BYTES  estimated tokens at 4 chars/token: $(( BYTES / 4 ))"

if [ "$COUNT_TOKENS" -eq 1 ]; then
  PY="$HERE/../../.venv/bin/python"; [ -x "$PY" ] || PY=python3
  "$PY" "$HERE/count_tokens.py" "$LIB" || echo "count_tokens failed; use the estimate above" >&2
fi
