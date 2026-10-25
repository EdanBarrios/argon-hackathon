# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Primary: product-security, embedded and appliance teams that ship a C parser inside their own product and want a memory-safe drop-in, with evidence their auditors can rerun. Second: upstream maintainers of those parsers, who need the same evidence before accepting a Rust rewrite.

Today, at the Gemini 4 Argon SF Hackathon (Oct 25, 2026), the run view is projected for judges during a live 3-minute demo and recorded for a 1-minute video. It must read instantly from the back of a room while remaining a real tool for engineers reviewing a migration's evidence. Confirmed priority: demo first, both audiences.

## Product Purpose

Theseus migrates C parsers to memory-safe Rust and hands back evidence a reviewer can check. Gemini 4 Argon writes the Rust; a deterministic verifier, not the model, decides what counts as a match. Success is "N tested behaviors preserved": the C and the Rust produced identical output bytes and exit status on N inputs through our driver, including held-out inputs the repair loop never saw.

## Positioning

Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them. Argon never grades its own homework; nothing ships without a person approving it.

## Operating Context

The run view (`/jobs/[id]`) is where a live run is followed: Theseus starts from a translation (Flash's saved first attempt, Argon's, or a fresh Argon translation), verifies it, sends deduped divergence patterns to Argon for repair, re-verifies, runs a held-out check and a CVE check, then waits for human approval, after which a managed agent opens a pull request with the crate and an EVIDENCE.md. The view polls `GET /jobs/{id}` and `/events` every second and falls back to the recorded run (`job_2fc7b6`) when the API is unreachable.

Confirmed jobs on the run view: follow the run live; inspect any divergence (input, C output, Rust output side by side); see Argon's repair and the verifier feedback it was given; see the held-out result; see the CVE input crash C under AddressSanitizer while Rust returns a clean error; approve to open the PR; compare Flash vs Argon; try any input against the real binaries (`POST /try`).

## Capabilities and Constraints

- Next.js 16 app in `frontend/` (owned by the frontend role); data logic in `lib/api.ts` and `lib/derive.ts` must be preserved.
- Venue Wi-Fi is unreliable: no runtime CDN or hosted fonts.
- The hackathon bans projects "where a dashboard is the main feature": the run view must read as a migration tool producing evidence, not a dashboard.
- Divergence function names are inferred by the pipeline; show them as "likely in".
- PR links in mock mode are placeholders.

## Brand Commitments

- Name: Theseus (the Ship of Theseus: replace every plank, is it the same ship?).
- Wording rules (docs/PITCH.md): say "N tested behaviors preserved" or "independently checks agreement on N tested inputs". Never "proves it behaves the same", "equivalent", "100% compatible", "secure", or "CVE fixed"; for the CVE input say "rejects this historical crashing input". Never call it a dashboard. Flash "hit its 64K output cap"; never say Flash "can't".

## Evidence on Hand

- Recorded real run `job_2fc7b6`: `public/fixtures/job-completed.json`, `job-events.json` (Flash draft 559/578 → Argon repair in 91 s → 578/578, held-out 64/64, CVE-2023-53154 ASan report).
- Flash vs Argon results: `data/baseline.json` (from `results/cjson-flash-vs-argon`).
- Real code: cJSON.c v1.7.19 and Flash's draft `parse_string` (`data/excerpts.ts`).
- libexpat results are pending: never fill them in. No customers, testimonials or benchmarks beyond the files above.

## Product Principles

1. The verifier decides, not the model. Every claim on screen traces to verifier output.
2. Show the evidence, not a score: the input, both outputs, and the fix.
3. Behavior-preserving means preserving the weird parts too.
4. A person approves before anything ships.
5. Say only what was tested; never generalize past the inputs.
