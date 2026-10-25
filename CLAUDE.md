Read docs/PLAN.md first.

# Theseus: agent rules

## Role detection

Read `.role` in the repo root. Valid values: `ai`, `frontend`, `systems`.
If `.role` is missing or invalid, this session is **read-only**: do not edit files.

## Ownership

| Role | May edit |
|---|---|
| `ai` | `backend/ai/` |
| `frontend` | `frontend/` |
| `systems` | `backend/api/`, `backend/harness/`, `scripts/`, and all shared files: root files, `docs/`, `contracts/`, `.claude/`, `backend/pyproject.toml` |

Anyone may edit `docs/DECISIONS.md`.

## No-overlap rules

- Never edit another role's paths.
- If the guard blocks you, stop and ask the owner. **Never work around it via bash.**
- Never change the contract silently. Announce first; C edits `contracts/`.
- No broad refactors.
- Never revert others' work.
- Never force-push.
- Merges go through C.
- Work on a `feat/<role>-*` branch.

## Before committing

1. `git status`
2. `git diff`
3. Run the tests for your area.

## Argon

Argon API facts are in docs/PLAN.md section 6. Use the Interactions API only.
