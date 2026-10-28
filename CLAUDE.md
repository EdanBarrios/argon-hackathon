# Theseus: agent rules

Read `README.md` first. `docs/PLAN.md` is the original hackathon-day plan and is kept for history.

## Working rules

- Work on a feature branch and open a PR. Never force-push to `main`.
- No broad refactors. Never revert someone else's work.
- Do not change `contracts/` silently; say so in the PR.
- Never commit secrets (`.env`), fetched third-party sources, or `runs/`.

## Before committing

1. `git status`
2. `git diff`
3. Run the tests for the area you changed (`cd backend && uv run pytest -q`; the billion-laughs test can time out under heavy machine load).

## Claims

Say "N tested behaviors preserved", never "proves", "equivalent", "secure" or "drop-in replacement". Every number in the README or docs needs a saved file under `results/` or `backend/ai/results/`. Lua is translated, not verified.

## Argon

Argon API facts are in `docs/PLAN.md` section 6. Use the Interactions API only.
