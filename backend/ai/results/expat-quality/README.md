# libexpat R_2_9_0: quality gate

n = 1, run 2026-10-25 13:58-14:13 PT. `ai.loop` from main `f5c4ac6` (rustfmt, clippy, panic lints, host fix, polish rounds). Loop inputs are seed 1, 747 inputs. Held-out is seed 1001, count 2000, minus the 747 inputs the loop saw: **1,253 unseen inputs**. libexpat has no adversarial corpus, so the evidence is the loop inputs plus the held-out set.

## Before vs after

| | Before: `argon-fresh-b/round_4` re-measured (`measure-b`, no model call) | After: host fix + 1 polish round (`polish`) |
|---|---|---|
| Loop inputs | 747/747 | 747/747 |
| Held-out | 1,253/1,253 | 1,253/1,253 |
| Rust crashes, loop + held-out | 0 | 0 |
| rustfmt (generated file) | 856 diff lines before host fix, clean after | clean |
| Default clippy warnings | 1 (`too_many_arguments`) | **0** |
| Panic lints (`indexing_slicing`, `unwrap_used`, `expect_used`, `panic`) | 153 (all `indexing_slicing`) | **0** |
| `arithmetic_side_effects` (reported only) | 491 | 488 |
| Rounds | 1 (measure only) | 2: start file + 1 polish; the loop stopped once the build was clean |
| Model seconds | 0 | 573 |
| Tokens | 0 | 626,924 in (280,695 cached), 75,239 out, 0 thought |
| Cost (`usd`) | $0.00 | **$1.47** |

Sources:
- Before: `measure-b/summary.json` (`quality_first_build`, then `quality` after host fix), `holdout`, `panics`.
- After: `polish/summary.json` (`quality`, `holdout`, `panics`, `total_model_seconds`, `usd`).

`measure-a` is the other passing port (`argon-fresh-a/round_3`). Its behavior is the same; it has 154 panic lints (one extra `unwrap_used`) and fails rustfmt. Its host fix did not run ("could not copy crate template"), most likely because both measure runs were started at once. b was better on every count, so polish started from b.

## What changed

- `1-host_fix.diff`: `argon-fresh-b/round_4` → host fix (`cargo clippy --fix` + rustfmt), kept after rebuild and reverify. Mostly layout.
- `2-polish.diff`: Argon's polish reply. Index sites became `.get()` / `.get_mut()` with `let ... else` early returns or `.unwrap_or(..)` fallbacks. One `#[allow(clippy::too_many_arguments)]` was added; the port already had 10 of those. No `allow` covers any panic lint, and there are no new `wrapping_`/`saturating_` ops. The reply itself had 3 `unnecessary_map_or` warnings and 37 lines of rustfmt diff.
- `3-polish-host_fix.diff`: the host fix cleared those, and it was kept after rebuild and reverify (`polish/round_2/host_fix/verify.json`).
- `passing_port_to_final.diff`: the whole change from the verified port to `expat.rs`.
- `expat.rs`: the chosen build, sha256 `759579bfab9f232a7aca46f45b24c1ccbccbe64b37c6feca05d6050673e1384c`.

## Independent recheck

`recheck.txt` is the output of `recheck.py`, which imports nothing from `backend/ai`. It:
- checks that the best file is the round file it claims to be, and that it contains no `unsafe`;
- rebuilds the file from scratch;
- reruns `verify.sh` on both the loop inputs and the held-out set;
- recounts fmt and lints from cargo's JSON in a fresh crate copy.

All 11 claims hold. Panic lints are keyed by full span: in cargo's `short` format, nested indexing such as `a[i].b[j]` prints as two identical lines and gets collapsed, so a line count undercounts by 2.

## Caveats

- **Prompt rule 7 was not used.** The "lint clean" rule was not in `EXPAT_SYSTEM_INSTRUCTION` for this run; `prompts.py` was unchanged. The polish turn's own message (`polish/round_2/lints_sent.md` plus `polish_input`) carried the lint report and the "behavior first, no arithmetic changes" rules.
- **Fallback values are only tested on these inputs.** A `.get(i).unwrap_or(0)` fallback returns a value on any path where C would read past its buffer. Behavior is preserved on these 2,000 tested inputs; that is the claim, not equivalence.
- **One run.** cJSON also reached 0/0, but this is n = 1 for libexpat.

How to say it: "Argon's 6.5K-line libexpat port went from 153 panic-prone index sites and 1 clippy warning to 0 and 0 in one polish round ($1.47, about 10 minutes). It still matches C on all 747 loop inputs and all 1,253 unseen held-out inputs, with 0 crashes."
