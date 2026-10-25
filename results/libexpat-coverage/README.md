# libexpat: how much of the C our inputs exercise

Answers the critique "you never measured how much of the C your inputs exercise". Measured by the systems side (C) on 2026-10-25 with clang source-based coverage.

## Setup

- libexpat R_2_9_0 (2.9.0, `expat.h` version macros 2.9.0), same sources and `config/expat_config.h` as `backend/harness/expat/build_expat.sh`: `xmlparse.c xmlrole.c xmltok.c xcs.c` plus the two entropy shims, linked with our `drivers/expat_driver.c`.
- Flags: `clang -g -O0 -fprofile-instr-generate -fcoverage-mapping -std=c99`, no sanitizers. Apple clang / llvm-cov 17.0.0 via `xcrun`.
- Inputs: `gen_inputs_expat.py --seed 4242 --count 2000`, the same fresh seed the host re-verify used (`results/libexpat-host-reverify/`). 599 W3C conformance documents, 148 edge cases, 1,253 mutations.
- All 2,000 runs exited 0 and wrote a profile; the 2,000 profiles were merged with `llvm-profdata merge -sparse`.

## Raw coverage (llvm-cov report, no exclusions)

| File | Lines | Functions | Branches |
|---|---|---|---|
| `xmlparse.c` | 2,628 / 6,671 (**39.4%**) | 94 / 201 (46.8%) | 1,102 / 3,768 (29.2%) |
| `xmltok.c` | 509 / 806 (63.2%) | 25 / 37 (67.6%) | 306 / 530 (57.7%) |
| `xmltok_impl.c` | 1,231 / 1,527 (80.6%) | 27 / 28 (96.4%) | 1,201 / 2,028 (59.2%) |
| `xmltok_ns.c` | 38 / 48 (79.2%) | 5 / 7 (71.4%) | 8 / 10 (80.0%) |
| `xmltok.c` + `xmltok_impl.c` + `xmltok_ns.c` | 1,778 / 2,381 (**74.7%**) | 57 / 72 (79.2%) | 1,515 / 2,568 (59.0%) |
| `xmlrole.c` | 792 / 914 (**86.7%**) | 47 / 54 (87.0%) | 410 / 492 (83.3%) |
| **Parser total** (the five files) | 5,198 / 9,966 (**52.2%**) | 198 / 327 (60.6%) | 3,027 / 6,828 (**44.3%**) |

`xmltok_impl.c` is a template included three times (`normal_`, `little2_` for UTF-16LE, `big2_` for UTF-16BE). llvm-cov merges the three per source line, so a line counts as covered if any instantiation ran it. Per instantiation, 51 of 84 functions ran; 30 of the 33 that did not are `little2_`/`big2_` variants (UTF-16 input), the other 3 are `*_ignoreSectionTok`.

## Coverage of code reachable in our configuration (estimate)

Our pinned scope (`backend/harness/expat/README.md`) is `XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse(..., isFinal=1)` call, a fixed hash salt, and the driver sets a fixed list of handlers. A lot of `xmlparse.c` can never run under that scope. We excluded whole functions that are unreachable for one of these reasons (list and reason per function in `coverage-summary.json`, `in_scope_estimate_lines.excluded_functions`):

- public `XML_*` API functions the driver never calls (getters, property API, suspend/resume, reset, version, error strings, memory API);
- namespaces (`addBinding`, `is_rfc3986_uri_char`, `XML_ParserCreateNS`, the NS encoding inits);
- external entities, external subset and conditional sections (`XML_ExternalEntityParserCreate`, `dtdCopy`, `externalEntityInitProcessor*`, `externalParEnt*`, `entityValue*Processor`, `doIgnoreSection`, `*ignoreSectionTok`, xmlrole `externalSubset*`, `condSect*`);
- suspend, reset, re-parse after an error, multi-call parsing (`cdataSectionProcessor`, `errorProcessor`, `dtdReset`);
- handlers or options the driver never sets (default handler `reportDefault`, element-decl content model `build_model`, unknown-encoding handler, protocol encoding, entropy salt, amplification setters);
- UTF-16 `XML_Char` output (`*_toUtf16`, only in `XML_UNICODE` builds);
- debug and test-only accounting (`unsignedCharToPrintable`, 522 lines on its own).

Line counts here come from `llvm-cov show`, which counts a few more lines than `report` (10,164 vs 9,966 in total), so the raw percentages differ by under half a point.

| File | In-scope lines covered | In-scope functions called |
|---|---|---|
| `xmlparse.c` | 2,658 / 4,190 (**63.4%**) | 94 / 95 |
| `xmltok.c` | 572 / 651 (87.9%) | 26 / 26 |
| `xmltok_impl.c` | 1,374 / 1,552 (88.5%) | 51 / 81 instantiations (the 30 misses are UTF-16 variants) |
| `xmltok_ns.c` | 38 / 48 (79.2%) | 5 / 5 |
| `xmlrole.c` | 794 / 838 (94.8%) | 47 / 48 (`error`) |
| **Parser total** | 5,436 / 7,279 (**74.7%**) | |

This is an estimate with our own judgment in the exclusion list. It only removes whole functions. Unreachable paths inside called functions still count as misses, so it understates in-scope coverage rather than overstates it: namespace branches in `storeAtts`, default-handler branches in `doContent`, external-entity branches in `doProlog`, and every `XML_ERROR_NO_MEMORY` path (no allocation-failure injection).

## Biggest uncovered areas

Never-called functions in `xmlparse.c` by size: `unsignedCharToPrintable` (522 lines, debug), `XML_ExternalEntityParserCreate` (170), `addBinding` (155, namespaces), `dtdCopy` (149), `build_model` (137, element-decl handler), `is_rfc3986_uri_char` (109, namespaces), `XML_ErrorString` (106), `getContext` (84), `entityValueInitProcessor` (81), `doIgnoreSection` (77). All are out of scope except `XML_ErrorString`, which the driver does not call because it prints the numeric error code.

Inside functions that did run, the most missed (regions missed / total):

| Function | Regions missed | What the missed lines are |
|---|---|---|
| `doProlog` | 321 / 662 | 446 of 980 lines. Mostly out-of-scope or unexercised branches: 41 `XML_ERROR_NO_MEMORY` lines, external entity ref handler, element-decl handler, not-standalone handler, notation and attlist handlers we do not set. |
| `storeAtts` | 161 / 263 | 195 of 306 lines. Namespace attribute handling (`m_nsAtts*`, unbound prefix, triplets) and 22 `NO_MEMORY` lines. |
| `doContent` | 109 / 310 | 138 of 472 lines. 14 `reportDefault` calls (no default handler), 9 `NO_MEMORY`, external entity and namespace end handlers. |
| `getAttributeId` | 49 / 61 | Namespace prefix binding (`xmlns:` attributes). |
| `storeEntityValue` | 45 / 86 | Parameter entity references inside entity values and error paths. |
| `handleUnknownEncoding` | 32 / 35 | Unknown-encoding handler path (we set none). |
| `doCdataSection` | 32 / 63 | Partial and error paths in CDATA. |

Genuine in-scope gaps worth closing: UTF-16 input beyond the basics (30 `little2_`/`big2_` tokenizer functions never ran: comments, CDATA, char refs, entity values, public IDs in UTF-16), and the xmlrole `error` state.

## What this does and does not show

- It measures which C code ran under our 2,000 inputs, which bounds what the differential test could have compared. It does not measure how much of the Rust port ran.
- Covered is not the same as checked: 1,659 of the 2,000 inputs end in a parse error and are compared only on error code, line and column (`results/libexpat-host-reverify/README.md`).
- Honest one-liner: our inputs run 52% of the lines and 44% of the branches of the five parser files overall, and about 75% of the lines that are reachable in our pinned configuration (`xmlparse.c` about 63%, the tokenizer about 88%, the role state machine about 95%).

## Files

- `coverage-summary.json`: raw llvm-cov numbers per file, the in-scope estimate with every excluded function and its reason, and the most-missed called functions.
- `per-function.json`: every function instantiation with call count, source span and regions covered.
- `scripts/`: `build_cov.sh`, `run_cov.sh`, `show.sh`, `funcs.py`, `scope.py`, `summarize_cov.py`, `zero_tally.py`, as run. They have this session's scratch paths hard-coded; edit the paths at the top to rerun.
