# libexpat: managed-agent sandbox verify, then host check

A Gemini managed-agent sandbox (`antigravity-preview-09-2026`, model `gemini-4-argon`) independently built libexpat R_2_9_0 and Argon's port, and ran the verifier. No model wrote, translated or repaired code: the prompt only lists commands, and the transcript shows 12 shell calls and one reply. The sandbox's number is a claim. The host then checked it. Run at 13:46 to 13:59 PT.

- Port: `backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs`, sha256 `418570361191d0019294b981d80e3185c7a732e0d4c02a627107f60b5af72c0d`, same in the sandbox and on the host.
- Sandbox, from a fresh `git clone` of this public repo at `9355beb`: apt clang, rustup, `fetch_expat.sh` (libexpat from github.com at the pinned commit, the W3C suite downloaded from www.w3.org and sha256-checked), `build_expat.sh` (ASan + UBSan), `scripts/build_rust_expat.sh` with the banned-API guard, `gen_inputs_expat.py --seed 7777 --count 2000`, `verify.sh --timeout 20`.
- Seed 7777 was never used before (repair loop seed 1, held-out 1001, host re-verify 4242).

| | Sandbox claim | Host check |
|---|---|---|
| match_rate | **1.0** (2000/2000) | **1.0** (2000/2000) |
| divergences, crashes, timeouts, infra failures | 0, 0, 0, 0 | 0, 0, 0, 0 |
| inputs digest (2000 files) | `0b75225e853471de...` | `0b75225e853471de...` (equal) |
| Platform | Linux x86_64 (gVisor), 9 cores | macOS x86_64 |

`match_rates_agree: true`, `inputs_digest_agree: true`, `expat_rs_sha256_agree: true`, `counts_agree: true`. The verify JSON pulled from the sandbox filesystem through the Environments files API matches the JSON the agent pasted.

What was compared (host, C driver output on the 2000 seed-7777 inputs): 335 inputs parse successfully and match event for event (233,515 events; 215 W3C documents, 70 edge cases, 50 mutations). The other 1,665 end in a parse error and match on the exact error code, line and column.

## Time and cost

| Stage | Seconds |
|---|---|
| Preflight gate (`scripts/preflight.sh expat`, host) | 112.7 |
| Sandbox interaction, end to end | 634.5 |
| inside it: apt clang / rustup / clone / fetch / build C / build Rust | 75 / 41 / 52 / 33 / 49 / 133 |
| inside it: sha256 / gen inputs / digest / verify.sh / summary | 37 / 46 / 34 / 53 / 26 |
| verify.sh alone in the sandbox (its `elapsed_seconds`) | 24.3 |
| Host: gen inputs / verify.sh (ran while the sandbox worked) | 1.5 / 112.6 |

Sandbox stage times are gaps between timestamps the agent wrote before each step, so they include the agent's turn latency. Tokens: 194,338 input (154,522 cached), 1,772 output. Estimated cost **$0.11**.

Interaction `v1_ChdHV3ZlYXJIUkpLbWkxTWtQejlfSjhBbxIXR1d2ZWFySFJKS21pMU1rUHo5X0o4QW8`, environment `4929dbd432dcf405e0d0803b2e3c5929`.

## Files

- `summary.json`: the numbers above. `result.json`: the full run record.
- `verify-sandbox-seed7777.json` (pulled from the sandbox) and `verify-host-seed7777.json`: raw verify.sh reports.
- `agent_output.md`: the agent's reply. `transcript.jsonl`: every step. `prompt.md`, `sandbox_stages.txt`.

Code: `backend/harness/remote/verify_remote.py` (`run_verify_remote`), CLI `scripts/run_remote.py --library expat --verify-only <expat.rs>`.

## Known limitations

- Same scope as `results/libexpat-host-reverify/README.md`: one parser configuration, events before an error not compared, hash flooding untested.
- n = 1: one port, one sandbox run, one fresh seed. 747 of the 2000 inputs (W3C and edge cases) are fixed across seeds; the 1,253 mutations are new.
- The host side used the binary built earlier from the same file (`runs/build-expat/edan-reverify-a`), not a rebuild.
- The W3C tarball was also mounted into the sandbox as a fallback; the agent reported, and the transcript shows, that it downloaded from www.w3.org instead.
