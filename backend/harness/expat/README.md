# backend/harness/expat

Owner: **C (systems)**. Differential harness for the moonshot target, **libexpat**, the XML parser inside Python, Firefox and Apache. Same method as the cJSON harness one level up: build the C library with ASan + UBSan, run it and a Rust port on the same inputs, and compare raw stdout bytes with the shared `../verify.sh`.

## Pinned version

libexpat **R_2_9_0** (2.9.0, released 2026-10-05, the latest stable release on 2026-10-25), commit `a76b1174c1ea52abb639a4c69d52b630a8a106a2`. Fetched into `vendor/libexpat`, never committed.

## Size (the "Argon reads the whole library" number)

`expat/lib/*.c` and `*.h`, measured by `fetch_expat.sh --count-tokens`:

| Set | Files | Lines (`wc -l`) | Bytes | Gemini tokens (`count_tokens`, gemini-4-argon) |
|---|---|---|---|---|
| All of `expat/lib` | 36 | 18,494 (15,020 in `.c`, 3,474 in `.h`) | 636,251 | **204,211** |
| Core (minus the `random_*` entropy shims and `winconfig.h`) | 23 | about 17,700 | about 607,000 | **195,762** |

`xmlparse.c` alone is 9,704 lines. The 4 chars/token rule of thumb gives 159K, which undercounts C by about 25%, so use the real count. Around 200K input tokens fits in one Argon context with ample room. The output is the hard part: a full safe-Rust port will likely be 100K+ tokens, so the run needs `background=True` and `max_output_tokens` well above 65,536.

## Scope (pinned)

- `XML_ParserCreate(NULL)`: no namespace processing. The encoding comes from the BOM or XML declaration, UTF-8 otherwise.
- No external entity loading: no external entity ref handler, parameter entity parsing left at NEVER.
- Internal DTD allowed: entity declarations and expansion, ATTLIST defaults, comments and PIs.
- Billion-laughs protection at library defaults (100x amplification, 8 MiB activation).
- A fixed all-zero hash salt (`XML_SetHashSalt16Bytes`), so runs are deterministic and need no entropy.
- One `XML_Parse(..., isFinal=1)` call on the whole input.

## Driver protocol

`drivers/expat_driver.c` (ours, not xmlwf) prints one line per callback, then `OK`. On a parse error it prints only `ERROR <XML_Error code> line <L> col <C>` (events before the error are discarded, matching the Rust `Result` interface). It always exits 0 unless it crashes.

```
XMLDECL "1.0" null -1
DOCTYPE_START "d" null null 1
ENTITY "e" 0 VALUE "x&amp;y"
DOCTYPE_END
START "d" "b"="<" "a"="1"
COMMENT "c"
PI "p" "q"
TEXT "hi x&y "
CDATA_START
TEXT "<z>"
CDATA_END
TEXT "\xc3\xa9"
END "d"
OK
```

Strings are quoted, with `\"`, `\\` and `\xNN` for every byte outside printable ASCII. Adjacent character data is joined into one `TEXT` line. The full format is in the driver's header comment. The Rust contract is in `rust_crate/INTERFACE.md`.

## How to run

```bash
backend/harness/expat/fetch_expat.sh [--count-tokens]   # libexpat + W3C suite, size report
backend/harness/expat/build_expat.sh                     # build/expat_driver (ASan+UBSan, no cmake)
python3 -I backend/harness/expat/gen_inputs_expat.py --seed 0 --count 2000
scripts/selftest_expat.sh                                # canary, C vs C, C vs stub
scripts/build_rust_expat.sh path/to/expat.rs [build_id]  # one JSON line, binary in runs/build-expat/<id>/
backend/harness/verify.sh --c backend/harness/expat/build/expat_driver --rust <binary> \
  --inputs backend/harness/expat/inputs --timeout 20
```

Use `--timeout 20`: macOS ASan symbolization is slow, and the billion-laughs input takes about 2 s per side under ASan.

Files:

- `fetch_expat.sh`: shallow clone at the pinned tag with a commit check, plus the W3C XML conformance suite `xmlts20130923.tar.gz` (sha256 pinned) into `vendor/xmlts`. Prints line counts.
- `count_tokens.py`: Gemini `models.count_tokens` on the library (no generation). Reads the key from `.env` and never prints it.
- `config/expat_config.h`: a hand-written config replacing the CMake-generated file. It mirrors a default build (`XML_DTD`, `XML_GE=1`, `XML_NS` compiled in, `XML_CONTEXT_BYTES 1024`).
- `build_expat.sh`: compiles `xmlparse.c xmlrole.c xmltok.c xcs.c` plus one entropy shim with `clang -g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer`. Skips the rebuild when the binary is up to date (`FORCE=1` to override).
- `gen_inputs_expat.py`: deterministic (`--seed`). It writes 2,000 inputs by default:
  - 599 W3C conformance documents: all 306 of James Clark's `xmltest` not-wf/sa and valid/sa, plus an even sample of sun, ibm, oasis, eduni and japanese.
  - 148 hand-written edge cases: entities, char refs, CDATA, comments, PIs, attributes (duplicate, unquoted, `&`, `<`), nesting to 100,000, BOMs, UTF-16 LE/BE, ISO-8859-1, US-ASCII, unknown encoding, invalid UTF-8, NUL, unclosed and mismatched tags, internal DTD expansion, recursive entities, and billion laughs at 4, 5 and 9 levels.
  - 1,253 mutations.
- `rust_crate/`: the fixed `src/main.rs` driver, the stub `src/expat.rs` (rejects everything) and `INTERFACE.md`.

## Self-test results

`scripts/selftest_expat.sh` (seed 0, 2,000 inputs, `--timeout 20`), run on 2026-10-25:

| Check | Result |
|---|---|
| Canary (a known document gives 15 exact event lines plus `OK`, and `<a>\n<b></a>` gives `ERROR 7 line 2 col 5`) | ok |
| C against C | match_rate **1.0**: 2000/2000 match, 0 divergences, 0 timeouts, no ASan or UBSan reports |
| C against the stub `expat.rs` (rejects everything) | match_rate **0.0165**: 33/2000 match, all of them inputs where C also returns `ERROR 2 line 1 col 0` |

Wall time was 9.6 minutes, with a load average around 240 from the other agents. Expect less on an idle machine.

Separately, a canned `expat.rs` that returns the canary's events produced output byte-identical to the C driver, including text coalescing and `\xNN` escapes. So `src/main.rs` and the C driver agree on the format.

## Security history worth citing

From the pinned release's own `expat/Changes` file ([R_2_9_0 Changes](https://github.com/libexpat/libexpat/blob/R_2_9_0/expat/Changes)):

- **71 distinct CVE IDs** appear in the changelog, from 2009 to 2026. 28 of them are from 2026 alone (releases 2.7.4 to 2.9.0). A few belong to the `xmlwf` tool, not the library.
- The changelog for this very release opens with a banner: "Expat has UNFIXED SECURITY ISSUES", pointing to [libexpat issue #1160](https://github.com/libexpat/libexpat/issues/1160).
- Notable entries:
  - [CVE-2022-40674](https://nvd.nist.gov/vuln/detail/CVE-2022-40674): heap use-after-free in `doContent` (2.4.9). Exactly the bug class safe Rust removes.
  - [CVE-2022-25235](https://nvd.nist.gov/vuln/detail/CVE-2022-25235): malformed 2- and 3-byte UTF-8 passed through to the application, "arbitrary damage (e.g. code execution)" (2.4.5).
  - [CVE-2022-22822](https://nvd.nist.gov/vuln/detail/CVE-2022-22822) to CVE-2022-22827: integer overflows near memory allocation, one CVE per C function (`addBinding`, `build_model`, `defineAttribute`, `lookup`, ...) (2.4.3).
  - [CVE-2024-8176](https://nvd.nist.gov/vuln/detail/CVE-2024-8176): stack overflow from chaining many entities, fixed by removing recursion (2.7.0). Our 100,000-deep nesting and entity inputs probe this class.
  - [CVE-2013-0340](https://nvd.nist.gov/vuln/detail/CVE-2013-0340): billion laughs. Protection arrived only in 2.4.0, in 2021, eight years after the CVE. Our port must keep it (`edge_bl_breach` must return code 43).
  - [CVE-2025-59375](https://nvd.nist.gov/vuln/detail/CVE-2025-59375): a ~250 KiB document could make the parser allocate ~800 MiB (2.7.2).
  - CVE-2026-72522: an out-of-bounds read (2.8.3), and CVE-2026-56403 to 56408: a cluster of integer overflows in `storeAtts`, `addBinding`, `getAttributeId`, `XML_ParseBuffer`, `textLen` and `copyString` (2.8.2).
