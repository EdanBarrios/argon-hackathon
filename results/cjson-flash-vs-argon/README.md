# Flash vs Argon on cJSON v1.7.19 (parse + print)

Copied from `runs/` (gitignored) so the whole team can read them. Same prompts, same verifier, same inputs for both models. n = 2 runs per model.

| | Argon (gemini-4-argon) | Flash (gemini-3.8-flash) |
|---|---|---|
| Runs | argon-1, argon-2 | flash-1, flash-2 |
| First attempt at thinking high | 2 of 2 finished, 100% in round 1, no repairs | 0 of 2 finished: both ended `incomplete` |
| Why Flash stopped | n/a | Flash caps thinking + output at 65,536 tokens (we asked for 131,072). It spent ~63K on thinking and ran out mid-file |
| Eventual result | 2,000/2,000 | flash-2 auto-retried at thinking medium: 2,000/2,000. flash-1 had no retry yet |
| Time to a working port | 333 s, 355 s | 838 s (failed attempt + retry) |
| Total tokens | 66K, 71K | 189K (both attempts billed) |
| Thinking tokens | 26K, 31K | 112K |
| Rust lines (`cjson.rs`) | 735, 714 | 734 |
| Targeted adversarial inputs | 958/958 | 958/958 |
| Argon-written adversarial inputs | 225/225 | 225/225 |
| Divergences | 0 | 0 |

## How to say it

- On cJSON both models reach the same correctness. Do not claim Flash "can't".
- The difference: Argon did it in one pass at full thinking, about 2.5x faster end to end with about 2.7x fewer tokens. Flash hit its 64K output ceiling at high thinking and only finished by thinking less.
- Calling flash-1 a "failure" invites "just give it more room". It hit a token cap, not a translation mistake.
- The originality claim (bigger libraries need Argon) is a prediction until the libexpat runs exist.
- These runs predate held-out verification, so there is no held-out rate yet.

## Files

- `<run>/summary.json`: rounds, seconds, token usage, final match rate
- `argon-1/full2000_verify.json`, `argon-2/final_verify.json`, `flash-2/full2000_verify.json`: verifier reports on 2,000 inputs
- `<run>/cjson.rs`: the Rust each model wrote
- `adversarial/`: `*-targeted.json` are hand-targeted edge inputs, `*-argon.json` are inputs Argon wrote to break the port
