# Session handoff (11:50 PM PT, Oct 25, after judging)

For a fresh Claude session (Edan's, role `systems`, merge owner). The hackathon is over: submitted, demoed, judged. Results not announced to us yet. This supersedes the 2:40 PM handoff (in git history). Not committed: this file is a working-tree change on `main`.

Read next: `docs/PITCH.md` (Q&A), the team briefing doc https://claude.ai/artifact/BYrxV8WECukdAR46sbGgd7 (roles, demo script, numbers, Q&A, judge prep; private unless Edan shared it), `docs/DECISIONS.md`.

## What happened

- Video submitted: https://youtu.be/OqdBqDwA-PM (49 s; title and description set by Edan).
- Demo went badly in Edan's view. Judges (Google DeepMind) took a photo ("I want to remember this") and asked how test cases were generated and how we handle flaky tests. Read as engaged questions, not a verdict.
- Next goal Edan raised: get the repo ready for GitHub show-and-tell, Reddit (r/rust) and maybe Show HN.

## State of the repo

- Local `main` is 3 commits behind `origin/main` (Brendan's Lua PR #13). `git pull --no-rebase` before any work.
- Repo is public; collaborators EdanBarrios, Keanooo7 (Brendan), Keon-Amid. Only open PR: #3 (the example certificate PR; keep).
- No secrets in git history (checked: no `.env` commit, no `ghp_` or `AIza` keys). **The GitHub token used by the sandbox still needs revoking.**
- Backend tests: 66 pass (`cd backend && uv run pytest -q`, about 3 min). Under heavy machine load the billion-laughs test times out; it passes alone.

## Landed today after 2:40 (all on main)

- Keon's UI: editorial story home page, two-plate run view, XML/JSON Try panel, run banner with timer, animations. Home page smooth scrolling (Lenis) removed by Edan's session because it fought the `#run` anchor (`frontend/components/story/smooth-scroll.tsx` returns null).
- Brendan, n=2: a second independent libexpat translation, 6,518 lines, 1-line compile fix, then 747/747, held-out 1,253/1,253, seed 4242 2,000/2,000 (341 event by event, 1,659 on error code/line/col). `backend/ai/results/expat-long-decode/argon-translate-2/`.
- CVE replay: CVE-2024-8176 (nested entity chain) crashes libexpat 2.6.4 C with an ASan stack overflow; Argon's Rust output is byte-identical to fixed 2.9.0 C. CVE-2022-25313 not reproduced with our input (stated). 71 CVEs listed upstream, most out of our port's scope. `results/libexpat-cve-replay/`.
- Brendan, Lua 5.4.7 (PR #13, merged after judging): Argon wrote 17,430 lines of Rust in one reply (191K output tokens). 2 borrow-check errors fixed **by hand** (9 lines). Informal smoke test 22/22 vs C. There is **no Lua harness or verifier run yet**: never present this as verified. `backend/ai/results/lua-long-decode/`.
- Token accounting doc and Lua target/prompts (PRs #9, #10).

## Unfinished, not on main

- Coverage-guided differential fuzzing: stopped mid-run, no write-up. Partial scripts in worktree `.claude/worktrees/agent-acf1acf613b0443fd/results/libexpat-diff-fuzz/` (branch `feat/systems-diff-fuzz`, unpushed). Its last C timeouts happened under load average ~590 and are not findings.
- Module swap (xmlrole.c replaced by Rust, linked with the other two C files): stopped while writing the FFI wrapper. Partial work in `.claude/worktrees/agent-a1d109615cb40cf91/backend/harness/expat/module_swap/` (branch `feat/systems-module-swap`, unpushed).
- Fable verifier audit: stopped before reporting.
- Codex claims audit (finished): `/private/tmp/claude-501/-Users-edan-dev/d0c3bd5f-086a-4fab-bb7e-f2921aa59d96/scratchpad/codex-claims-audit.md` (scratch, may vanish). Main findings below.
- Local throwaway branch `review/ui-keon` and scratch worktree `keon-wt`: safe to delete.

## Show-and-tell plan (not started)

Must (2-3 h):
1. Add LICENSE (MIT) and credit cJSON (MIT) and libexpat (MIT) as third-party inputs.
2. Fix fresh-clone bug: `backend/api/harness.py:183` runs `build_cjson.sh` without `fetch_cjson.sh` first. Then test a clean clone end to end (mock mode needs no API key).
3. README rewrite: one-line pitch, demo GIF from the video, honest results table (add n=2, CVE replay; Lua only as "translated, not yet verified"), limitations near the top.
4. Codex audit fixes: README line ~79 describes a 514-input / 96.3% / 166 s run that has no saved file (saved run: 578 inputs, 19 divergences, 146 s). Remove from PITCH: "your CI would merge it", "2,000 fresh inputs", "every behavior change / every claim re-checked", "kept the security limits", "that whole class of bug can't be written in safe Rust". Several sandbox numbers (517 s, 49 shell calls, token-proxy test, PR #3) have no saved evidence file; add one or soften.
5. Revoke the GitHub token.

Should (3-4 h):
- One no-API-key command that builds the committed Argon libexpat port and verifies it against C on held-out inputs (anyone with clang + cargo can check 2,000/2,000).
- State platforms: only macOS x86_64 tested.
- Label known weaknesses: pyexpat bridge aliasing issue, 18 `unwrap_or(0)` fallbacks, fuzzing/module swap/Lua as "next".
- Delete stale branches; move hackathon docs to `docs/hackathon/`.

Posting: lead with the verifier, not the AI. Expect r/rust to raise "AI slop", c2rust, the existing `expat` crate, "most inputs are errors", `unwrap_or(0)`; answers are in the briefing doc's Q&A.

## Answers to the judges' questions (for the README)

- Test generation: libexpat uses 747 fixed inputs (W3C conformance suite plus edge cases) and seeded mutations; held-out seeds (1001, 4242, 7777) are never seen by the loop, overlap removed. cJSON uses generated JSON plus targeted inputs (the `str_` set caught Flash's 19). The C is the answer key; no hand-written expected outputs.
- Flakiness: preflight requires C vs C = 1.0 (and a reject-everything stub <= 0.05, plus a canary) before any model call. Nondeterminism removed at the source (libexpat hash salt fixed to zero). How the verifier counts timeouts is **not yet checked**; check `backend/harness/verify.py` before claiming anything.

## Never say

"proves", "equivalent", "mathematically", "one shot", "2,000 fresh inputs", "Argon beats Flash", "Flash can't", "we found a bug in cJSON", "libexpat was translated in a sandbox", "drop-in replacement", "secure", "Lua is verified".

## Live state on Edan's Mac

- API on :8000 in real mode (pid 11891), frontend dev on :3000. Fine to stop now; restarting the API wipes in-memory job history (jobs `job_be9796`, `job_532323`, `job_29a12b`; their files persist under `runs/ai/live-*`).
- Desktop files from today: `keon-verify-checklist.md`, `theseus-team-briefing.md`, `theseus-text-cards-and-voiceover.md`, `theseus-live-demo-clicks.md`, `theseus-closing.png`.

## How Edan wants to work

Short prose, plain language, no em-dashes. Ask before outward-facing actions. Don't overload the machine: earlier, four parallel agents plus fuzzing froze it (load ~590). Run one heavy job at a time. Edan decides when work stops.
