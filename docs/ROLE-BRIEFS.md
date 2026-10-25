# Role briefs (10:50)

Read this after docs/PLAN.md. Each role: what to ship, what to read, what to decide, what to watch.
Target is decided: **cJSON v1.7.19, core cJSON.c, parse + print first.** Stop researching the target; build.

---

## A: AI (`backend/ai/`)

**The method is decided. Don't research alternatives.** One Argon call via the Interactions API with all of `cJSON.c` in context (about 30K tokens, cheap) plus `backend/harness/rust_crate/INTERFACE.md`. Argon returns one complete `cjson.rs`. Repair = send the verifier's feedback back with `previous_interaction_id` and get a full new `cjson.rs`.

**Deliverable by 11:30:**
1. Sandbox smoke test result (PLAN section 4): did rustc + clang work in the managed agent, and how many seconds? This decides the 11:15 runner call.
2. `backend/ai/translate.py` with these two functions (C wires them into the pipeline, so keep the signatures):
   ```python
   def translate(c_source: str, model: str = "gemini-4-argon") -> TranslateResult
   def repair(prev: TranslateResult, feedback: dict, model: str = "gemini-4-argon") -> TranslateResult
   # TranslateResult: rust_source: str, interaction_id: str, usage: dict
   # feedback: {"compile_errors": str | None, "divergences": [...], "crashes": [...]}  (shape of verify.sh output)
   ```
3. Proof: run `translate()` once, save the output, then `scripts/build_rust.sh out/cjson.rs` builds it. Match rate doesn't matter yet; compiling does.

**Read:** `backend/harness/rust_crate/INTERFACE.md` (the exact contract), `src/main.rs` (fixed driver), `docs/GEMINI-NOTES.md`, PLAN section 6.

**Decide (log in docs/DECISIONS.md):**
- Runner at 11:15: sandbox or local, based on your smoke test.
- Output extraction: ask for exactly one ```rust block; reject replies without one.
- Repair feedback size: send compile errors in full; send at most ~10 divergences per round, one per distinct pattern, not 10 copies of the same bug.
- Repair budget: proposed 5 rounds, then report honestly.
- Thinking: planner/translate `high`, repair `medium`.

**Think about:**
- Argon took 14s just to say "ok". Real calls will take minutes, so use `background=True` and poll. Never block an HTTP request on it.
- `tools`, `system_instruction` and `generation_config` must be re-sent every turn, even with `previous_interaction_id`.
- The Flash baseline must use the **same prompts and budget**. Make `model` a parameter, not a copy-paste.
- The repo is public, so the managed agent can `git clone` it from an allowlisted github.com to get the harness into the sandbox.
- Argon never sees "pass/fail" from itself; only `verify.sh` decides. That's the line "Argon never grades its own homework."
- Moonshot (libexpat) kicks off ~12:30, **only after** one loop works.

---

## B: Frontend (`frontend/`)

**Deliverable by 11:30:** Next.js scaffold (system fonts, no Google Fonts) showing **agent lanes** and the **divergence table** from `contracts/fixtures/`. Proof: a screenshot in the team chat.
**By 12:30:** the same UI driven by the live API (`GET /jobs/{id}`, `/events?after=`, polling). A mock API that replays the fixtures over about 30s is landing now. Switch to it when C says.

**Read:** `contracts/README.md`, all of `contracts/fixtures/`, PLAN section 8 (the demo script: every screen you build should map to a timestamp there).

**Decide:**
- Hero screen for the live demo: lanes on top, divergence table below, match rate big.
- Poll interval (1s is fine locally).
- What a clicked divergence shows: input, C output vs Rust output side by side, function name. This is the 0:50 demo moment.

**Build in this order:** lanes, divergence table + detail, match-rate chart over iterations, crash card ("C: ASan stack-overflow / Rust: clean error", demo 1:40), approve button that shows the PR link (2:20), Flash vs Argon 2x2 table.

**Think about:**
- It's projected: large fonts, high contrast, no tiny tables.
- If the live run stalls, the UI must still demo from a finished (pre-run) job. Make sure loading a completed job by id works.
- You also record the 1-minute video at 4:00, so keep the UI recordable.

---

## C: Systems (`backend/api/`, `backend/harness/`, `scripts/`)

Done: repo, contract, guard, cJSON driver, `verify.sh`, input generator, Rust crate template, `build_rust.sh`. In progress: C-vs-C self-test, mock `/jobs` API, cost tracking. Next: wire A's `translate`/`repair` + build + verify into the real pipeline.

---

## Checkpoints

- **11:15:** runner decision (A + C).
- **11:30:** integration, 15 min max. Bring your proof.
- **12:30:** thin slice end to end on one module.
