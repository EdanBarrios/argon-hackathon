# CPython 3.13.5's pyexpat on Argon's Rust libexpat

CPython's own, unmodified `Modules/pyexpat.c` and `Modules/_elementtree.c` (tag
v3.13.5) compiled against libexpat 2.9.0's `expat.h` and linked to an expat-ABI dylib.
The interpreter is the stock `/opt/anaconda3` Python 3.13.5 (x86_64 macOS); our two
extension modules shadow the built-in ones via `PYTHONPATH`. Contract:
`contracts/pyexpat-shim.md`.

`_elementtree`: rebuilt from v3.13.5 source too (not forced to pure Python). It talks to
expat only through pyexpat's capsule, but checks the capsule's `XML_MINOR_VERSION`
against its own headers, so the stock one (2.7) would reject a 2.9 pyexpat. Both
`test_xml_etree` (pure-Python ElementTree on our pyexpat) and `test_xml_etree_c`
(C accelerator on our pyexpat capsule) are run.

## Results (CPython v3.13.5 test files, `python -m unittest -v test.<suite>`)

ARGON_TABLE

### Reference rows

Same harness, same 780 tests.

| Library behind pyexpat | run | pass | fail | err | skip |
|---|---|---|---|---|---|
| Stock pyexpat (bundled expat 2.7.1), baseline | 780 | 768 | 0 | 0 | 12 |
| C libexpat 2.9.0 through our build path (control) | 780 | 768 | 0 | 0 | 12 |
| C stub that rejects every document (floor) | 780 | 392 | 10 | 374 | 4 |

The control shows the build path itself (our pyexpat.so, our _elementtree.so, rpath,
2.9.0 headers) loses nothing. The floor matters for reading the Argon row: 392 tests
pass without parsing anything (API surface, constructors, error paths, tests that
never reach the parser), so only passes above 392 are evidence of real parsing.

Per suite, baseline = control: test_pyexpat 42/42, test_sax 185/185,
test_minidom 129/129, test_xml_etree 198/203 (5 skip), test_xml_etree_c 214/221 (7 skip).
Stub floor: test_pyexpat 12, test_sax 145, test_minidom 57, test_xml_etree 82,
test_xml_etree_c 96.

FAILURES

## Repro

```
# Python side (branch feat/systems-pyexpat)
backend/harness/pyexpat/fetch_cpython.sh            # v3.13.5 sources, tests, xmltestdata
backend/harness/expat/fetch_expat.sh                # libexpat 2.9.0 headers (if missing)

# Argon shim (branch feat/ai-pyexpat-shim)
backend/ai/shim/build.sh                            # -> runs/shim/libtheseus_expat.dylib
backend/harness/pyexpat/build_pyexpat.sh runs/shim/libtheseus_expat.dylib runs/pyexpat/build-argon
backend/harness/pyexpat/run_tests.sh runs/pyexpat/build-argon --label argon   # + baseline

# Reference rows
backend/harness/pyexpat/build_c_control.sh runs/pyexpat/cctl
backend/harness/pyexpat/build_pyexpat.sh runs/pyexpat/cctl/libexpat_c_control.dylib runs/pyexpat/build-cctl
backend/harness/pyexpat/run_tests.sh runs/pyexpat/build-cctl --label c-control --no-baseline
backend/harness/pyexpat/build_stub.sh runs/pyexpat/stub
backend/harness/pyexpat/build_pyexpat.sh runs/pyexpat/stub/libtheseus_expat_stub.dylib runs/pyexpat/build-stub
backend/harness/pyexpat/run_tests.sh runs/pyexpat/build-stub --label stub --no-baseline

# Demo
PYTHONPATH=runs/pyexpat/build-argon python3 backend/harness/pyexpat/demo.py
```

Logs and `summary.json` per run: `runs/pyexpat/<label>/` (gitignored).
