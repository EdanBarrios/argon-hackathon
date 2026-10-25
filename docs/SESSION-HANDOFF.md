# Session handoff (11:50 AM PT, Oct 25)

For a fresh Claude session (Edan's, role C) or any teammate. Read this, then `docs/PITCH.md`, `docs/DECISIONS.md`, `docs/FRICTION-LOG.md`. `docs/PLAN.md` is the original pre-event plan; where it conflicts with this file or DECISIONS.md, those win.

## Deadlines

- **Submit by 4:45** (hard deadline 5:00 PM). Submission form needs a 1-minute video of only today's work, public repo, all members added.
- Judging round 1: 5:00-7:00 PM, ~3 min live demo + 1-2 min Q&A. Top 6 go on stage 7:00-8:00 (equal weights).
- Feature freeze 3:30. Video at 4:00 (Keon).

## Rubric (official)

| Criterion | Weight | What it means for us |
|---|---|---|
| Working live demo | 50% | Runs live with **real inputs**; effective use of Argon / Managed Agents / companion models; **real verification loop** |
| Problem and usefulness | 25% | A **specific person** (libexpat / cJSON maintainers); could become a real open-source tool |
| Originality | 15% | **Only possible with Argon's 1M context / long decode**: libexpat in one pass, Flash as baseline |
| Clarity of pitch | 10% | Who the user is, working product, 3 min, no slides |

- Prize: Best use of Managed Agents, $5,000 (plus 1st/2nd/3rd).
- **Banned: "any project where a dashboard is the main feature."** Pitch Theseus as a migration tool with proof, never a dashboard.
- **DQ risk:** judges must see what we built today. README has a "Built today vs. used" table; say it on stage too.
- Never commit venue details (address, Wi-Fi) from the participant guide.

## Product in one line

Theseus rewrites C into memory-safe Rust with Gemini 4 Argon and proves it behaves the same: a deterministic differential verifier runs the C (under ASan) and the Rust on thousands of identical inputs and compares bytes. Divergences go back to Argon. Argon never grades its own homework. A human approves before a PR opens.

Capability ladder for the pitch: cJSON (~3K lines, works) → Flash vs Argon baseline → libexpat (~20K+ lines, one long-context pass, the Argon-only claim). Only claim "smaller models can't" if the data shows it.

## Team and rules

| Who | Role | GitHub | Owns |
|---|---|---|---|
| Edan | C, systems, merges | EdanBarrios | `backend/api/`, `backend/harness/`, `scripts/`, shared files |
| Brendan | A, AI | Keanooo7 | `backend/ai/` |
| Keon | B, frontend | Keon-Amid | `frontend/` |

- Brendan is online and active: he merged PR #1 (held-out verification, managed-agent sandbox pre-run in `backend/ai/`) and PR #2 (harness: mutation check `mutants.py`, -O0 cross-check, `coverage.sh`, provenance in verify report) at 11:47. **He self-merged into harness files (C's area).** Agree with him that merges go through Edan, or at least get a heads-up.
- Keon status unknown (Wi-Fi). Frontend was built by an agent and merged; he takes over on return.
- Teammates work on `feat/<name>-*` branches. Never force-push.

## Proven so far (numbers you can quote)

- **Verifier self-test:** C vs C 2,000/2,000 plus 12,000 mutated, all 1.0, with a canary. Our first driver passed C-vs-C while broken, which is why the canary exists (friction log #1, best Q&A story).
- **Argon on cJSON:** first translation, no repairs, **2,000/2,000** (735 lines of Rust, no unsafe, compiled first try). One call took **355 s** at thinking "high" (34K in, 7K out, 31K thinking). Files: `runs/ai/argon-1/`.
- **Real CVE:** CVE-2023-53154 (heap over-read, cJSON v1.7.17, fixed 1.7.18). Input `{"1":1,` crashes v1.7.17 under ASan (`heap-buffer-overflow cJSON.c:786 in parse_string`); **Argon's Rust returns `PARSE_ERROR at 7`**; verifier labels it `safety_improvement`. `backend/harness/oldcve/`. Use `--timeout 20` (macOS ASan symbolization is slow).
- **Sandbox (managed agents):** works with apt mirrors allowlisted (`scripts/sandbox_smoke.py --apt`): clang 18 + ASan + Rust + harness, ~46 s setup per new sandbox, files persist across turns via previous_interaction_id + environment_id.
- **Gemini key works:** Argon ~14 s minimum latency, Flash ~2 s. Key in `.env` (gitignored). Billing shows no balance (internal account); track cost via `GET /jobs/{id}/cost`.

## Decisions (full list in DECISIONS.md)

Target cJSON v1.7.19 core parse+print · runner **hybrid** (live demo local, pre-runs and PR step in sandboxes) · repair thinking medium, planner high · demo crash = CVE-2023-53154 · Edan temporarily covered A and B while they were offline.

## In flight when this was written (background agents from the previous session)

Their results land as commits on main and files under `runs/`. Check `git log` and these paths instead of waiting on notifications.

| Work | Output to look for |
|---|---|
| Real pipeline (`THESEUS_MODE=real`), `POST /try` "Try any input" endpoint + UI panel, home page reframed around the action | commits touching `backend/api/`, `frontend/`, `contracts/README.md` |
| Managed Agents in product: parallel Argon vs Flash sandbox runs, local re-verify, human-approved PR step (`--dry-run` until a token exists) | `backend/harness/remote/`, `scripts/run_remote.py`, `runs/remote/`, `backend/harness/remote/README-SECTION.md` |
| Adversarial inputs (targeted + Argon-written) vs Argon and Flash Rust | `backend/harness/adversarial/`, `runs/adversarial/` (partial results already there) |
| libexpat harness (driver, Rust template, inputs, self-test, size in lines/tokens) | `backend/harness/expat/`, `scripts/*expat*` |
| AI loop second Argon run + Flash baseline on cJSON | `runs/ai/argon-2`, `runs/ai/flash-1`, `runs/ai/flash-2` |

## Next steps, in priority order

1. **Live demo (50%):** confirm the real pipeline + Try panel work end to end; rehearse the 3-minute flow in `docs/PITCH.md`.
2. **libexpat long-context run** with Flash attempting the same, as soon as the expat harness lands (Brendan should kick it off; long decode, `background=True`, `max_output_tokens` > 65536). Record lines, tokens, match rates.
3. **Baseline numbers:** Flash vs Argon on cJSON (and libexpat), ideally 3 runs each. Fill `[numbers]` in PITCH.md and the README Results section.
4. **Managed Agents:** pre-runs in sandboxes; PR step live once the token exists; fill the README "How Theseus uses Managed Agents" section from `backend/harness/remote/README-SECTION.md`.
5. **Q&A readiness:** Brendan and Keon do the drills page and read their code. Fact-check every `[verify]` line in PITCH.md (Codex was asked to).
6. Merge any `feat/*` branches, then feature freeze at 3:30, video at 4:00, submit by 4:45.

## Waiting on Edan (human-only)

- **Create a fine-grained GitHub token** for the PR step: repo `argon-hackathon` only, Contents read/write + Pull requests read/write. Put it in `.env` as `GITHUB_TOKEN=` (sent to the sandbox via the Credentials API, never in prompts).
- Paste Codex's fact-check / red-team / Q&A results into the session.
- Get Keon online (hotspot) and onboarded: `docs/ROLE-BRIEFS.md`, drills page.

## Links and local state

- Team learning page (product explainer + 6 verifier drills, shared by link): https://claude.ai/artifact/VDZ7twqnjoaK9GmgkQX2Rg. Its source is in the previous session's scratchpad. To edit it from a new session, use the Artifact tool with this URL (read first, then publish with `url`).
- Servers running on Edan's laptop: mock API on :8000 (`THESEUS_MODE=mock`, from `backend/`) and Next.js on :3000, **started from an old agent worktree** (`.claude/worktrees/agent-a4d1f0163b1163b8b/frontend`). Restart the frontend from `~/dev/argon-hackathon/frontend` now that it's merged.
- `.claude/worktrees/` is excluded locally via `.git/info/exclude`; never commit it.
- Old pre-event folder `~/dev/argon-hackathon-pre-event` must never be read or copied from (compliance).

## Gotchas

- Verifying 2,000 inputs takes ~3.5 min (ASan startup on macOS); repair loops use ~300.
- `build_rust.sh` rejects the word `unsafe` anywhere (comments too); the crate also has `#![forbid(unsafe_code)]`.
- cJSON prints non-finite numbers as `null`; `[9007199254740993]` prints `[9.00719925474099e+15]` (a good Q&A example).
- Several agents commit to main concurrently: stage explicit paths, `git pull --rebase --autostash` before push.

## How Edan wants to work

Long builds run in background agents so chat stays open. Before starting, say in simple bullets what will be built and what human steps he should take meanwhile. Short prose, no em-dashes. Interrupt him when a decision is needed or the next dev step is ready.
