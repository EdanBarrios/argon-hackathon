# EVIDENCE: cJSON v1.7.19 parse + print, C to safe Rust

Every number below was produced by `backend/harness/verify.sh` (differential: same input bytes to the
ASan/UBSan C driver and the Rust driver, raw stdout compared). The model did not grade itself.
Agreement on the tested inputs below is evidence, not a proof of equivalence.

**Scope:** cJSON v1.7.19 parse+print through our driver. **Status:** experimental; needs maintainer review.

**Claim:** agreement on 578 tested inputs (seed 1) and None held-out inputs; not a proof of equivalence.

## Certificate

- Source: `EdanBarrios/argon-hackathon` harness commit `5848d5a9b25501bc63f16dd15d0eb0130a63d5d9`; reference C: cJSON `v1.7.19` at `c859b25da02955fef659d658b8f324b5cde87be3`
- Model: `gemini-3.8-flash, repaired by gemini-4-argon`; interaction `None`; environment `None`
- Same-seed inputs (seed 1, None generated): match rate 1.0000; attempted 578, scored 578, matched 578, skipped 0, safety_improvements 0, infra_failures 0
- Held-out inputs (seed 1001, 236 overlaps removed): match rate 1.0000; counts: not available (report has no summary object)
- Sandbox claimed match rate n/a; host re-verified 1.0000; agree: n/a (verified on the host; no sandbox claim)
- Verifier preflight (canary, C vs C, reject-everything stub): passed (canary: ok, c_vs_c: ok, stub: ok, mutants: skipped; 117.3 s)
- Machine-readable copy: `certificate.json`

## Result

| | match rate | behaviors tested | divergences | Rust crashes | timeouts |
|---|---|---|---|---|---|
| Host verify, live job job_29a12b (inputs-c300-s1+targeted_str_) | 1.0000 | 578 | 0 | 0 | 0 |
| Host held-out, seed 1001 (inputs the agent never saw, 236 overlaps removed) | 1.0000 | 64 | 0 | 0 | 0 |
| Sandbox claim (verify.sh JSON the agent pasted) | n/a | | | | |

Sandbox claim and host re-verify agree: **n/a (verified on the host; no sandbox claim)**.

## Divergences found and fixed, per round (verify.sh reports from the live job)

| round | build | match rate | divergent inputs | Rust crashes |
|---|---|---|---|---|
| 1 | ok | 0.9671 | 19 | 0 |
| 2 | ok | 1.0000 | 0 | 0 |

Divergences the repair rounds fixed (input, C output, Rust output before the fix):

- round 1: input `"\uZ\t\\"`: C `OK	""`, Rust `PARSE_ERROR at 7`
- round 1: input `"\uZ\\\\"`: C `OK	""`, Rust `PARSE_ERROR at 7`
- round 1: input `"\u\xd4\t\\"`: C `OK	""`, Rust `PARSE_ERROR at 7`
- round 1: input `"\uZ\"\\"`: C `OK	""`, Rust `PARSE_ERROR at 7`
- round 1: input `"\u000\\"`: C `OK	""`, Rust `PARSE_ERROR at 7`
- round 1: input `"a\uZZZ\\"`: C `OK	"a"`, Rust `PARSE_ERROR at 8`
- round 1: input `["\uZ\t\\",1]`: C `OK	["",1]`, Rust `PARSE_ERROR at 8`
- round 1: input `["\uZ\\\\",1]`: C `OK	["",1]`, Rust `PARSE_ERROR at 8`

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

- Model: `gemini-3.8-flash, repaired by gemini-4-argon` inside managed agent `antigravity-preview-09-2026`
- Sandbox seconds: None; tokens: input 90163, output 7372, thought 4401, cached 1310; estimated cost $0.295567
- Interaction `None`, environment `None`
- cjson.rs sha256 `5b8c3fa03b36b6a15dd9354bdc54faefb450163892ca2ec80609f2075277e861`
- Approved by **maintainer** at 2026-10-25T16:06:34; branch `theseus/live-20261025-155633-job_29a12b-release`

## Known limitations

- Covers parse+print only (cJSON_ParseWithLengthOpts and cJSON_PrintUnformatted through our driver), not the full cJSON API or its C ABI.
- Output after an embedded NUL byte is not compared: the C driver prints with fputs, so both sides are compared only up to the first NUL.
- The banned-API guard (no std::process, std::fs, std::net, Command, include!, ...) is a text scan of cjson.rs, not a sandbox.
- Agreement on the tested inputs is not a proof of equivalence: inputs nobody generated can still diverge.
- Experimental: produced by a model and checked by our differential verifier; it needs maintainer review before anyone relies on it.

## Reproduce

No reproduce.sh for this job: spec.json has no seed/inputs: cannot write a reproduce script with the same inputs
