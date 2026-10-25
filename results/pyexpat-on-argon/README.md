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

Argon shim: `runs/shim/libtheseus_expat.dylib` built from `feat/ai-pyexpat-shim`
(merged to main as e9b9690) with `backend/ai/shim/build.sh` on this machine, x86_64,
dylib sha256 `5cca1c9c…f186`. Core `expat.rs` = `argon-fresh-b/round_4`, sha256
`b3a97cbf…bfccf` (checked by build.sh). `pyexpat.EXPAT_VERSION` = `expat_2.9.0-theseus-argon`.

| suite | run | Argon pass | fail | err | skip | baseline pass | stub floor | Argon above floor |
|---|---|---|---|---|---|---|---|---|
| test_pyexpat | 42 | 26 | 10 | 6 | 0 | 42 | 12 | 14 |
| test_sax | 185 | 169 | 7 | 9 | 0 | 185 | 145 | 24 |
| test_minidom | 129 | 114 | 0 | 15 | 0 | 129 | 57 | 57 |
| test_xml_etree | 203 | 160 | 22 | 20 | 1 | 198 | 82 | 78 |
| test_xml_etree_c | 221 | 177 | 20 | 21 | 3 | 214 | 96 | 81 |
| **total** | **780** | **646** | **59** | **71** | **4** | **768** | **392** | **254** |

646/780 pass on Argon vs 768/780 on stock expat. Of the 376 baseline-passing tests that
the stub fails (768 - 392), Argon passes 254 (68%). Every test the stub passes also
passes on Argon (checked per test name), so "above floor" is a clean subset.

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

## What fails and why (130 = 59 fail + 71 err, grouped by cause)

All five groups are on the contract's "knowingly unsupported" list except the
encoding group, which is a real gap.

| cause | tests | where |
|---|---|---|
| **Namespaces** (option c: any `xmlns` or prefixed name in a namespace parser is a clean `unbound prefix` error) | 71 | etree 23, etree_c 23, minidom 15, sax 6, pyexpat 4 |
| **Incremental delivery** (shim buffers until `isFinal`, so `feed()`-time events, `XMLPullParser`/`iterparse` before close, `buffer_size` chunking counts, `Parse` after final) | 32 | etree 12, etree_c 12, pyexpat 6, sax 2 |
| **Encodings**: non-UTF-8 documents (`iso-8859-1`, `latin-1` decl -> `unknown encoding`), and `str` input / character streams where a declared encoding conflicts with the UTF-8 override pyexpat sets (`not well-formed (invalid token)`, wrong non-ASCII text) | 12 | sax 5, pyexpat 3, etree 2, etree_c 2 |
| **DTD / entities**: doctype handler, notation decls, `UseForeignDTD`, external DTD/entity resolution, undefined-entity error not raised, internal-DTD entity expansion (xmltoolkit62) | 14 | etree 5, etree_c 4, sax 3, pyexpat 2 |
| **Positions during callbacks** (`CurrentByteIndex` -1 expected at a handler) | 1 | pyexpat |

Failing test names with the last traceback line: `runs/pyexpat/argon/failures.tsv`
(`python3 backend/harness/pyexpat/failures.py runs/pyexpat/argon`).

Scope claim this supports: pyexpat and SAX on Argon's libexpat, plus ElementTree on
namespace-free XML, with UTF-8 input, parsed at close rather than incrementally.

## Demo

```
$ PYTHONPATH=runs/pyexpat/build-argon python3 backend/harness/pyexpat/demo.py
pyexpat: /Users/edan/dev/argon-hackathon/runs/pyexpat/build-argon/pyexpat.cpython-313-darwin.so | expat_2.9.0-theseus-argon
<inventory owner="argon"><item id="1">bolt</item><item id="2">nut &amp; washer</item></inventory> [('1', 'bolt'), ('2', 'nut & washer')]
start a {'x': '1'}
start b {}
```

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
