You are the **AI role (A)** on Theseus in `/Users/brendankeane/Documents/GitHub/argon-hackathon1` (Gemini 4 Argon SF hackathon, Sun Oct 25 2026; **feature freeze 3:30 PM PT, submit 4:45 PM**). Read `CLAUDE.md`, `docs/PLAN.md` (sections 2, 3, 6), `docs/DECISIONS.md`, `backend/ai/README.md` and `backend/ai/results/README.md` first. `.role` is `ai`: you may edit only `backend/ai/` and `docs/DECISIONS.md`. Everything else is C's (Edan, systems), including `runs/handoff/`: send C a note or a PR, never a direct edit, and never work around the edit guard via bash.

## Goal

Run the Rust quality optimizer from `feat/ai-quality` on **libexpat**, the next repo. It is the big, proven port and has never been through the quality gate. Then, if there is time, prepare the third repo (below). The deliverable is a committed `backend/ai/results/expat-quality/` with numbers that come only from harness and tool output.

"Perfected" means: held-out match 1.0, `cargo fmt --check` clean on the generated file, default `cargo clippy` at 0 (`-D warnings` exits 0), panic lints (`indexing_slicing`, `unwrap_used`, `expect_used`, `panic`) as low as possible without changing behavior, and 0 Rust crashes on every input set.

## Where things stand (13:15)

- **main** has the libexpat results: Argon's one-pass port (6,555 lines, `backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs`) needed 3 lines of compile fixes, then matched 747/747 loop inputs and 1,253/1,253 held-out. Edan's independent host re-verify on fresh seed 4242: 2000/2000 (`results/libexpat-host-reverify/`). Flash produced no file.
- **`feat/ai-quality`** (pushed, `7fbcaeb`, not yet on main) has the optimizer: `backend/ai/quality.py` (rustfmt and clippy counts on the generated file only, in a temp crate built from `target.crate_template`), `loop.host_fix` (host `cargo clippy --fix` + rustfmt, then rebuild and reverify; the fix is kept only if the loop rate is no lower, crashes are no higher and lints are no worse), polish rounds (`--polish-rounds`, lint-only turns after the build matches every loop input; a polish that breaks behavior is never chosen, and the next round repairs it), and prompt rule 7 (lint-clean) **in the cJSON system instruction only**.
  - cJSON result: clippy 3 → **0**, panic lints 41 → **0**, held-out 763/763, targeted 980/980, $0.67 (`backend/ai/results/cjson-quality/`). One polish of the older port broke `\t`/`\r` print escapes on 6/300 and was correctly rejected, so the guard is real.
- **libexpat target today:** `polish_rounds=0` and no lint rule in `EXPAT_SYSTEM_INSTRUCTION`, so the optimizer is wired but switched off for it. libexpat has no adversarial corpus (`adversarial_run=None`), so its evidence is the loop corpus + held-out.

## First steps

1. **Work in your own worktree.** Another session shares the main checkout and switches branches in it; that is how commits ended up on the wrong branch earlier. Put the worktree under `.claude/worktrees/` (excluded via `.git/info/exclude`), so `translate._load_key` still finds the repo-root `.env` by walking up:
   ```bash
   cd /Users/brendankeane/Documents/GitHub/argon-hackathon1
   git fetch -q && git worktree add .claude/worktrees/ai-expat-quality -b feat/ai-expat-quality origin/feat/ai-quality
   cd .claude/worktrees/ai-expat-quality && echo ai > .role && git merge --no-edit origin/main
   cd backend && THESEUS_ROOT=/Users/brendankeane/Documents/GitHub/argon-hackathon1 uv run pytest -q
   ```
   Always pass `THESEUS_ROOT=` the main checkout: the vendored libexpat, the ASan build and the generated inputs live there (gitignored), not in the worktree.
2. **Key check:** `uv run python ../scripts/gemini_check.py`. Never print the key; never commit `.env` or `.env.example` with a value in it.
3. **Harness check, before any model call:** `scripts/preflight.sh expat` (C's required gate: canary, C vs C 1.0, reject-everything stub ≤ 0.05).

## Test plan, cheapest first

1. **Measure only, no model call.** Use the passing port as round 1, with the host fix on and polish off:
   ```bash
   uv run python -m ai.loop --target expat --model gemini-4-argon --rounds 1 --seed 1 --count 300 \
     --holdout-count 2000 --jobs 4 --timeout 20 \
     --start-from ../backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs \
     --out ../runs/ai/expat-quality/measure
   ```
   Record fmt, clippy warnings, panic lints, `arithmetic_side_effects` and whether `host_fix` was kept. This is the baseline row. Expect hundreds of `indexing_slicing` sites in a byte-level parser; that is the honest starting point.
2. **Turn polish on for libexpat** (prompt changes go in `prompts.py` only):
   - Add a lint rule to `EXPAT_SYSTEM_INSTRUCTION` mirroring cJSON's rule 7, worded generally, never describing specific inputs. Note that Flash would get the same words in any comparison.
   - Set `polish_rounds` for `EXPAT` in `targets.py` (3 is the cJSON value), or pass `--polish-rounds 3`.
   - Run polish rounds with **`--fresh-repair`, always**: the `previous_interaction_id` turn died at 65,416 output tokens on this file, and fresh calls worked twice (about 10 min each).
   ```bash
   uv run python -m ai.loop --target expat --model gemini-4-argon --rounds 4 --polish-rounds 3 --fresh-repair \
     --seed 1 --count 300 --holdout-count 2000 --jobs 4 --timeout 20 \
     --start-from ../backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs \
     --out ../runs/ai/expat-quality/polish
   ```
   About 10 min per round, about 40 min total. Start it by 1:45 so the result exists before the freeze.
3. **Independent recheck.** Adapt `backend/ai/results/cjson-quality/recheck.py` (it rebuilds and reverifies independently of `backend/ai`; read it first) for the chosen libexpat build, and send C the file so C can re-verify on a fresh seed as before.
4. **Commit** `backend/ai/results/expat-quality/` (summary.json, every round's verify, held-out and quality json, the chosen `expat.rs`, the diff from the passing port, and a README table: before vs after for each metric). Replace the local home path with `<repo>`, and grep for keys before committing. Open the PR from your branch; the user runs `git push` and `gh pr create --body-file <file>` (long `--body` strings get cut when pasted).

## Rules that keep the numbers honest

- Behavior first: a build is chosen by **loop** match rate, then lint counts. Held-out never steers selection; it is measured once, on the chosen build.
- Never grade our own homework: the AI side never writes inputs, the verifier or the counts.
- A polished file that lowers any match rate, adds a crash, or changes a single output byte is a failure, even if its lints are better. Report it as rejected; do not hide it.
- Do not silence `arithmetic_side_effects` by switching to wrapping ops; that can change behavior. Report the count only.
- If the panic-lint count cannot reach 0 within the budget, report the before/after counts honestly. "Default clippy 0, panic lints N → M, behavior unchanged" is a good result.
- Wording: "N tested behaviors preserved", never "100% equivalent". Every number traces to a json the harness or a tool wrote.
- Keep `translate()`, `repair()` and `polish()` signatures stable; C's `RealPipeline` calls them.

## Seams to watch

- `quality.measure` and `autofix` copy `backend/harness/expat/rust_crate` (`crate_template`). Check that clippy runs `--offline` and that diagnostics count only `expat.rs` spans, not `main.rs`.
- `clippy --fix` on 6.5K lines can rewrite many sites. `host_fix` rebuilds and reverifies, but read `round_N/host_fix/` to confirm what changed and that it was kept or rejected for the right reason.
- A whole-file polish reply is about 74K output tokens. If a polish reply is cut off (`incomplete`), add `--continue-turns 2` and record that continuation was used, in its own row.
- Usage reported 815,566 input tokens for a ~204K prompt on the first libexpat call. Track `usd` per round from summary.json and quote it, not an estimate.

## Third repo (only after libexpat quality is committed)

Recommendation: **md4c** (Markdown parser, MIT, about 8.7K lines / 78K tokens of C with `md4c-html`, heap-overflow CVEs CVE-2018-11536 and CVE-2018-11546, still found by OSS-Fuzz). Its `md2html` output is a deterministic stdout differential, and it has no well-known Rust port. Avoid libyaml (`unsafe-libyaml` is a published Rust port of it, so a match could be memorized), cmark (comrak) and zlib (zlib-rs). The vendor fetch, C driver, input generator, crate template and build script are **C's**. Ask Edan first with a short spec (scope, driver output format, input sources such as the CommonMark spec examples). Our side is then one `Target` entry in `targets.py` plus prompts in `prompts.py`, with `polish_rounds` set from the start.

## Report back

One table for the user and Edan: for libexpat, before vs after for fmt, clippy, panic lints, arithmetic (reported only), loop rate, held-out rate, crashes, rounds, seconds, `usd`. Name the summary.json each number came from.
