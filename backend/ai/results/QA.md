# Q&A: three answers Brendan (A) owns on stage

Each answer has a short version to say out loud, then the evidence behind it. Every number is from a file in this folder or from the API's own interaction record.

## 1. "Why did the libexpat call use 815K input tokens for a 200K-token library?"

**Say:** "Argon's reply came out in three thinking-then-writing segments, and the usage counter charges the input once per segment. A one-segment cJSON call reports exactly its prompt size. So 815K is about four reads of the prompt, not four times the library. It's our reading of the API's usage record; Google hasn't confirmed it."

**Evidence** (`interactions.get` on each call's `interaction_id` from its `meta.json`):

| Call | Steps in the record | Reported input | Prompt size |
|---|---|---|---|
| cJSON translate (`cjson-2x2/gemini-4-argon-r1`) | user_input → thought → model_output | 33,733 | 33,733 (same) |
| libexpat translate (`expat-long-decode/gemini-4-argon/round_1`) | user_input → thought → output (16K chars) → thought → output (217K chars, the code) → thought → output (936 chars) | 815,566 | ~208K (Flash counted the same prompt as 207,813) |
| libexpat repair (`argon-fresh-a/round_3`) | user_input → thought → output → thought → output | 626,150, of which 554,740 cached | ~2x fewer |

- `total_cached_tokens` was 0 on the first libexpat call, so none of the re-reads were cached. Later repairs resent the same prefix and show large cached counts.
- Cost impact: at $2 per million input tokens, 815,566 is about $1.63. A single read would be about $0.42. Quote `usd` from `summary.json`, not this estimate.
- If pushed: "The fix on our side would be a smaller prompt, for example only the files a module needs, rather than the whole library each call. We kept the whole library in one call on purpose: that is the long-context claim."

## 2. "What did the two compile fixes change?"

**Say:** "Three lines in a 6,555-line file, and neither changed behavior. One added a type annotation the compiler couldn't infer. The other changed a `for` loop over a borrowed vector to an index loop, so the vector could be handed back to the parser on the duplicate-attribute error path. Rust's borrow checker caught a move of something still borrowed, which C would simply have allowed. The first build that compiled already matched all 2,000 inputs. A second, independent repair run made the same two fixes."

**Evidence:** `expat-long-decode/argon-fresh-a/round1_to_round3.diff` (whole diff):

```diff
5958c5958
<         let mut used_att_ids = Vec::with_capacity(raw_atts.len());
---
>         let mut used_att_ids: Vec<usize> = Vec::with_capacity(raw_atts.len());
5960c5960,5961
<         for curr_att in &raw_atts {
---
>         for i in 0..raw_atts.len() {
>             let curr_att = raw_atts[i];
```

- Fix 1, E0282 "type annotations needed": `used_att_ids` collects attribute ids for expat's duplicate-attribute check (`storeAtts`). The compiler could not infer the element type, so it got `Vec<usize>`.
- Fix 2, E0505 "cannot move out of `raw_atts` because it is borrowed": on a duplicate attribute, the code returns `raw_atts` to `self.atts` while still iterating over a borrow of it. Indexing by position copies each attribute record (it is `Copy`) and ends the borrow, so the move is legal.
- rustc reports borrow errors only after type checking passes, which is why it took two rounds.
- `argon-fresh-b` (same settings) made the same two fixes independently, plus one extra: `tag_stack.pop().unwrap()` became `let ... else { return XML_ERROR_ASYNC_ENTITY }`, removing a panic site on a path no input reaches. Both runs: 747/747 loop, 1,253/1,253 held-out. Edan's host re-verify on fresh seed 4242: 2000/2000 (`results/libexpat-host-reverify/`).

## 3. "What does the quality gate check?"

**Say:** "Matching the C output is the first gate. On top of that, the host, not the model, runs rustfmt and clippy on the generated file. We count default clippy warnings, and separately every site that could panic: indexing without a check, `unwrap`, `expect`, `panic!`. Then the host applies clippy's own safe fixes and rustfmt, and keeps that version only if a rebuild and a full reverify show behavior no worse. Leftover rounds go to the model as lint-only polish turns. A polish that changes even one output byte is thrown out; we caught exactly that once, when a polish broke tab and carriage-return escapes on 6 of 300 inputs. On cJSON the result was 0 clippy warnings and 0 panic sites, with held-out still 763/763."

**Evidence** (`backend/ai/quality.py`, `backend/ai/loop.py`, `cjson-quality/`):

- **Measured on the generated file only, in a temp copy of the harness crate:** `rustfmt --check`; `cargo clippy --offline --message-format=json` with default lints (the `-D warnings` gate) plus `-W` for `indexing_slicing`, `unwrap_used`, `expect_used` and `panic` (the panic lints). The harness's `main.rs` is excluded.
- **Reported but not gated:** `arithmetic_side_effects`. Forcing it to zero pushes code toward wrapping arithmetic, which can change behavior.
- **Host fix:** `cargo clippy --fix` (machine-applicable suggestions only) plus rustfmt, then rebuild and reverify. Kept only if the loop match rate is no lower, Rust crashes are no higher and lint counts are no worse.
- **Choosing a build:** loop match rate first, then lints. The held-out set never steers the choice; it is measured once on the chosen build.
- **Already enforced at build:** the word `unsafe` is rejected anywhere in the file, and the crate has `#![forbid(unsafe_code)]`. std only, no FFI, builds offline.
- **cJSON numbers:** default clippy 3 → 0, panic lints 41 → 0, held-out 763/763, targeted 980/980, $0.67 (`cjson-quality/`).
- **Be careful on stage:** libexpat has **not** been through the quality gate yet. Its claim is behavior (2,000 tested behaviors, no `unsafe`), not "0 clippy". The run that measures it is planned in `backend/ai/handoff/HANDOFF-expat-quality.md`.
