# EVIDENCE: cJSON v1.7.19 parse + print, C to safe Rust

Every number below was produced by `backend/harness/verify.sh` (differential: same input bytes to the
ASan/UBSan C driver and the Rust driver, raw stdout compared). The model did not grade itself.

## Result

| | match rate | behaviors tested | divergences | Rust crashes | timeouts |
|---|---|---|---|---|---|
| Host re-verify, seed 1 (same inputs as the sandbox) | 1.0000 | 514 | 0 | 0 | 0 |
| Host held-out, seed 1001 (inputs the agent never saw, 236 overlaps removed) | 1.0000 | 64 | 0 | 0 | 0 |
| Sandbox claim (verify.sh JSON the agent pasted) | n/a | | | | |

Sandbox claim and host re-verify agree: **n/a (verified on the host; no sandbox claim)**.

## Divergences found and fixed, per round (sandbox verify.sh reports)

| round | build | match rate | divergent inputs | Rust crashes |
|---|---|---|---|---|

## Crashes

Rust crashes (panic, signal) on the host run: 0. Both crashed: 0. C-only crashes where Rust exited cleanly (safety_improvement): 0.

## Safety improvement: CVE-2023-53154

cJSON v1.7.17 reads one byte past the end of the input buffer on an object that ends right after a comma
(`{"1":1,`), fixed upstream in v1.7.18. Same 7-byte PoC, run on the host:

- C v1.7.17 with ASan: `SUMMARY: AddressSanitizer: heap-buffer-overflow cJSON.c:786 in parse_string`
- This Rust crate: `PARSE_ERROR at 7` (exit 0)

Safe Rust cannot express the out-of-bounds read: indexing is bounds checked and the crate has
`#![forbid(unsafe_code)]`; the build script rejects the word `unsafe` anywhere in cjson.rs.

## Provenance

- Model: `gemini-3.8-flash` inside managed agent `antigravity-preview-09-2026`
- Sandbox seconds: None; tokens: input 90058, output 6662, thought 4301, cached 88009; estimated cost $0.122529
- Interaction `None`, environment `None`
- cjson.rs sha256 `1df02c3e9404799d97ad547c69424c145f799b002234f033df2ddd2906e1cdf4`
- Approved by **Edan** at 2026-10-25T12:27:57; branch `theseus/live-20261025-122502-job_a130a1-release`

Reproduce: `scripts/build_rust.sh migrated/cjson/src/cjson.rs` then
`backend/harness/verify.sh --c backend/harness/build/cjson_driver --rust <binary> --inputs <dir> --timeout 20`.
