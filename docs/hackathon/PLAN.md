# Theseus: Day-of Handoff

Gemini 4 Argon SF Hackathon · Sun Oct 25 · Hacking 10:30 AM–5:00 PM PT
Repo: fresh, public `argon-hackathon`. Empty until 10:30. Every file in it is written during the event.

---

## 1. Before 10:30: each teammate (no code yet)

- [ ] Accept the GitHub collaborator invite on the new empty repo, then run `gh auth login`
- [ ] Have this file (Edan AirDrops it) and read it
- [ ] Install the Gemini skills: `npx skills add google-gemini/gemini-skills --skill gemini-api-dev`
- [ ] Make sure Claude Code is signed in
- [ ] Install the local toolchain (needed for the local fallback runner): `rustup` (stable), `clang` (Apple clang works; check `-fsanitize=address` compiles hello world), Node 20+, Python 3.11+, `uv`
- [ ] Phone hotspot ready
- [ ] Ask the Google mentors / `#google-deepmind` the sandbox questions (section 6)

---

## 2. What we're building

**Theseus migrates C to memory-safe Rust and shows evidence that behavior is preserved.**

Pitch opener:
> "On Tuesday, two memory bugs in PHP's C core (PHP2Shell: use-after-free and memory corruption) put most of the web one HTTP request away from takeover. Safe Rust makes that whole class of bug impossible. Everyone agrees critical C should move to Rust. Nobody does it, because you can't prove the rewrite behaves the same. Theseus rewrites it and shows you the evidence."

- **Track 1: Million Token Engineering.** Lead Argon reads the whole library to plan the migration.
- **Target for the $5K Managed Agents prize too:** the repair loop runs in Antigravity sandboxes.
- **Wording rule:** say "N tested behaviors preserved," never "100% equivalent."
- **Baseline, as a 2×2 comparison through the same harness, prompts, and repair budget:** {Flash, Argon} × {single pass, with repair loop}. Metrics: match rate, behaviors tested, rounds to 100%, crashes, time, cost. Run the small-module comparison by ~1:30 PM and report honestly whatever comes out. If Flash keeps up on small modules, the story becomes "Flash handles leaf modules; Argon is needed for whole-library planning and cross-module consistency" (Google's own recommended pattern).
- **The Argon-only claim must be demonstrated, not asserted:** start one full-library run in a single long-context pass (`background=True`, long decode) by ~1:00 PM, with Flash attempting the same. The gap at that scale is the originality proof.
- Don't claim we migrated PHP. PHP is the reason the tool matters, not the demo target.
- **The verifier is the product.** The judges value deterministic verifiers and taste over how many tokens or agents you burn. Never brag about agent counts or token volume. Show what was verified, and what a human no longer has to check by hand.
- **Friction log:** keep `docs/FRICTION-LOG.md` all day. Record every real divergence and failure: what Argon got wrong, how the verifier caught it, how it was fixed. It goes in the README and the Q&A.
- **Know your code.** Each engineer must be able to explain their piece line by line in Q&A. Don't merge generated code you haven't read.
- Don't use TraceGrid code. Visual inspiration only.

**Ambition ladder (same engine, three targets):**
- **Floor (live demo):** one module of a small parser (cJSON or inih). Must work.
- **Target (pre-run):** the full small library, with the Flash vs Argon comparison.
- **Moonshot (background, starts ~12:30 once the single loop works):** a famous, security-critical C library with no well-known Rust port (avoids "it memorized the port"). Lead candidate: **libexpat** (the XML parser inside Python, Firefox, Apache; long CVE history). Its `xmlwf` CLI is a ready-made differential driver: same XML in, compare canonical output. Brendan kicks it off as one long-decode run; report whatever match rate it reaches, honestly. The moonshot never blocks the floor.
- Framing: DARPA's TRACTOR program exists to automate C→Rust translation. "We did a verified slice of it today, with evidence."

**Target library: decide at kickoff.** Requirements: real and widely used, small (a few thousand lines), a parser (JSON/INI/URL), a permissive license, a CLI or simple API we can drive with test inputs, and ideally a security history. Candidates to check: cJSON, inih.

---

## 3. Architecture

```
UI (Next.js) ──polls──▶ API (FastAPI /jobs) ──▶ Lead Argon (Interactions API, thinking=high)
                                                 │  reads whole library, plans modules in dependency order
                                                 ▼
                                   One managed agent per module, in parallel
                                   (antigravity-preview-09-2026, model gemini-4-argon)
                                   translate → verify.sh → read divergences → repair → repeat
                                                 ▼
                                   Final agent: whole-library differential run
                                                 ▼
                                   Human clicks "Open PR" → agent pushes crate + evidence report
                                   (GitHub token via Credentials API; network allowlist)
```

**One verify script, two runners.** `verify.sh` builds the C version (with ASan) and the Rust version, runs both on the same generated inputs, and prints JSON (match rate, divergences, crashes). It runs **remotely** inside the sandbox (main path) or **locally** (fallback). If the sandbox is slow or flaky, switch runners. The live demo always works.

---

## 4. Roles and first tasks

| Role | Owns | First task at 10:30 | By 12:30 |
|---|---|---|---|
| **A: AI** (`backend/ai/`) | Lead planner, managed-agent prompts, repair prompts | 10:35 sandbox smoke test: one managed-agent call that installs rustup and clang, compiles hello world in both with ASan, and prints versions and timing | One function translated and repaired from divergence feedback |
| **B: Frontend** (`frontend/`) | Run view, agent lanes, divergence table and trace, score chart, video and README | Build from the sample data: agent lanes and divergence table | UI driven by the real `/jobs` API |
| **C: Systems** (`backend/api/`, `backend/harness/`, `scripts/`) | `verify.sh`, input generator, ASan, `/jobs` API, merges, deploy | Run BOOTSTRAP, then `verify.sh` working locally on one function. **Self-test first: run C against C. It must report 100% match before any Rust exists**, so we know the verifier itself is trustworthy. | Thin slice end to end: UI → API → Argon → verify → UI |

**11:15 decision:** if A's smoke test took under ~3 minutes and worked, the sandbox is the main path. Otherwise run locally, and use managed agents only for the final verification and the PR step.

Rules: stay in your own folders. Only C edits shared files and the contract. Announce contract changes before coding against them. (A guard hook that enforces this is optional. C adds it after the 10:30 basics only if it takes under 15 minutes.)

---

## 5. 10:30 bootstrap (C, in order)

1. Clone the empty repo. First commit: `.gitignore` (`.env*` except `.env.example`, `.role`, `node_modules/`, `.venv/`, `target/`, build output), then push. **Nobody creates `.env` before this.**
2. Commit this plan as `docs/PLAN.md`, plus a short `CLAUDE.md` (read docs/PLAN.md first, role ownership from section 4, no-overlap rules, Argon cheat sheet pointer).
3. Write `.env.example`, folder READMEs, and the sample JSON (`contracts/fixtures/`) with the fields below. Push.
4. Teammates clone and run `echo <ai|frontend|systems> > .role`. B scaffolds Next.js (use system fonts; Google Fonts breaks on bad Wi-Fi). C scaffolds FastAPI.
5. Everyone works on a `feat/<role>-*` branch.

**Contract fields to add for Theseus:**
- Job: `agents: [{id, module, status, iteration, match_rate, interaction_id}]`
- Results: `baseline_match_rate`, `final_match_rate`, `behaviors_tested`, `divergences: [{input, c_output, rust_output, function}]`, `crashes: [{input, sanitizer_report}]`
- `approval: {required, approved_by, pr_url}`

---

## 6. Argon cheat sheet

- **Interactions API only:** `client.interactions.create(model="gemini-4-argon", input=..., generation_config={"thinking_level": "high"})`. Read output with `interaction.output_text`. Use SDK `google-genai` / `@google/genai` ≥ 2.3.0.
- No `thinking_budget`, `temperature`, `top_p`, `top_k`, or `cached_content`.
- Output over 64K tokens: `background=True`, `max_output_tokens` > 65536, then poll `client.interactions.get(id=...)`.
- HTTP timeouts 15–20 minutes, or use background. The backend never keeps a request open waiting on Argon. The UI polls our `/jobs`.
- **Managed agent:**
  ```python
  client.interactions.create(
      agent="antigravity-preview-09-2026",
      agent_config={"type": "antigravity", "model": "gemini-4-argon"},
      input=...,
      environment={"type": "remote", "network": {"allowlist": [...]}},
  )
  ```
- Secrets go through `client.credentials.create` and never into prompts.
- Price: $2 / 1M input tokens, $10 / 1M output tokens.

**Questions for the mentors before 10:30:**
1. Does the sandbox keep files and tools across `previous_interaction_id` turns?
2. Are clang/rustc preinstalled? Can it reach crates.io and apt?
3. How do we get files in: git clone from an allowlisted host, or upload?
4. Typical latency per call? Any limit on parallel agents?
5. Anything specific judged for the Managed Agents prize?

---

## 7. Timeline

| Time | Milestone |
|---|---|
| 10:30–10:50 | Bootstrap repo, pick target library, approve contract |
| 11:15 | Sandbox decision: remote or local runner |
| 12:30 | **Thin slice end to end on one function** |
| Hourly | Integration checkpoint, max 15 min, C merges |
| 3:30 | **Feature freeze.** Bugs and demo only. |
| 3:30–4:15 | Demo prep: pre-run the full library; pick one small module that completes live in under 90s |
| 4:00–4:30 | B records the 1-min video (built-today work only, ~10s on sandbox lanes); README includes "How Theseus uses Managed Agents" |
| 4:45 | Submit: repo public, video link works, all members added |

**Build priority if behind:** verifier → translation → repair → UI → baseline → Managed Agents polish → extras. Start with one module and one agent; fan out to parallel agents only after the single loop works.

---

## 8. Demo (3 min, no slides)

1. **0:00** PHP2Shell opener (one sentence) and who the user is: maintainers of critical C code.
2. **0:20** Start a live run on one small module. Agent lanes light up.
3. **0:50** A divergence appears in red. Click it to see the input, C output vs Rust output, and the function where they split. Argon repairs it, and the match rate climbs.
4. **1:40** Switch to the pre-run full library: baseline vs final match rate, behaviors tested, and an ASan crash in C where Rust returns a clean error.
5. **2:20** Approve → PR opens with the evidence report.
6. **2:40** Close: "N tested behaviors preserved, in isolated sandboxes. Argon never grades its own homework."

**Q&A prep:** have one divergence from the friction log ready to explain in depth: why the C and Rust versions differed, and why the fix is right. That shows taste, not just output.

Rehearse twice before 5:00.

---

## 9. Rules that can DQ us

- Everything in the repo and demo is written during the event. The first commit is after 10:30. The only pre-written thing is this plan (notes, which the judges allow).
- Repo public. Never commit `.env`, keys, or venue details.
- Video shows only what was built today.
