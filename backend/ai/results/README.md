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

Same settings, run `argon-fresh-b`: round 2 returned the file unchanged (0 thought tokens at medium); its later rounds are reported as they land.

Checks before quoting: no `std::process`, `std::fs`, `std::env` or `std::net` in the file; the C side is libexpat R_2_9_0 under ASan + UBSan (`backend/harness/expat`).

How to say it: "Argon read all of libexpat, the XML parser inside Python, 18.5K lines of C, and wrote 6.5K lines of safe Rust in one 18-minute reply. After two one-line compile fixes, it matches the C library on 2,000 tested behaviors, including the W3C conformance suite and 1,253 inputs it never saw. Flash could not produce a file." Say "N tested behaviors preserved", never "equivalent".

Open questions, recorded as reported:
- Usage for Argon's round 1 reports **815,566 input tokens** for a prompt of about 204K tokens. We have not explained the 4x.
- The repair turn failed at 65,416 output tokens, close to 65,536. Suspected: a follow-up turn (`previous_interaction_id`) has a lower output cap than a fresh call. Not confirmed.

## Files

- `<run>/summary.json`: per-round results, seconds, token usage, held-out block.
- `<run>/round_N/`: the Rust the model wrote, `build.json`, `verify.json`, `feedback_sent.json` (what the repair prompt was built from), `meta.json`.
- `<run>.log`: the loop's console output.
