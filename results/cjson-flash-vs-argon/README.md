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
| Targeted adversarial inputs | 980/980 | **961/980 (98.06%), 19 divergences** |
| Argon-written adversarial inputs | 225/225 | 225/225 |
| Divergences | 0 | 19, one root cause (below) |

## Flash's divergence (the only real one we have)

Input `"\uZ\t\\"` (9 bytes). C prints `OK\t""`, Flash's Rust prints `PARSE_ERROR at 7`. In an array, `["\uZ\t\\",1]`: C `OK\t["",1]`, Flash `PARSE_ERROR at 8`.

Why: C's `parse_string` (vendored v1.7.19 `cJSON.c` line 877) reads a string in two passes. Pass one finds the end by pairing escapes. Pass two decodes: `\u` eats six bytes and invalid hex becomes U+0000, which leaves a lone backslash before the closing quote. C only checks `< 1` byte remaining, so it accepts. Flash checks `< 2` (its `cjson.rs` line 175): stricter, but not what C does. C is the reference, so Flash is wrong. Argon matched this quirk.

Numbers from 12:09 re-run; an earlier 958-input run had 32 inputs silently overwritten by macOS case-insensitive filenames, now fixed. Argon also matched 20,000 extra fuzz inputs.

## How to say it

- On the 2,000 generated inputs both models score 100%. On targeted edge inputs, Argon is 980/980 and Flash 961/980. One real behavior difference, n=1 port each. Do not claim Flash "can't".
- The difference: Argon did it in one pass at full thinking, about 2.5x faster end to end with about 2.7x fewer tokens. Flash hit its 64K output ceiling at high thinking and only finished by thinking less.
- Calling flash-1 a "failure" invites "just give it more room". It hit a token cap, not a translation mistake.
- The originality claim (bigger libraries need Argon) is a prediction until the libexpat runs exist.
- These runs predate held-out verification, so there is no held-out rate yet.

## Files

- `<run>/summary.json`: rounds, seconds, token usage, final match rate
- `argon-1/full2000_verify.json`, `argon-2/final_verify.json`, `flash-2/full2000_verify.json`: verifier reports on 2,000 inputs
- `<run>/cjson.rs`: the Rust each model wrote
- `adversarial/`: `*-targeted.json` are hand-targeted edge inputs, `*-argon.json` are inputs Argon wrote to break the port
