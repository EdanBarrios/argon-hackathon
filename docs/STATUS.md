# Status

Last updated 2026-10-28. The README has the results; this page says what works, what is unverified and what is next. Hackathon-day documents (plan, pitch, role briefs, handoffs) are in `docs/hackathon/` and are kept for history.

## What works

- **Differential verifier** (`backend/harness/verify.sh`): C against Rust on identical bytes, one outcome class per input, self-tested with C-vs-C, a canary and a reject-everything stub.
- **libexpat 2.9.0 parser core**: Argon's port matches C on 2,000/2,000 inputs, reproducible without an API key with `scripts/reproduce.sh`. A second independent translation agrees.
- **cJSON 1.7.19**: Argon and Flash both reach 2,000/2,000 generated inputs; both miss the same 19 targeted `str_straddle_*` inputs in most ports.
- **Jobs API, run view, Try panel**: work from a clean clone in mock mode.
- **Managed-agent run and PR step**: one saved run for each model, with host re-verification (`results/cjson-sandbox-run/`).

## Unverified or unfinished

| Item | State |
|---|---|
| Lua 5.4.7 | Translated (17,430 lines), 2 borrow errors fixed by hand, informal smoke test 22/22. No harness, so **not verified**. |
| Edit-based repair (`--patch-repair`) | Implemented and unit tested. Never run against the model, so there is no token or latency number. |
| Coverage-guided differential fuzzing | Scripts on branch `feat/systems-diff-fuzz` (WIP). Run stopped mid-way, no write-up. |
| Module swap (xmlrole.c replaced by Rust) | Crate and build script on `feat/systems-module-swap` (WIP). Stopped while writing the FFI wrapper. |
| Verifier audit of timeouts | `verify.py` keeps timeouts in the denominator, so they never count as matches. Flaky-test behavior beyond that has not been studied. |
| Platforms | Only macOS x86_64 with Apple clang has been run. |
| CVE replay script | `results/libexpat-cve-replay/reproduce.sh` now builds the committed port by default; syntax-checked, not re-run after that change. |

## Known weaknesses

- 1,659 of 2,000 libexpat seed-4242 inputs end in a parse error and are compared only on error code, line and column.
- 18 `unwrap_or(0)` fallbacks in the libexpat port where C would read out of bounds.
- Hash-flooding protection is untested (fixed hash salt).
- The pyexpat bridge has an aliasing issue in its unsafe layer.
- The banned-API guard is a text scan, not AST analysis.

## Next

1. Measure `--patch-repair` against the saved full-file repair (output tokens and seconds per repair) once a Gemini key is available.
2. Build a Lua harness and verify the translation, or stop calling it a target.
3. Finish one of fuzzing or module swap with a real write-up.
4. Replace `unwrap_or(0)` fallbacks with errors and re-verify.
5. Record a demo GIF once the run view is in better shape.
6. Run on Linux and add CI (`scripts/reproduce.sh` is a good first job).

## Rules for claims

Say "N tested behaviors preserved". Do not say "proves", "equivalent", "secure", "drop-in replacement" or "Lua is verified". Every number needs a saved file under `results/` or `backend/ai/results/`.
