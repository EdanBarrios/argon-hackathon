# 00: Plan toward a perfected Argon run

Scope: backend/ai only, gemini-4-argon only. "Perfected" means every input matches on the
held-out set and on the targeted adversarial corpus, `cargo clippy` is clean, `cargo fmt --check`
passes, and the Rust never panics.

Sources: 01-libraries.md, 02-rust-quality.md and 03-argon-tools.md (02 and 03 were pasted into
the session and are not yet saved under research/), backend/ai/README.md, docs/DECISIONS.md,
and the current loop.py. 01 does not change the top 5: a new target does not move cJSON or
libexpat closer to perfect. Its effect is in "Next target" below.

## Where we stand

- cJSON, Argon: held-out 763/763 in every saved run. Targeted corpus 980/980 in 1 of 4 ports,
  961/980 in the other 3, always the same 19 `str_straddle_*` inputs (DECISIONS 12:27).
- libexpat, Argon: one 6,554-line reply with one compile error (E0282). The repair turn over
  `previous_interaction_id` failed at 65,416 output tokens. No verified rate yet.
- summary.json has no quality fields at all. The build rejects the word `unsafe` and runs
  `cargo build` only, so clippy and fmt are never measured.
- Panics already show up dynamically: the crate builds with `panic = "unwind"` and verify.sh
  counts a panic as `rust_crash`.

## Status (2026-10-25, branch feat/ai-quality)

The next tests run on cJSON. Implemented from the quality list: host `clippy --fix` + rustfmt
with a rebuild and reverify (`loop.host_fix`), lint rule 7 in the cJSON system instruction,
polish rounds with a repair turn after a polish that breaks behavior, and panic-lint counts
plus `panics` in summary.json (`ai/quality.py`). One correction to the first measurement:
the 12-line fmt diff was in the harness's main.rs, and Argon's cjson.rs was already
rustfmt-clean, so fmt is now checked on the generated file only.

## Top 5 actions

### 1. Measure fmt, clippy and panic lints on every built round

- **Goal:** know the quality numbers before trying to move them. After a round builds, copy
  `harness/rust_crate` to a temp dir with the round's file, run `cargo fmt --check` and
  `cargo clippy --message-format=json -- -W clippy::all`, and separately count the
  panic-shaped restriction lints (`unwrap_used`, `expect_used`, `indexing_slicing`, `panic`,
  `arithmetic_side_effects`). Write `round_N/quality.json`, copy the best round's numbers into
  summary.json, and add the columns to `ai.report`.
- **Files:** backend/ai/loop.py (or a new backend/ai/quality.py), backend/ai/report.py,
  backend/ai/tests/.
- **Effort:** S.
- **Depends on:** nothing. Runs in our own temp crate, so the systems-owned build scripts stay
  untouched. Must work `--offline`, as the crate does today.
- **Proof:** `quality.fmt_ok`, `quality.clippy_warnings`, `quality.panic_lints` exist in
  summary.json for the best round. Also add `panics`: the sum of `rust_crash` over the loop,
  `holdout.counts` and every `adversarial.*.counts`.

### 2. Close the targeted-corpus gap without training on the test

- **Goal:** 980/980 on `targeted` in every run, not 1 in 4. The fix must not feed targeted
  divergences into repair, or the targeted rate stops being evidence. Feed repair from the
  generated inputs plus the `argon` corpus (226 inputs Argon wrote, which it is free to see),
  and keep `targeted` and `human` held out. First look at why 3 of 4 ports split on
  `str_straddle_*` (gen_targeted.py:159, strings that cross a buffer boundary) and whether the
  generic rules in prompts.py already cover it. Any prompt change has to be a general rule, never
  a description of those inputs.
- **Files:** backend/ai/loop.py (which corpora join the feedback inputs), backend/ai/prompts.py.
- **Effort:** M.
- **Depends on:** nothing in code. Needs a clear rule in DECISIONS.md on which corpora may feed
  repair, recorded before the change, since it changes what "held out" means.
- **Proof:** `adversarial.targeted.match_rate == 1.0` with `behaviors_tested == 980`, and
  `holdout.match_rate == 1.0`, across repeated runs (action 5).

### 3. Polish rounds once behavior matches

- **Goal:** today the loop stops at match 1.0. Instead, spend the leftover round budget on
  repair turns whose feedback is the clippy and panic-lint list from action 1, run
  `rustfmt` on the host (deterministic, no tokens, and reported as host-applied), and rebuild
  and reverify after each. Change best-build selection to lexicographic: loop match rate
  first, then fewer clippy warnings, then fewer panic lints. A polish round that drops the
  match rate can never become the held-out build.
- **Files:** backend/ai/loop.py, backend/ai/prompts.py (a short polish repair message),
  backend/ai/translate.py if the trim code needs a lint section.
- **Effort:** M.
- **Depends on:** 1.
- **Proof:** `quality.clippy_warnings == 0`, `quality.fmt_ok == true`, `quality.panic_lints`
  lower than round 1, with `holdout.match_rate` and `adversarial.targeted.match_rate` still
  1.0. Add `quality.fmt_applied_by_host` so a reader can tell model fmt from host fmt.

### 4. A libexpat repair that can finish

- **Goal:** get libexpat from "one compile error" to a verified held-out rate. Step one is
  cheap: rerun with `--start-from` the saved round 1 file and `--fresh-repair`, which avoids the
  follow-up turn that died near 65,536 output tokens. If a whole-file reply is still too long
  to repair reliably, switch expat repairs to a patch format (the model returns replaced
  functions, the host splices them in), which is the context slicing that 03 cites from PR2
  and C2SaferRust.
- **Files:** backend/ai/targets.py (expat defaults), backend/ai/translate.py and
  backend/ai/prompts.py (patch format), backend/ai/loop.py (splice step).
- **Effort:** S for the fresh-repair rerun, L for the patch format.
- **Depends on:** nothing for the rerun. The patch format needs a reliable function splitter on
  a 6.5K-line file.
- **Proof:** in the expat run's summary.json, `rounds[1].compile_ok == true`, then a numeric
  `holdout.match_rate`, and `usd` per round. Expat has no adversarial runner yet
  (`adversarial_run` is None), so "targeted" cannot be proven for it until the harness owner
  adds one.

### 5. Repeat runs so "every run" is a measured claim

- **Goal:** every number in results/ is n = 1. Run cJSON with Argon 3 or more times at the
  final settings and let `ai.report` show the spread (it already groups repeats). This is what
  turns actions 2 and 3 from one lucky sample into a result.
- **Files:** backend/ai/report.py (quality columns, min over the group), backend/ai/results/.
- **Effort:** S in code, and real API spend (each cJSON run is roughly 5 minutes and is
  recorded in `usd`).
- **Depends on:** 1 to 3.
- **Proof:** in `ai.report`, every Argon row in the group has held-out 1.0, targeted 980/980,
  `clippy_warnings` 0, `fmt_ok` true and `panics` 0.

## Next target (from 01)

01 picks LibYAML as the target after cJSON. I would not take that as given. PLAN.md section 2
wants a library with no well-known Rust port, so that a match cannot come from a memorized port.
`unsafe-libyaml` is a widely used, mechanical (c2rust) Rust translation of libyaml that serde_yaml
depends on, which is the worst case for memorization. 01's "no Rust port" claim also looks wrong
for zlib (zlib-rs, miniz_oxide) and SQLite (a Rust rewrite exists). These come from memory, not
from a fresh check, so verify them before deciding.

Recommendation: keep cJSON and libexpat as the targets for the perfected run. If a third target
is wanted, Jansson looks best of 01's list: MIT, small, a JSON API similar enough to cJSON to
reuse the harness shape, no known Rust port. Its CVE history is thin, though, which weakens the
security pitch. A new target is a `Target` entry in targets.py plus prompts on our side, but the
vendor fetch, C driver, input generator and build script belong to the harness owner, so it
needs their agreement first.

## Deliberately left out

- **cargo-geiger, cargo-mutants, cargo-fuzz, hyperfine, cargo-bloat, rust-code-analysis
  (02).** None is installed or needed for the goal. `unsafe` is already rejected outright, so
  geiger would always report 0. The crate has no dependencies and builds offline, so new cargo
  subcommands are a setup risk. 02's timings are all marked unverified.
- **Function-calling tools and the Antigravity SKILL.md (03).** A `compile` or `clippy` tool
  inside one interaction could save repair turns, but the host-driven loop already gives the
  same signal with one verdict path. Revisit only if action 4's patch format is not enough.
- **A composite `quality_score` (02).** The weights are arbitrary and the goal is a set of
  hard gates, not a score. Report the raw counts.

## Conflicts between the reports

1. **Clippy lint level.** 02 measures with `-W clippy::pedantic -W clippy::nursery`. 03's skill
   gates on `cargo clippy -- -D warnings` (default lints). "Clean clippy" should mean the
   default set at zero. Pedantic counts can be reported but are not a gate, since many pedantic
   lints (casts, `must_use`) fight a faithful C port.
2. **What picks the best build.** 02 step 4 ranks matching candidates by `quality_score`. Our
   rule (DECISIONS 12:27) ranks by loop match rate. Action 3 combines them lexicographically:
   behavior first, quality only breaks ties.
3. **What the behavioral gate runs on.** 02 gates the quality score on the stdout match but does
   not say which inputs. If that check used held-out inputs, the held-out set would steer
   selection and stop being held out. Selection must only use the loop's inputs.
4. **`previous_interaction_id` for repair.** 03 recommends it for context and implicit caching.
   Our expat data contradicts it for long files: the follow-up turn failed near 65,536 output
   tokens, and round 1 reported 815,566 input tokens for a ~204K prompt. `--fresh-repair`
   exists because of this. Neither claim is confirmed with Google.
5. **Build step.** 03 says use `cargo check` in intermediate rounds for speed. We need a binary
   to verify every round, so `cargo build --release` stays. `cargo check` only helps inside a
   function-calling tool, which is out of scope.
6. **Unsafe.** 02 spends a tool (geiger) and a 15-point penalty on unsafe code. Our build
   already rejects the keyword, so that term is always zero for any build that reaches verify.
7. **Repair caps.** 03 cites PR2's 5 repairs per pointer. We cap at the run level (`--rounds`).
   Same idea, different unit. No change unless the patch format in action 4 lands, where a
   per-function cap would fit.
8. **Target choice vs DECISIONS.** 01 ranks next targets without mentioning libexpat, which
   DECISIONS (12:09) already chose as the moonshot and which already has a working harness. 01
   also dates zlib to 1.2.11 (2017) and puts YAJL at ~1K lines, both of which look stale or low.
   Its CVE ids are partly marked unverified. Treat 01's table as a lead list, not facts.
9. **Reusing CRUST-Bench or TRACTOR corpora (01).** Both are GPLv3. They could serve as extra
   held-out inputs, but only for targets they actually cover, and the license should be checked
   before committing any of their files to this repo.
