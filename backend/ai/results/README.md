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
| Verified match rate | none yet (did not build) | none |

How to say it: "Argon translated the whole of libexpat (18.5K lines of C) into 6.5K lines of safe Rust in one 18-minute reply, with one compile error. Flash could not produce a file." Nothing about behavior is claimed until a build passes verify.sh.

Open questions, recorded as reported:
- Usage for Argon's round 1 reports **815,566 input tokens** for a prompt of about 204K tokens. We have not explained the 4x.
- The repair turn failed at 65,416 output tokens, close to 65,536. Suspected: a follow-up turn (`previous_interaction_id`) has a lower output cap than a fresh call. Not confirmed.

## Files

- `<run>/summary.json`: per-round results, seconds, token usage, held-out block.
- `<run>/round_N/`: the Rust the model wrote, `build.json`, `verify.json`, `feedback_sent.json` (what the repair prompt was built from), `meta.json`.
- `<run>.log`: the loop's console output.
