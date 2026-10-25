# Session handoff (2:40 PM PT, Oct 25)

For a fresh Claude session (Edan's, role C = systems, merge owner). Read this first, then `docs/PITCH.md` (script + Q&A) and the team brief: https://claude.ai/artifact/PT4JyioLP6iVd4NENGQzDw (shared by link). `docs/DECISIONS.md` has the history. The previous handoff (11:50) is in git history; this one supersedes it.

## Clock

- **Video at 4:00** (Keon records; 1 minute, only today's work, real footage, royalty-free music; CapCut and OBS are installed; shot list was sent to Keon).
- **Submit by 4:45**, hard limit 5:00: video, public repo, all members added. Everything the judges should see must be merged to main before submitting.
- Judging 5:00-7:00 (~3 min live demo + Q&A), top 6 on stage 7:00-8:00.
- Edan removed internal hard stops ("I say when we're done"). Feature freeze for the demo path is still wise: don't change backend/api or the live run after rehearsal.

## Team

| Who | Role | Owns | Now |
|---|---|---|---|
| Edan | C, systems, merges, **presenter** | `backend/api/`, `backend/harness/`, `scripts/`, shared files | Should be rehearsing |
| Brendan (Keanooo7) | A, AI | `backend/ai/` | Second libexpat translation (n=2) started 14:31, ~18 min per round; parallel workstreams handoff given (W2 n=2, W3 Flash with continuation, W4 Argon writes its own harness for a new library, W5 port anatomy) |
| Keon (Keon-Amid) | B, frontend | `frontend/` | Built an editorial "story page" home + restyled run view on `feat/ui-keon` (6 commits, **NOT merged**). Needed for the demo: XML toggle on the Try panel and the claim-vs-check tick; unconfirmed |

Rules: merges go through Edan; `git pull --no-rebase` before push (`--rebase` flattens merges); never force-push; never print `.env`; no em-dashes; `.role` is gitignored, so worktrees need `echo <role> > .role`.

## The idea (one sentence)

AI can rewrite old C into safe Rust, but nobody can trust the rewrite. Theseus runs old and new code side by side on thousands of inputs, catches every difference, has Argon fix it, re-checks (Argon never grades its own homework), and hands a human a PR with evidence.

Three proofs: (1) it works on something big (libexpat), (2) it catches what tests miss (Flash's cJSON port passes 2,000 tests, wrong on a 9-byte input, caught and repaired live), (3) nothing is trusted blindly (sandbox does the work, host re-checks, human approves, PR carries evidence).

## Measured results (all on main; say only these)

- **libexpat (parser Python and Firefox use), Argon:** one reply, ~18,500 lines of C (18,494) to 6,554 lines of safe Rust (1,089 s). Two compile-only fixes, 3 lines, no behavior fixes. Matched C on every input tried: 747 loop inputs (W3C suite + edge cases, 285 well-formed) and 1,253 unseen mutations, of which **only 52 are well-formed** (compared event by event); the other 1,201 match on exact error code, line, column. Host re-check (seed 4242) and a managed-sandbox re-check from scratch (seed 7777, sandbox claim 1.0 = host 1.0) agree. Flash at thinking high hit its 65,536 cap after 2,620 output tokens; retry DEADLINE_EXCEEDED (maybe infra). n=1 translation (fresh-a and fresh-b repaired the same translation). `results/libexpat-host-reverify/`, `results/libexpat-sandbox-verify/`, `backend/ai/results/`.
- **Coverage:** seed-4242 inputs run 52.2% of parser lines, 44.3% of branches; ~74.7% of lines reachable in our configuration. `results/libexpat-coverage/`.
- **Provenance:** Argon's port shares 7% of 8-token sequences / 3.6% of lines with the published `expat` crate (published 2026-10-18), mostly from the shared C; a real copy scores 99.8%. Rules out copying, not training exposure. `results/libexpat-provenance/`.
- **Real Python on Argon's Rust:** CPython 3.13.5's unmodified pyexpat + _elementtree pass **646/780** Python XML tests (stock 768, C control 768, parse-nothing stub floor 392, so 254 from real parsing). Failures: namespaces 71 (by design, clean error), streaming 32, DTD 14, non-UTF-8 encodings 12. Shim `ffi.rs` written by Argon in one call, compiled first try. **Q&A only, not on stage**: the unsafe FFI layer has a known aliasing issue; namespaced XML fails. `results/pyexpat-on-argon/`, `contracts/pyexpat-shim.md`, `backend/ai/shim/`.
- **cJSON:** Argon first-try port 4/4 runs; Flash 0/4 first try (cap). Live repair from Flash's port: round 1 ~96%, 19 divergences all from `"\uZ\t\\"`, Argon repairs (~90 s), round 2 100%, held-out passes; total 146-174 s (last run 174 s). Targeted split 980 vs 961 is one sample (Argon also misses the same 19 in 3/4 ports): never present as Argon beating Flash.
- **CVE-2023-53154:** `{"1":1,` crashes cJSON 1.7.17 under ASan; Rust returns PARSE_ERROR at 7.
- **Managed Agents:** Argon repair inside a sandbox (517 s, 0.9933 then 1.0, host re-verified); proxy-injected GitHub token (fake-token test: sandbox never sees it); real PR #3 (from an earlier run); certificate PR format (EVIDENCE.md, certificate.json, reproduce.sh tested from a fresh clone); preflight gate before any sandbox model call.
- **Quality gate (Brendan):** cJSON 0 clippy / 0 panic lints. libexpat "153 -> 0 panic lints" is partly metric gaming (18 `unwrap_or(0)` fallbacks); don't sell it as a safety win.

## Never say

"proves", "equivalent", "mathematically", "cryptographic", "one shot", "2,000 fresh inputs", "we rewrote the parser inside Python", "Flash can't", "we found a bug in cJSON" (already documented), the 815K input-token figure, "libexpat was translated in a sandbox" (only the re-check ran there), "Python runs on Argon's expat" on stage.

## Known gaps (admit first; answers are in PITCH.md Q&A)

No planner (one-file translation; Track 1 says "plan the migration"). Repair has no edit tool (each fix regenerates 6.5K lines). Held-out is mostly error-position tests (52 well-formed). No mutation testing on libexpat. Hash-flooding untested (fixed salt). Banned-API guard is a text scan. Parsers are the easiest domain. n=1.

## Live state

- API: real mode on :8000, pid 11891, started from `backend/` with `THESEUS_MODE=real uv run uvicorn api.main:app --port 8000` (log in the earlier session's scratchpad). In-memory job history; restarting wipes "Load pre-run". A finished live job `job_be9796` exists. Endpoints: `/try` (cjson + expat), `/try/presets?library=expat` (rich, billion_laughs), `/results/libexpat`, `/remote-runs`, `/jobs`.
- Frontend: Next dev on :3000 from `frontend/` (pid 46786), serving main's frontend (not Keon's unmerged story page).
- `runs/shim/libtheseus_expat.dylib` built on this Mac (x86_64). Argon libexpat binary: `runs/build-expat/edan-reverify-a/`.
- Backend tests: 63 pass (`cd backend && uv run pytest -q`).

## Open items, in order

1. **Keon's `feat/ui-keon`** (story page home + restyle, 29 files): review before merging. It must still have the Try panel with XML toggle, the live run view and the claim-vs-check tick, all real data, not dashboard-like. Merge only if the demo flow in PITCH.md still works end to end.
2. **Brendan's branches** as they land: n=2 translation, Flash-with-continuation, Argon-writes-harness, anatomy. Merge results; update README/PITCH numbers only from measured files.
3. **Brendan's lane fix:** restate the polish result honestly and drop "How to say it" blocks in `backend/ai/results/`.
4. **Rehearsal:** Edan, stopwatch, script in PITCH.md; practise the 174 s fallback line.
5. **Video at 4:00**, then merge everything, final README pass, submit by 4:30.
6. Leftovers: PR #3 is open (do not merge; it's the demo PR). `feat/ai-main` has one stale decisions commit.

## How Edan wants to work

Short prose, no em-dashes, plain language. Long work goes to background agents; say what's being built and what he should do meanwhile. Be brutally honest when asked. He decides when work stops.
