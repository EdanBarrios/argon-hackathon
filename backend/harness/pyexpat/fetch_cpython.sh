#!/usr/bin/env bash
# Fetch CPython v3.13.5 sources needed to rebuild pyexpat/_elementtree and run
# the stdlib XML test suites. Output: backend/harness/pyexpat/vendor/cpython (gitignored).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
DEST="$HERE/vendor/cpython"
TAG="${CPYTHON_TAG:-v3.13.5}"
BASE="https://raw.githubusercontent.com/python/cpython/$TAG"
FILES=(
  Modules/pyexpat.c Modules/clinic/pyexpat.c.h
  Modules/_elementtree.c Modules/clinic/_elementtree.c.h
  Include/pyexpat.h
  Lib/test/test_pyexpat.py Lib/test/test_xml_etree.py Lib/test/test_xml_etree_c.py
  Lib/test/test_minidom.py Lib/test/test_sax.py
)
mkdir -p "$DEST"
for f in "${FILES[@]}"; do
  [ -s "$DEST/$f" ] && continue
  mkdir -p "$DEST/$(dirname "$f")"
  curl -sfL "$BASE/$f" -o "$DEST/$f"
done
# Test data (xmltestdata/ incl. the c14n-20 directory) from the release tarball.
if [ ! -d "$DEST/Lib/test/xmltestdata/c14n-20" ]; then
  curl -sfL "https://codeload.github.com/python/cpython/tar.gz/refs/tags/$TAG" \
    | tar -xz -C "$DEST/Lib/test" --strip-components=3 "cpython-${TAG#v}/Lib/test/xmltestdata"
fi
echo "cpython $TAG sources in $DEST"
