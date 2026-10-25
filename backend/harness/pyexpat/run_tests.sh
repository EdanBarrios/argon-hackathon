#!/usr/bin/env bash
# usage: run_tests.sh <out-dir> [--label NAME] [--no-baseline]
# Runs test_pyexpat, test_sax, test_minidom, test_xml_etree(_c) from CPython v3.13.5
# with our pyexpat/_elementtree first on PYTHONPATH, then the stock baseline.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
[ -d "$HERE/vendor/cpython/Lib/test/xmltestdata" ] || "$HERE/fetch_cpython.sh"
exec "${PYTHON:-/opt/anaconda3/bin/python3}" "$HERE/run_tests.py" "$@"
