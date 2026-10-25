#!/bin/bash
# Required verifier preflight gate. Run before any model call.
# Checks: driver canary, C vs C == 1.0, reject-everything stub near 0, and for
# cjson optionally the mutant kill rate (--mutants, slow). Prints one JSON line on
# stdout, logs on stderr, exits nonzero if any check fails.
#
# Usage: scripts/preflight.sh <cjson|expat> [--count 200] [--seed 0] [--mutants]
# Details: backend/harness/preflight.py. Bash 3.2 compatible.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
export LC_ALL=C
exec python3 -I "$ROOT/backend/harness/preflight.py" "$@"
