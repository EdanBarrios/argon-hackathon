# Theseus

Theseus rewrites C libraries into memory-safe Rust and proves the new code behaves the same. It runs the original C (under AddressSanitizer) and Gemini 4 Argon's Rust translation on thousands of identical inputs, compares their output byte for byte, and feeds every divergence back to Argon until they match. Argon writes the code; it never grades its own homework.

## Team

- Edan (systems: verifier, pipeline, API)
- Brendan (AI: translation and repair with Gemini 4 Argon)
- Keon (frontend: run view and demo)

## Built during the event

Built entirely during the Gemini 4 Argon SF Hackathon, Oct 25 2026. Hacking started ~10:13 AM PT per organizer announcement and ends 5:00 PM PT. The only pre-written file is the plan in `docs/PLAN.md` (notes).

### Built today vs. used

| Built today by our team | Used as inputs (third-party, not our code) |
|---|---|
| Differential verifier (`backend/harness/verify.py`), drivers, input generators, self-test with canary | [cJSON](https://github.com/DaveGamble/cJSON) v1.7.19 and v1.7.17 (MIT): the C library being migrated, fetched at build time, never committed |
| Rust crate template, safe-Rust build gate (`scripts/build_rust.sh`) | [libexpat](https://github.com/libexpat/libexpat) (MIT): the moonshot target, fetched at build time |
| Argon translate/repair loop (`backend/ai/`) | Gemini 4 Argon, Gemini 3.8 Flash, Antigravity managed agents (Google APIs) |
| Jobs API (`backend/api/`), managed-agent runner, PR step | FastAPI, Next.js, google-genai (open-source frameworks) |
| Run view UI (`frontend/`) | |

The Rust code in any results was written by Gemini 4 Argon during the event, through our pipeline.

## Results

_To be filled in from `runs/` and `docs/FRICTION-LOG.md`._

## How Theseus uses Managed Agents

_To be filled in._

## Run it

```
cd backend && uv sync && THESEUS_MODE=mock uv run uvicorn api.main:app --port 8000
cd frontend && npm install && npm run dev   # http://localhost:3000
scripts/selftest.sh                          # verifier self-test (C vs C, canary)
```
