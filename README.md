# Theseus

Theseus migrates C parsers to memory-safe Rust and hands back evidence a reviewer can check. Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them. Gemini 4 Argon writes the Rust; a deterministic verifier, not the model, decides what counts as a match.

**Who it is for.** Teams that ship a C parser inside their own product (product security, embedded, appliances) and want a memory-safe drop-in, with evidence their auditors can rerun. Upstream maintainers of those parsers come second: the same evidence is what a maintainer would need before accepting a Rust rewrite.

**What a result means.** "N tested behaviors preserved" means the C and the Rust produced identical output bytes and exit status on N inputs, through our driver. It is agreement on the tested inputs, not a claim about every possible input or every API function.

## Team

- Edan (systems: verifier, pipeline, API)
- Brendan (AI: translation and repair with Gemini 4 Argon)
- Keon (frontend: run view and demo)

## Built during the event

Built entirely during the Gemini 4 Argon SF Hackathon, Oct 25 2026. Hacking started ~10:13 AM PT per organizer announcement and ends 5:00 PM PT. The only pre-written file is the plan in `docs/PLAN.md` (notes).

### Built today vs. used

| Built today by our team | Used as inputs (third-party, not our code) |
|---|---|
| Differential verifier (`backend/harness/verify.sh`, `verify.py`), C drivers, input generators, self-test with canary, mutation check (`mutants.py`) | [cJSON](https://github.com/DaveGamble/cJSON) v1.7.19 and v1.7.17 (MIT): the C library being migrated, fetched at build time, never committed |
| Adversarial corpora (`backend/harness/adversarial/`), including 225 inputs Argon wrote to break its own port | [libexpat](https://github.com/libexpat/libexpat) R_2_9_0 (MIT) and the W3C XML conformance suite: the moonshot target, fetched at build time |
| Rust crate template and build gate (`scripts/build_rust.sh`): `#![forbid(unsafe_code)]`, rejects process, filesystem, network and environment APIs | Gemini 4 Argon, Gemini 3.8 Flash, antigravity managed agents (Google APIs) |
| Argon translate/repair loop with held-out check (`backend/ai/`) | FastAPI, Next.js, google-genai (open-source frameworks) |
| Jobs API (`backend/api/`), managed-agent runner and PR release step (`backend/harness/remote/`) | |
| libexpat differential harness (`backend/harness/expat/`) | |
| Run view UI with the Try panel (`frontend/`) | |

The Rust code in any results was written by Gemini 4 Argon or Gemini 3.8 Flash during the event, through our pipeline.

## Architecture

1. **Translate.** Argon reads the C module and writes one safe-Rust file against a fixed driver interface.
2. **Build gate.** `scripts/build_rust.sh` compiles it in a crate with `#![forbid(unsafe_code)]` and no dependencies, and a text scan of the source (not AST analysis, not a sandbox) rejects any use of process, filesystem, network or environment APIs, so the Rust cannot simply shell out to the C.
3. **Verify.** `verify.sh` feeds the same bytes to the C driver (built with AddressSanitizer and UBSan) and the Rust driver and compares raw stdout and exit status. Every input lands in one class: match, divergence, Rust crash, timeout, infra failure, or safety improvement (C crashed, Rust exited cleanly). Infra failures never count as matches, and the report states its counts explicitly.
4. **Repair.** Divergences go back to Argon with the exact input and both outputs. Repeat until everything matches or the round budget runs out.
5. **Held-out check.** The final build is verified again on inputs the repair loop never saw.
6. **Approve and release.** A person approves, then a managed agent opens a pull request with the crate and an `EVIDENCE.md` built from verifier output.

The verifier is checked in both directions: C against C must match on every input after a canary passes, a do-nothing stub must score near zero, and `mutants.py` plants single-site bugs in the C to confirm the inputs catch them.

## Results

All numbers come from `results/cjson-flash-vs-argon/README.md`, `results/libexpat-host-reverify/README.md`, `backend/ai/results/README.md`, `docs/FRICTION-LOG.md`, `backend/harness/expat/README.md` and run artifacts under `runs/`. Targets: libexpat 2.9.0 parser core (one configuration, below) and cJSON v1.7.19 parse and print.

### libexpat: Argon's port, independently re-verified

In one reply, Argon translated libexpat's parser core, about 18,500 lines of C, into 6,554 lines of safe Rust. After two compile-only fixes (3 lines changed, no behavior fixes), independent host testing found agreement on all 2,000 fresh XML inputs: 341 compared event by event, including 215 valid W3C conformance documents, and 1,659 on exact error code, line and column. Flash produced no file in our tested configuration.

| | Gemini 4 Argon | Gemini 3.8 Flash |
|---|---|---|
| Translate at thinking high | Completed in one reply: 1,089 s, 6,554 lines of `expat.rs`, one compile error | Stopped `incomplete`: hit its 65,536 thinking + output cap after 2,620 output tokens (459 s) |
| After translation | Two fresh repair calls, each fixing one compile error (E0282 type annotation, E0505 borrow); 3 lines changed, no behavior repaired | Retry at thinking medium ended `DEADLINE_EXCEEDED`, which may be infrastructure, so we don't count it as a capability result |
| Repair loop inputs (seed 1) | 747/747: 599 W3C conformance documents + 148 edge cases (deep nesting, billion laughs, UTF-16, invalid UTF-8) | no file |
| Held-out inputs (seed 1001, minus every input the loop saw) | 1,253/1,253 | no file |
| Host re-verify on a fresh seed (4242) | 2,000/2,000: 0 divergences, 0 crashes, 0 timeouts, 0 infra failures | no file |

n = 1 per model. libexpat ran on a local runner, not in a managed sandbox. The C side is libexpat R_2_9_0 under ASan + UBSan. libexpat is the XML parser Python and Firefox use; our port covers its parser core under one configuration, not the full library. Details: `backend/ai/results/README.md` and `results/libexpat-host-reverify/README.md`.

Scope: libexpat 2.9.0 parser core with `XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call on the whole input, default billion-laughs limits, fixed hash salt. On a parse error only the error code, line and column are compared; events before the error are not. It is not the full public API, not incremental parsing, and not a drop-in replacement.

### cJSON: Argon vs Flash (same prompts and verifier, n = 2 runs per model)

| | Gemini 4 Argon | Gemini 3.8 Flash |
|---|---|---|
| First attempt at thinking high | 2 of 2 finished, every input matched in round 1, no repairs | 0 of 2 finished: Flash caps thinking + output at 65,536 tokens and ran out mid-file |
| Generated inputs | 2,000/2,000 | 2,000/2,000 (flash-2, after an automatic retry at thinking medium) |
| Time to a working port | 333 s, 355 s | 838 s (failed attempt + retry) |
| Total tokens | 66K, 71K | 189K |
| Targeted edge inputs | 980/980 in this port; across all 4 saved Argon ports, 1 at 980/980 and 3 at 961/980 | 961/980 in 2 of 2 ports: 19 divergences, one root cause |
| Inputs Argon wrote to break the port | 225/225 | 225/225 |

Argon was about 2.5x faster end to end with about 2.7x fewer tokens. Argon's port also agreed on 20,000 extra fuzz inputs. The targeted split is one sample, not a model difference: Argon missed the same 19 inputs in 3 of its 4 ports. Those 19 divergences all come from one cJSON quirk: on `"\uZ\t\\"` C prints `OK\t""` and Flash's port prints `PARSE_ERROR at 7`. Details in `results/cjson-flash-vs-argon/README.md`.

### Live repair of Flash's port

A live run starting from Flash's first attempt (514 inputs, including the targeted string inputs) began at 96.3% agreement with 4 divergent inputs, all in string parsing. Argon repaired them in one round (96 s) and the repaired build matched all 514 inputs. The whole run, including build, verify, repair, held-out check and CVE check, took 166 s.

### Historical crashing input

cJSON v1.7.17 has CVE-2023-53154: the 7 bytes `{"1":1,` cause a heap buffer over-read under AddressSanitizer when parsed as an exact-length buffer. The Rust port rejects this historical crashing input with `PARSE_ERROR at 7`. cJSON itself fixed the bug in v1.7.18; this shows the Rust port handles the input cleanly, not that the Rust supplied the fix.

### Managed-agent sandbox runs

| | Argon | Flash |
|---|---|---|
| Wall time in the sandbox | 517 s | 1,827 s |
| Input tokens | 350K | 4.0M |
| Rounds | Round 1: 0.9933 on 300 inputs (Argon rejected `"\uZZZZ"`, which C parses as `""`). Round 2: 1.0 | Reported 1 round (transcript shows 49 shell calls of edit and rerun) |
| Host rebuild and re-verify | 1.0 on the same 300 inputs, 1.0 on 64 held-out inputs, 1.0 on 2,000 | 1.0 on the same inputs, 1.0 held-out |

The held-out set was only 64 inputs because most of a 300-input set is cJSON's fixed test files, shared across seeds. The live pipeline now uses at least 300 generated inputs for the held-out check.

### Known limitations

- libexpat scope: the 2.9.0 parser core under one configuration (`XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call). Not the full public API, not incremental parsing, not a drop-in replacement.
- libexpat: 1,659 of the 2,000 host re-verify inputs end in a parse error and are compared only on error code, line and column; events before the error are not compared.
- libexpat hash-flooding protection is untested: the hash salt is fixed for determinism, so the class of attack behind CVE-2012-0876 is not exercised.
- Possible training exposure: an experimental Rust `expat` crate exists. Our only mitigation is that the port had to match our own driver interface and exact error positions.
- n = 1 per model on libexpat.
- The banned-API guard is a text scan of the generated source, not AST analysis and not a sandbox.
- cJSON agreement covers parse-and-print output and exit status through our driver, not every cJSON function or performance.
- Embedded NUL bytes in strings are compared only up to the NUL.
- n = 2 runs per model on cJSON. Across all saved ports (DECISIONS 12:27): Argon 1 of 4 ports at 980/980, 3 of 4 at 961/980; Flash 2 of 2 at 961/980; all 19 misses are the same `str_straddle_*` inputs. So this quirk is hard for both models, not a model difference. Argon's quality-gate run (backend/ai/results/cjson-quality/argon-full) reached 980/980 with 0 clippy and 0 panic lints.

## How Theseus uses Managed Agents

Every sandbox pre-run is a real migration, not a wrapper around three `gh` commands. Each module and model gets its own isolated antigravity sandbox, so an Argon run and a Flash baseline run side by side without sharing a filesystem. Inside the sandbox the agent installs clang with ASan and Rust, clones this repo, builds the original C, writes the Rust, runs our `verify.sh` itself, reads the divergences, repairs, and reruns. The prompt says `verify.sh` is the only judge and the agent must return its JSON verbatim.

**The agent's report is a claim, not evidence.** When a sandbox finishes, the host pulls the sandbox's own verify reports through the Environments files API, rebuilds the returned `cjson.rs` locally, and reruns `verify.sh` on the same inputs and on held-out inputs the agent never saw. Claim and host check are stored side by side. In our first run they agreed for both models (numbers in the sandbox table above).

**Opening the PR is the only step with a credential, and it needs a person.** After someone clicks Approve, a release agent clones the repo, commits `migrated/cjson/` (the crate plus an `EVIDENCE.md` built from verifier output), pushes a `theseus/<job>` branch and opens the PR. The GitHub token is registered with the Credentials API and injected by the sandbox's egress proxy only on requests to github.com and api.github.com, the only two hosts on that sandbox's allowlist. We tested this with a fake token: GitHub received it through the proxy (it answered `401 Bad credentials`, not `Requires authentication`), while the token never appeared in the sandbox environment. Credentials are deleted when the step ends. An example of the result is [PR #3](https://github.com/EdanBarrios/argon-hackathon/pull/3).

Code: `backend/harness/remote/` (built on `backend/ai/sandbox_run.py`). What broke along the way is in `docs/FRICTION-LOG.md`.

## Run it

```
cd backend && uv sync && THESEUS_MODE=mock uv run uvicorn api.main:app --port 8000
cd frontend && npm install && npm run dev   # http://localhost:3000
scripts/selftest.sh                          # verifier self-test (C vs C, canary)
```
