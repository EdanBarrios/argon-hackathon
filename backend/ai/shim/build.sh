#!/bin/bash
# Build the pyexpat shim: runs/shim/libtheseus_expat.dylib (contracts/pyexpat-shim.md).
# Usage: backend/ai/shim/build.sh        (THESEUS_ROOT overrides where runs/ goes)
# Bash 3.2 compatible. Needs cargo; builds offline for the host's default arch.
set -eu
SHIM=$(cd "$(dirname "$0")" && pwd -P)
ROOT=$(cd "$SHIM/../../.." && pwd -P)
OUT="${THESEUS_ROOT:-$ROOT}/runs/shim"

# Argon's core must be byte-identical to the verified artifact.
CORE_SRC="$ROOT/backend/ai/results/expat-long-decode/argon-fresh-b/round_4/expat.rs"
CORE_SHA=b3a97cbf79ceb9cf53f92e24c356d88ded25a6611688216212c385a274bbfccf
for f in "$SHIM/src/expat.rs" "$CORE_SRC"; do
  got=$(shasum -a 256 "$f" | cut -d' ' -f1)
  [ "$got" = "$CORE_SHA" ] || { echo "sha256 mismatch: $f ($got != $CORE_SHA)" >&2; exit 1; }
done
# unsafe only in ffi.rs
if grep -qw unsafe "$SHIM/src/expat.rs" "$SHIM/src/lib.rs"; then
  echo "'unsafe' found outside ffi.rs" >&2; exit 1
fi

cd "$SHIM"
cargo rustc --release --offline --lib --quiet -- \
  -C link-arg=-Wl,-install_name,@rpath/libtheseus_expat.dylib
mkdir -p "$OUT"
cp target/release/libtheseus_expat.dylib "$OUT/libtheseus_expat.dylib"
echo "$OUT/libtheseus_expat.dylib"
