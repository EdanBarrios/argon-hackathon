# cJSON in managed-agent sandboxes: Argon and Flash in parallel

Evidence copied from the gitignored `runs/remote/20261025-115152/` (Oct 25 2026, `scripts/run_remote.py --models gemini-4-argon,gemini-3.8-flash --rounds 4`). Input directories are omitted; they are regenerated from seeds. Absolute paths inside the JSON refer to the original machine.

| | Gemini 4 Argon | Gemini 3.8 Flash |
|---|---|---|
| Sandbox wall time | 517.2 s | 1,827.3 s |
| Input tokens (cached) | 350,463 (274,775) | 3,986,791 |
| Output tokens | 25,959 | 22,400 |
| Cost (from usage) | $0.49 | $1.36 |
| Rounds claimed by the agent | 2: 0.9933, then 1.0 | 1: 1.0 |
| Code-execution calls in the transcript | 4 | 49 (plus 37 thought steps) |
| Sandbox verify / host rebuild, same 300 inputs / host held-out (64) | 1.0 / 1.0 / 1.0 | 1.0 / 1.0 / 1.0 |

Notes:

- Argon's round 1 scored 0.9933 because it rejected `"\uZZZZ"`, which C parses as `""`. Round 2 fixed it.
- Flash reported one round, but its transcript shows 49 code-execution calls of edit and rerun inside that round. The agent's round count is a claim; the transcript is the record.
- The host rebuilt each returned `cjson.rs` and reran `verify.sh` itself. Claim and host check agree for both models.
- n = 1 per model, one run.
