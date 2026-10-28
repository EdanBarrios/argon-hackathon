# Theseus

**Theseus migrates C parsers to memory-safe Rust and checks the result with a verifier the model cannot influence.**

Asking an LLM whether its rewrite is correct is not evidence. Theseus runs the original C and the Rust replacement on the same inputs, compares raw output bytes and exit status, hands every difference back to the model to repair, and then re-checks on inputs the model never saw. Gemini 4 Argon writes the Rust. A deterministic differential verifier decides what counts as a match.

Built at the Gemini 4 Argon SF Hackathon (Oct 25, 2026) by a team of three, then extended afterwards. Current status of each target is below, including what is **not** verified.

## Results

| Target | What happened | Status |
|---|---|---|
| **libexpat 2.9.0** (parser core) | Argon translated about 18,500 lines of C into 6,554 lines of safe Rust in one reply. After 3 lines of compile fixes, no behavior repairs, it matched C on 2,000/2,000 inputs from a seed the repair loop never saw. | Verified on tested inputs, n = 1 |
| libexpat, second independent run | A second Argon translation (6,518 lines, 1-line compile fix) matched 747/747 loop inputs, 1,253/1,253 held-out inputs and 2,000/2,000 on seed 4242. | Verified on tested inputs |
| **cJSON 1.7.19** | Argon and Gemini 3.8 Flash both reached 2,000/2,000 generated inputs. Details below. | Verified on tested inputs, n = 2 per model |
| CVE replay | CVE-2024-8176 crashes libexpat 2.6.4 under ASan with a stack overflow. Argon's Rust output is byte-identical to fixed 2.9.0 C. CVE-2022-25313 was not reproduced with our input. | One reproduced, one stated as not reproduced |
| **Lua 5.4.7** | Argon wrote 17,430 lines of Rust in one reply (191K output tokens). Two borrow-check errors were fixed by hand (9 lines). | **Translated, not verified.** No Lua harness exists yet. An informal smoke test passed 22/22. |

"N tested behaviors preserved" means C and Rust produced identical output bytes and exit status on N inputs, through our driver. It is agreement on tested inputs. It is not a proof, not a claim about every input or every API function, and not a drop-in replacement.

Sources: `results/`, `backend/ai/results/`, `docs/FRICTION-LOG.md`.

### Reproduce the libexpat result without an API key

```
scripts/reproduce.sh
```

Needs `clang`, `cargo`, `git`, `curl`, `python3` and network access. It fetches libexpat R_2_9_0 and the W3C XML conformance suite at pinned versions, builds the C library with AddressSanitizer and UBSan, builds the committed Argon port (checked against a recorded sha256), and runs both on 2,000 inputs. Expected last line: `match_rate 1.0  behaviors_tested 2000`. Only macOS x86_64 has been tested.

### libexpat details

| | Gemini 4 Argon | Gemini 3.8 Flash |
|---|---|---|
| Translate at thinking high | Completed in one reply: 1,089 s, 6,554 lines, one compile error | Stopped `incomplete`: hit its 65,536 thinking + output cap after 2,620 output tokens |
| After translation | Two fresh repair calls, each fixing one compile error; 3 lines changed, no behavior repaired | Retry at thinking medium ended `DEADLINE_EXCEEDED`, which may be infrastructure, so it is not counted as a capability result |
| Loop inputs (seed 1) | 747/747: 599 W3C conformance documents + 148 edge cases | no file |
| Held-out (seed 1001, loop inputs removed) | 1,253/1,253 | no file |
| Host re-verify, seed 4242 | 2,000/2,000: 0 divergences, 0 crashes, 0 timeouts, 0 infra failures | no file |

n = 1 per model, one configuration. Scope: parser core with `XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call on the whole input, default billion-laughs limits, fixed hash salt. On a parse error only the error code, line and column are compared. Of the 2,000 seed-4242 inputs, 341 are well-formed and compared event by event; 1,659 end in an error. See `results/libexpat-host-reverify/README.md`.

### cJSON: Argon vs Flash (same prompts and verifier, n = 2 runs per model)

| | Gemini 4 Argon | Gemini 3.8 Flash |
|---|---|---|
| First attempt at thinking high | 2 of 2 finished, every input matched in round 1 | 0 of 2 finished: ran out of its 65,536-token cap mid-file |
| Generated inputs | 2,000/2,000 | 2,000/2,000 (after an automatic retry at thinking medium) |
| Time to a working port | 333 s, 355 s | 838 s (failed attempt + retry) |
| Total tokens | 66K, 71K | 189K |
| Targeted edge inputs | 980/980 in this port; across all 4 saved Argon ports, 1 at 980/980 and 3 at 961/980 | 961/980 in 2 of 2 ports |
| Inputs Argon wrote to break its own port | 225/225 | 225/225 |

The targeted split is one sample, not a model difference: Argon missed the same 19 inputs in 3 of its 4 ports. All 19 come from one cJSON quirk (`"\uZ\t\\"`: C prints `OK\t""`, Flash's port prints `PARSE_ERROR at 7`). This behavior is already publicly documented; differential testing surfaced it for us. Details: `results/cjson-flash-vs-argon/README.md`.

Live repair runs, starting from Flash's first port with the targeted string inputs added: round 1 matched 559 of 578 inputs with 19 divergences, all in string parsing. Argon repaired them in one call (86 s in the saved run `job_be9796`), and round 2 matched 578/578, then 64/64 on held-out inputs. Six saved runs show the same 19 divergences and the same fix; two earlier runs used 514 inputs (96.3% in round 1, 0 divergences after repair). Stage time ranged from 145 s to 412 s, the slowest under heavy machine load. See `results/cjson-live-repair/`.

cJSON v1.7.17 has CVE-2023-53154: the 7 bytes `{"1":1,` cause a heap buffer over-read under ASan when parsed as an exact-length buffer. The Rust port rejects that input with `PARSE_ERROR at 7`. cJSON fixed the bug in v1.7.18; this shows the port handles the input cleanly, not that the Rust supplied the fix.

## How it works

1. **Translate.** Argon reads the C module and writes one safe-Rust file against a fixed driver interface.
2. **Build gate.** `scripts/build_rust.sh` compiles it in a crate with `#![forbid(unsafe_code)]` and no dependencies. A text scan of the source (not AST analysis, not a sandbox) rejects process, filesystem, network and environment APIs, so the Rust cannot shell out to the C.
3. **Verify.** `verify.sh` feeds identical bytes to the C driver (ASan + UBSan) and the Rust driver and compares raw stdout and exit status. Every input lands in exactly one class: match, divergence, Rust crash, timeout, infra failure, or safety improvement (C crashed, Rust exited cleanly). Timeouts and infra failures stay in the denominator, so a broken run can never read as 100%.
4. **Repair.** Divergences go back to Argon with the exact input and both outputs. Repeat until everything matches or the round budget runs out.
5. **Held-out check.** The final build is verified again on inputs the repair loop never saw.
6. **Approve and release.** A person approves, then a managed agent opens a pull request with the crate and an `EVIDENCE.md` built from verifier output. [Example](https://github.com/EdanBarrios/theseus/pull/3).

**Is the verifier itself trustworthy?** Before any model call, preflight requires C against C to match on every input, a canary input with a known output to pass, and a reject-everything stub to score near zero. Nondeterminism is removed at the source (libexpat's hash salt is fixed to zero). `mutants.py` plants single-site bugs in cJSON to confirm the inputs catch them. Our first driver passed C-vs-C while broken, which is why the canary exists (`docs/FRICTION-LOG.md`).

**Where test inputs come from.** libexpat uses 747 fixed inputs (W3C conformance suite plus edge cases) and seeded mutations. Held-out seeds are never seen by the loop, with overlap removed. cJSON uses generated JSON plus targeted edge inputs. The C is always the answer key; no expected outputs are written by hand.

## Managed agents

Argon can run the whole loop inside an isolated Google-hosted sandbox: it installs clang and Rust, builds the C, writes the Rust, runs our `verify.sh`, reads the divergences and repairs. **The agent's report is a claim, not evidence.** The host pulls the sandbox's own reports, rebuilds the returned code locally, and reruns `verify.sh` on the same inputs and on held-out inputs the agent never saw. In our first run the sandbox and host agreed for both models.

| cJSON, one sandbox run per model | Argon | Flash |
|---|---|---|
| Sandbox wall time | 517 s | 1,827 s |
| Input tokens | 350K | 4.0M |
| Rounds claimed | 2 (0.9933, then 1.0) | 1 (the transcript shows 49 code-execution calls of edit and rerun) |
| Host rebuild, same 300 inputs / 64 held-out | 1.0 / 1.0 | 1.0 / 1.0 |

n = 1 per model. Evidence: `results/cjson-sandbox-run/`.

Opening the PR is the only step with a credential, and it needs a person to approve. The GitHub token is registered with the Credentials API and injected by the sandbox egress proxy only on requests to github.com and api.github.com. We tested this with a fake token: GitHub received it through the proxy (`401 Bad credentials`), while the token never appeared in the sandbox environment. Code: `backend/harness/remote/`. What broke along the way is in `docs/FRICTION-LOG.md`.

## Limitations

- **Scope.** libexpat covers the 2.9.0 parser core under one configuration, not the full public API and not incremental parsing. cJSON covers parse-and-print through our driver, not every function or performance.
- **Mostly error inputs.** 1,659 of the 2,000 libexpat seed-4242 inputs end in a parse error and are compared only on error code, line and column.
- **Hash flooding is untested.** The hash salt is fixed for determinism, so the attack class behind CVE-2012-0876 is not exercised.
- **Training exposure.** An experimental Rust `expat` crate exists. Our only mitigation is that the port had to match our own driver interface and exact error positions. A similarity check rules out copying, not exposure.
- **`unwrap_or(0)` fallbacks.** 18 bounds-checked reads in the libexpat port fall back to a default where C would read out of bounds. A loud error would be better; we have not made that change.
- **pyexpat bridge.** CPython's own pyexpat built on Argon's libexpat passes 646 of 780 XML tests (stock expat: 768). The C bridge has a known aliasing issue in its unsafe layer.
- **Text-scan guard.** The banned-API check is not AST analysis and not a sandbox.
- **Embedded NUL bytes** in cJSON strings are compared only up to the NUL.
- **Small samples.** n = 1 or 2 per model per target. Nothing here is a general statement about either model.
- **Platforms.** Only macOS x86_64 with Apple clang has been tested.
- **Edit-based repair is implemented but not measured.** `python -m ai.loop --patch-repair` asks the model for SEARCH/REPLACE blocks that the host applies (all or nothing, falling back to a full-file repair if a block does not match exactly once). It has unit tests but no live run, so there is no token or latency number for it yet. The default repair still regenerates the whole file.
- **Not done yet:** Lua verification, coverage-guided differential fuzzing, swapping a single Rust module into the C build.

## Run the app

```
cd backend && uv sync && THESEUS_MODE=mock uv run uvicorn api.main:app --port 8000
cd frontend && npm install && npm run dev   # http://localhost:3000
scripts/selftest.sh                          # verifier self-test (C vs C, canary)
```

Mock mode needs no API key and replays recorded runs. Real mode needs a Gemini API key in `.env` (see `.env.example`). Backend tests: `cd backend && uv run pytest -q` (about 3 minutes; the billion-laughs test can time out under heavy load).

## Repository layout

| Path | What |
|---|---|
| `backend/harness/` | Differential verifier (`verify.sh`, `verify.py`), C drivers, input generators, mutation check, libexpat harness |
| `backend/ai/` | Argon translate/repair loop, held-out check, saved results |
| `backend/api/` | FastAPI jobs API and release step |
| `frontend/` | Next.js run view and Try panel |
| `scripts/` | Build gate, self-tests, `reproduce.sh` |
| `results/` | Committed evidence and write-ups |
| `docs/` | Plan, decisions, friction log, notes |

## Credits and license

Theseus is MIT licensed (`LICENSE`). Systems (verifier, pipeline, API): Edan Barrios. AI (translation and repair): Brendan. Frontend: Keon.

cJSON (MIT), libexpat (MIT), Lua (MIT) and the W3C XML conformance suite are third-party inputs, fetched at build time and never committed. See `THIRD_PARTY.md`. The Rust in `backend/ai/results/` was written by Gemini models through this pipeline.
