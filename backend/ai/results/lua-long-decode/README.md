# Lua 5.4.7: one long-decode pass (Argon), round 1 only

Run 2026-10-25 15:43-16:14 PT on a local runner. This is the same call as `ai.loop --target lua` round 1: `translate.translate(..., targets.LUA)`, thinking high, background, `max_output_tokens` 1,048,576. The systems harness for Lua did not exist yet, so the prompt's interface, fixed `main.rs` and C driver are the scratch versions in `harness_scratch/`, written to `backend/ai/lua/HARNESS_SPEC.md`. The C driver was built against Lua 5.4.7 under ASan + UBSan and gives byte-identical output across runs.

| | libexpat round 1 (`expat-long-decode/gemini-4-argon`) | **Lua 5.4.7 round 1** (`round_1/meta.json`) |
|---|---|---|
| C in the prompt | 17,763 lines | 25,575 lines (52 files) |
| Prompt (`count_tokens`) | 206,294 | **276,914** |
| Reply | one reply, 6,554 lines | **one reply, 17,430 lines** (576 KB) |
| Output tokens | 73,653 | **191,440** (2.6x libexpat, 2.9x Flash's 65,536 cap) |
| Thought tokens | 57,767 | 29,888 |
| Seconds | 1,089 | 1,854 |
| Reported input / cached | 815,566 / 0 | 1,501,147 / 278,955 |
| Cost (`usd`) | | **$4.69** |
| Build | 1 compile error | **2 compile errors, both borrow-check** (E0499 at `lua.rs:930`, E0503 at `lua.rs:9798`; `round_1/build_stderr.txt`) |

- **What the build errors mean:** rustc runs borrow checking only after the whole crate passes name resolution and type checking, so all 17,430 lines type-check.
- **The two errors:**
  - a `get_mut` / fallback pattern that the current borrow checker rejects;
  - `SETARG_k(&mut pc_inst, GETARG_k(pc_inst) ^ 1)`.
- **The failed first attempt:** it started at 15:35 and failed server-side after 433 s with `INTERNAL`, with 0 tokens generated or charged (`attempt_1_failed.json`). The retry was the identical call.
- **No behavior is claimed.** Nothing has been run against the C driver. Verified results need the systems harness: generator, `build_rust_lua.sh` and preflight.
- **No Flash run.** Flash's 65,536-token cap is a third of this reply's output.

How to say it: "Argon translated the Lua 5.4.7 interpreter (25.6K lines of C) into 17,430 lines of safe Rust in one 31-minute reply: 191K output tokens, almost three times Flash's output cap. It type-checks; two borrow-checker errors remain. Behavior not yet verified."
