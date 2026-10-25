# Friction log

Things that surprised us, in plain language. Newest at the bottom. Use these for Q&A: each one is a place where the obvious approach was wrong.

## 10:50: The C-against-C self-test passed while the driver was broken

**What happened.** The first build of the cJSON driver rejected every input, even `[1]`. It printed `PARSE_ERROR` for valid JSON. Running C against C would still have reported a perfect 100% match, because both sides were wrong in exactly the same way.

**Why.** We call `cJSON_ParseWithLengthOpts(buf, len, &end, require_null_terminated=1)`. With that flag, cJSON wants the terminating NUL byte to sit *inside* the length you pass. We passed the exact input length, so cJSON looked one past the end, found no NUL, and failed. cJSON's own `cJSON_ParseWithOpts` passes `strlen + 1` for this reason. The fix: append a NUL and pass `len + 1`.

**What we changed.** `scripts/selftest.sh` now runs a canary first: one known document has to come back as `OK` with the exact expected output before the C-against-C run counts. A 100% match only means something if the driver really parses.

**After the fix.** C against C: match_rate 1.0 on 2000 inputs (seed 0) and on a separate 12,000-input mutation run (seed 7). No divergences, no timeouts, no flaky output, and no ASan or UBSan reports.

**Other facts the self-test pinned down (cJSON v1.7.19 through our driver):**
- `[1e999]` prints `OK\t[null]`, not `inf`. cJSON prints non-finite numbers as `null`.
- Nesting is capped at 1000. Depth 1001 and depth 100000 are both rejected cleanly with `PARSE_ERROR at 1000`. They do not overflow the stack.
- `[9007199254740993]` prints `[9.00719925474099e+15]`, which loses digits. cJSON's round-trip check uses an approximate comparison, so it accepts 15 significant digits. A Rust port that prints the shortest exact form will diverge here.
- Each ASan process takes about 0.2 s to start on macOS, so 2000 inputs take about 3.5 minutes on 8 cores. Repair loops should use a smaller `--count`.

## 11:13: The managed-agent sandbox has no C compiler, so ASan cannot run there

**What happened.** The smoke test (`scripts/sandbox_smoke.py`, antigravity agent on gemini-4-argon, 145 s) found no `clang`, `rustc` or `cargo`. rustup installed Rust in 12 s, but `rustc hello.rs` then failed with ``error: linker `cc` not found``. `apt-get update` returned `403 Forbidden [IP: 10.100.0.1 8081]` for every mirror, because the network allowlist proxy blocks them, so clang could not be installed. `build_cjson.sh` failed with `clang: command not found`.

**What worked.** `git clone --depth 1` of the public repo (0.8 s) and `fetch_cjson.sh`. Files in `/tmp` and `~/.cargo` survived into a follow-up turn, but only because the follow-up passed turn 1's `environment_id` as `environment`. Shell state does not persist: each turn must re-run `. "$HOME/.cargo/env"`.

**SDK gotchas.** Allowlist entries must be objects, `{"domain": "github.com"}`, not bare strings. Resending the environment dict on a follow-up may give a fresh sandbox, so pass the previous `environment_id` string instead.

**Untested option.** Adding the Ubuntu mirrors (`archive.ubuntu.com`, `security.ubuntu.com`) to the allowlist might let apt install clang. We have not tried this yet.
