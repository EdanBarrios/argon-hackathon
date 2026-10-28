#!/usr/bin/env bash
# Build CPython 3.13's own pyexpat (and _elementtree) for the local interpreter,
# linked against an expat-ABI dylib (the Argon Rust shim, or the C stub).
#
# usage: build_pyexpat.sh <path-to-dylib> <out-dir>
#        build_pyexpat.sh --object-only <out-dir>   (just pyexpat.o, for nm -u)
#
# Env: PYTHON (default python3), EXPAT_INC (dir with libexpat 2.9.0 expat.h).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
PY="${PYTHON:-python3}"
SRC="$HERE/vendor/cpython"
[ -s "$SRC/Modules/pyexpat.c" ] || "$HERE/fetch_cpython.sh"

if [ -z "${EXPAT_INC:-}" ]; then
  for d in "$HERE/../expat/vendor/libexpat/expat/lib"; do
    [ -s "$d/expat.h" ] && { EXPAT_INC="$d"; break; }
  done
fi
[ -s "${EXPAT_INC:-}/expat.h" ] || { echo "expat.h not found; set EXPAT_INC or run backend/harness/expat/fetch_expat.sh" >&2; exit 1; }

PYINC="$("$PY" -c 'import sysconfig;print(sysconfig.get_paths()["include"])')"
EXT_SUFFIX="$("$PY" -c 'import sysconfig;print(sysconfig.get_config_var("EXT_SUFFIX"))')"
CC="${CC:-clang}"
CFLAGS=(-fPIC -O2 -DNDEBUG -DPy_BUILD_CORE_MODULE -DXML_DTD -DXML_NS -DXML_GE=1
        -I"$EXPAT_INC" -I"$SRC/Include" -I"$SRC/Modules" -I"$PYINC" -I"$PYINC/internal")

if [ "$1" = "--object-only" ]; then
  OUT="$2"; mkdir -p "$OUT"
  "$CC" -c "${CFLAGS[@]}" "$SRC/Modules/pyexpat.c" -o "$OUT/pyexpat.o"
  echo "$OUT/pyexpat.o"; exit 0
fi

DYLIB="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
OUT="$2"; mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
LIBDIR="$(dirname "$DYLIB")"

# pyexpat: links the expat-ABI dylib; rpath points at its directory so the
# @rpath install name resolves without DYLD_LIBRARY_PATH.
"$CC" "${CFLAGS[@]}" -bundle -undefined dynamic_lookup \
  "$SRC/Modules/pyexpat.c" "$DYLIB" -Wl,-rpath,"$LIBDIR" \
  -o "$OUT/pyexpat$EXT_SUFFIX"

# _elementtree: no expat link (it calls expat through pyexpat's capsule), but
# it checks the capsule's XML_MAJOR/MINOR against the expat.h it was built with,
# so the stock one (expat 2.7) rejects ours (2.9). Rebuild it with the same headers.
"$CC" "${CFLAGS[@]}" -bundle -undefined dynamic_lookup \
  "$SRC/Modules/_elementtree.c" -o "$OUT/_elementtree$EXT_SUFFIX"

echo "built $OUT/pyexpat$EXT_SUFFIX and _elementtree$EXT_SUFFIX against $DYLIB"
otool -L "$OUT/pyexpat$EXT_SUFFIX" | sed -n '2,3p'
