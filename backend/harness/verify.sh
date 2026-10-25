#!/bin/bash
# Differential verifier entry point. Thin wrapper over verify.py.
# Usage:
#   verify.sh --c <cmd> --rust <cmd> --inputs <dir> \
#             [--timeout 5] [--max-bytes 1048576] [--out runs/<name>.json]
# Prints the JSON report on stdout. Bash 3.2 compatible.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd -P)
export LC_ALL=C
exec python3 -I "$HERE/verify.py" "$@"
