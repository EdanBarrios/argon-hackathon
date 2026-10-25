# AI-side results (A)

Every number here is copied from a `summary.json`, `meta.json` or `build.json` the harness wrote. Same prompts for both models (`backend/ai/prompts.py`), same verifier, same seeds. Local machine absolute paths are replaced with `<repo>`.

## cJSON v1.7.19: the 2x2 (`cjson-2x2/`)

n = 1 per cell, run 2026-10-25 11:55-12:15 PT. The loop's inputs are seed 1, 300 inputs. Held-out = seed 1001, count 1000, minus every input whose bytes the loop saw: **763 unseen inputs**.

| | Single pass (`--rounds 1`) | Repair loop (`--rounds 5`) |
|---|---|---|
| **Argon** | held-out **763/763**, 364 s, round 1, 33.7K in / 7.0K out / 30.5K thought | held-out **763/763**, 270 s, round 1 (no repair needed), 33.7K / 6.7K / 20.5K |
| **Flash** | **no file**: incomplete at thinking high, retry at medium also incomplete; 917 s, 122K thought tokens | held-out **763/763**, 512 s: round 1 (high) failed to compile (one error, ambiguous range pattern), round 2 (medium) fixed it |

With C's earlier runs (`results/cjson-flash-vs-argon/`, no held-out): Argon's first call produced a working port 4 of 4 times. Flash's first round did 0 of 4 (three hit the 65,536-token output cap, one compile error). Both reach the same correctness with a retry or a repair. Do not claim Flash "can't" do cJSON.

## libexpat R_2_9_0: one long-decode pass (`expat-long-decode/`)

Run 12:15-12:42 PT, local runner. Input: all of `expat/lib` minus the entropy shims and `winconfig.h`, 23 files, about 196K tokens by `count_tokens`, plus the interface and both drivers. Settings: `background=True`, `max_output_tokens=1,048,576`, translate at thinking high, repair at medium, 4-round budget.

| | Argon | Flash |
|---|---|---|
| Round 1 (translate) | **completed in one reply**: 1,089 s, **6,554 lines** of `expat.rs`, 73,653 output + 57,767 thought tokens | cut off: `incomplete` at thinking high after 62,912 thought / 2,620 output tokens (459 s); retry at medium failed with `DEADLINE_EXCEEDED` (1,065 s total) |
| Build | **1 compile error**: E0282 "type annotations needed" at `expat.rs:5967` | no file |
| Round 2 (repair, `previous_interaction_id`) | **failed server-side**: `INVALID_ARGUMENT` after 505 s at 65,416 output tokens, so no repaired file | n/a |
| Verified match rate | see the headline below: 1.0 after two compile-only repairs | none |

How to say it: "Argon translated the whole of libexpat (18.5K lines of C) into 6.5K lines of safe Rust in one 18-minute reply, with one compile error. Flash could not produce a file." Nothing about behavior is claimed until a build passes verify.sh.

## libexpat headline: Argon's port passes 747/747 and 1,253/1,253 held-out (`argon-fresh-a`)

**Argon's one-pass libexpat translation, after two compile-only repairs, matches the C library on every tested input.**

| | |
|---|---|
| Loop inputs | **747/747** match: 599 W3C XML conformance documents + 148 edge cases (100,000-deep nesting, billion laughs at 4/5/9 levels incl. the code-43 breach, recursive entities, UTF-16 LE/BE with and without BOM, ISO-8859-1, invalid UTF-8, NUL) |
| Held-out inputs | **1,253/1,253** match (seed 1001 mutations, minus all 747 the loop saw), 0 divergences, 0 crashes, 0 timeouts |
| Rust | 6,555 lines of safe Rust, std only, no `unsafe` anywhere (build-script check) |
| Change after the one-pass translation | **3 lines** (`argon-fresh-a/round1_to_round3.diff`): one type annotation (E0282) and one borrow fix (E0505). No behavior was repaired: the first build already matched every input |
| Model time | 1,089 s translate + 570 s + 601 s repairs = 37.7 min |
| Tokens (as reported) | translate: 815,566 in / 73,653 out / 57,767 thought; repairs: 1,251,950 in (554,740 cached) / 147,332 out / 0 thought. One failed repair call (`gemini-4-argon/round_2`, 338K in / 65K out) is not included |

How it ran: round 1 = `gemini-4-argon/round_1/expat.rs` (`--start-from`, no new model call). Each repair is a new conversation (`--fresh-repair`) with the C source, the previous file and the compiler output, thinking medium. Continuation off. Rounds 2 and 3 each fixed one compile error (rustc reports borrow errors only after type checking passes).

**Repeat, same settings, run `argon-fresh-b`: also 747/747 and 1,253/1,253 held-out**, in round 4 (round 2 returned the file unchanged at 0 thought tokens; round 3 fixed the E0282; round 4 fixed the E0505). It made the **same two compile fixes independently**, plus one extra change (`argon-fresh-b/round1_to_round4.diff`): `self.tag_stack.pop().unwrap()` became a `let ... else { return XML_ERROR_ASYNC_ENTITY }`, which removes a panic site on a path no input reaches. So n = 2 repair runs from the one-pass file, both 1.0 on every input set.

Checks before quoting: no `std::process`, `std::fs`, `std::env` or `std::net` in the file; the C side is libexpat R_2_9_0 under ASan + UBSan (`backend/harness/expat`).

How to say it: "Argon read all of libexpat, the XML parser inside Python, 18.5K lines of C, and wrote 6.5K lines of safe Rust in one 18-minute reply. After two one-line compile fixes, it matches the C library on 2,000 tested behaviors, including the W3C conformance suite and 1,253 inputs it never saw. Flash could not produce a file." Say "N tested behaviors preserved", never "equivalent".

Open questions, recorded as reported:
- Usage for Argon's round 1 reports **815,566 input tokens** for a prompt Flash counted as 207,813. The interaction's own `steps` record (`interactions.get`) shows the reply was produced in **3 thought → model_output segments**; a cJSON call with 1 segment reports exactly its prompt (33,733), and a 2-segment libexpat repair reports about 2x its prompt (626,150, of which 554,740 cached). So reported input grows with the number of generation segments, consistent with the context being re-read for each segment. That is our reading of the usage record, not confirmed by Google; see `QA.md`.
- The repair turn failed at 65,416 output tokens, close to 65,536. Suspected: a follow-up turn (`previous_interaction_id`) has a lower output cap than a fresh call. Not confirmed.

## cJSON v1.7.19: quality gate (`cjson-quality/`)

n = 1, run 2026-10-25 ~12:55-13:10 PT with `ai.loop` from `feat/ai-quality` (rustfmt, clippy, panic lints, host fix, polish rounds). Same seeds as the 2x2: loop seed 1 / 300 inputs, held-out 763 unseen.

| | Argon 2x2 port, re-measured (`argon-polish-old-port` round 1, no model call) | Fresh Argon run, prompt rule 7 + polish (`argon-full`) |
|---|---|---|
| Held-out | 763/763 | 763/763 |
| Targeted corpus | 961/980 | **980/980** |
| Argon corpus | 225/225 | 225/225 |
| rustfmt (generated file) | clean | clean |
| Default clippy warnings | 3 | **0** (`cargo clippy -- -D warnings` exits 0) |
| Panic lints (indexing_slicing, unwrap_used, expect_used, panic) | 41 | **0** (already 0 in round 1) |
| arithmetic_side_effects (reported only) | 62 | 40 |
| Rust crashes, all input sets | 0 | 0 |
| Rounds, cost | | translate 352 s + polish 55 s, $0.67 |

`argon-full.recheck.txt` is `recheck.py`, which imports nothing from `backend/ai`: it rebuilds the best file from scratch, reruns verify.sh on the held-out set and run.sh on the corpora, and recounts lints from plain cargo output. All 13 claims hold.

`argon-polish-old-port` is the gate working: a polish of the 2x2 port cleared every lint but broke print escapes for `\t` and `\r` on 6/300 loop inputs (`round_2/verify.json`), so it was never held out; the held-out build stayed the original. Since then a broken polish gets a normal repair turn instead of ending the loop.

How to say it: "With the lint rules in the prompt and one polish round, Argon's cJSON port is clippy clean with zero panic-prone index or unwrap sites, and still matches C on 763 unseen inputs and all 980 targeted ones." One run; 980/980 targeted was 1 of 4 before, so repeat before quoting it as typical.

## libexpat R_2_9_0: quality gate (`expat-quality/`)

n = 1, run 2026-10-25 13:58-14:13 PT. The start file was `argon-fresh-b/round_4`, followed by the host fix and 1 polish round (`--fresh-repair`). Loop inputs: 747. Held-out: 1,253 unseen.

| | Before (`measure-b`) | After (`polish`) |
|---|---|---|
| Loop / held-out | 747/747, 1,253/1,253 | 747/747, 1,253/1,253 |
| Rust crashes | 0 | 0 |
| Default clippy warnings | 1 | **0** |
| Panic lints | 153 | **0** |
| arithmetic_side_effects (reported only) | 491 | 488 |
| Cost | $0 | 573 s, **$1.47** |

Prompt rule 7 was not in the expat system instruction; the polish turn's own message carried the lint report. `expat-quality/recheck.txt`: all 11 claims hold. Details and caveats are in `expat-quality/README.md`.

## Files

- `<run>/summary.json`: per-round results, seconds, token usage, held-out block.
- `<run>/round_N/`: the Rust the model wrote, `build.json`, `verify.json`, `feedback_sent.json` (what the repair prompt was built from), `meta.json`.
- `<run>.log`: the loop's console output.
