# Pitch (3 min, no slides)

Mapped to the judging rubric: live demo 50%, problem and usefulness 25%, originality 15%, clarity 10%.
Lines marked **[verify]** must be checked right before we go on stage.

## Rules for every line

- Lead with the user, not the model.
- Say "independently checks agreement on N tested inputs" or "N tested behaviors preserved".
- Never say "proves it behaves the same", "equivalent", "100% compatible", "secure", or "CVE fixed". For the CVE input say "rejects this historical crashing input".
- Theseus is a migration tool with evidence. Never call it a dashboard (banned project type).
- Say out loud what we built today and what we used: cJSON and libexpat are third-party inputs, not our code.
- One memorable moment per beat, not five stats.
- libexpat claim, verbatim or very close: "In one reply, Argon translated libexpat's parser core, about 18,500 lines of C, into 6,554 lines of safe Rust. After two compile-only fixes (3 lines changed, no behavior fixes), it matched the C on every input we tried: the 747 inputs the repair loop used (W3C conformance suite plus edge cases; 285 well-formed, compared event by event) and 1,253 unseen mutated inputs (only 52 of them well-formed and compared event by event; the other 1,201 match on exact error code, line and column). Host and sandbox re-checks on two more mutation seeds agreed. Flash produced no file in our tested configuration." Sources: `backend/ai/results/README.md`, `results/libexpat-host-reverify/README.md`.
- libexpat scope: the 2.9.0 parser core under one configuration (`XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call, events before an error not compared). Not the full public API, not incremental parsing, not a drop-in replacement. Never say "we rewrote/replaced the XML parser inside Python and Firefox"; say libexpat is the parser Python and Firefox use.
- Flash on libexpat: at thinking high it hit its 65,536 thinking + output cap after 2,620 output tokens. The medium-thinking retry ended `DEADLINE_EXCEEDED`, which may be infrastructure: never present it as a capability failure. n = 1 per model. Never say "Flash can't".
- Never quote the input-token figure for Argon's libexpat reply (unexplained). Never say libexpat ran in a sandbox (it ran locally). Say "one reply plus two compile-only fixes", never "one shot".
- The banned-API guard is a text scan, not AST analysis and not a sandbox.
- Never say "your CI would merge it" or "any reviewer would approve it": no saved evidence for either.
- The cJSON `\uZZZZ` behavior is already publicly documented (independent cJSON analysis, RFC 8259 requires four hex digits). Never claim we discovered it. Say "differential testing surfaced it".
- Never say "proves", "mathematically", "cryptographic", "audited" or "equivalence", even if a reviewer suggests it.

## Core line

"Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them."

## Script

**Timing.** The live cJSON run takes ~146-166 s (round 1 lands ~0:25, Argon repair ~90 s, round 2 + held-out ~2:20-2:45). Never open a new PR on stage. Recorded things are always called recorded.

**Before 0:00.** Home page open, "Start live run" (from Flash's port) ready, Try panel on XML with the rich preset loaded, PR tab open, a finished recorded run loaded as fallback.

**0:00 Headline (click Start while speaking).**
"Today Argon read libexpat's parser core, the XML parser Python and Firefox use, about 18,500 lines of C, and wrote 6,500 lines of memory-safe Rust in one 18-minute reply. In our run, Flash ran out of room. The real question is how you'd ever trust that rewrite. That's what we built."

**0:20 Try XML.**
"Give me some XML." Run it (or the rich preset): real libexpat on the left, Argon's Rust on the right, event by event, identical. "We checked it on 2,000 inputs, including the W3C conformance suite. Every one matched." Then billion laughs, 5 s: "Same error, same line, same column on billion laughs."

**0:45 The villain, live.**
Switch to the run, round 1 is showing. "This is Flash's port of a smaller library, cJSON. It compiles, it's memory-safe, it passes 2,000 tests. It's wrong." Click the 9-byte divergence. "That went to Argon, which is fixing it right now. Argon writes the code; it never grades its own homework."

**1:15 CVE, 15 s.**
`{"1":1,`. "A published CVE in old cJSON, fixed upstream since. The C reads past its buffer. The Rust refuses. Safe Rust bounds-checks reads, which is the class of bug behind this one."

**1:30 Managed Agents, 25 s, recorded.**
"Repairs run in Google-hosted sandboxes. The sandbox says 1.0; our host re-ran it: 1.0, check. A human approves, the sandbox opens the pull request, and the GitHub token never enters the sandbox." Show the PR: "From an earlier run: code, evidence, a command to reproduce, and its limits."

**1:55 What we didn't check, 15 s.**
"We're explicit about limits: these are tested behaviors, not a proof, and the certificate lists what's not covered."

**2:10 Back to the live run.**
When round 2 and held-out land: "There's Argon's repair, re-checked on inputs the loop never saw." If still running: "Still running; here's the same flow from our recorded run," and say it's recorded.

**2:35 Close.**
"Every difference we tested is made visible, and a person approves before anything ships. AI can already rewrite your C. Theseus is how you trust it. Built today."

**Q&A only (not on stage):** the Flash-was-more-spec-correct tradeoff, the JSON Try panel.

## Q&A answers to have ready

- **"How many unseen, well-formed documents were compared event by event?"** 52 in the held-out set (56 more in the seed-4242 re-check). The other 1,201 unseen inputs end in an error and match on exact error code, line and column. The 747 loop inputs, including 285 well-formed W3C and edge documents, were also matched, but the loop saw them.
- **"Show me the planner."** There isn't one. We translate one library in one call; the multi-module planner from our plan isn't built. That's the next step.
- **"Why does fixing one line cost so many tokens?"** Each compile fix regenerated the whole 6,500-line file in a fresh conversation, because a follow-up turn on the original conversation failed near its output limit. An edit-based repair (send a diff, not the file) is the obvious fix; we didn't build it today.
- **"Is unwrap_or(0) safer than a panic?"** No. The polish pass replaced panic-prone indexing with bounds-checked reads, and 18 of those fall back to a default where C would read out of bounds. A loud error is better; returning an error instead of 0 is a fix we haven't made. We don't claim the lint count as a safety win.
- **"Run mutation testing on the libexpat inputs."** We haven't; mutation testing covers cJSON only. Coverage we did measure: our inputs run 52% of the parser's lines overall and about 75% of the lines reachable in the tested configuration.
- **"Did Argon copy the published Rust expat crate?"** Argon's port shares 7% of its 8-token sequences and 3.6% of its lines with that crate, nearly all traceable to the C both translate, against 99.8% for an actual copy. That rules out copying, not training exposure.
- **"Does real Python run on it?"** Partly. CPython's own pyexpat and ElementTree, built unmodified, pass 646 of 780 of Python's XML tests on Argon's libexpat (768 on stock expat; a stub that parses nothing passes 392). Failures are namespaces (by design, we return a clean error), streaming, some DTD features and non-UTF-8 encodings. The C bridge has a known aliasing issue in its unsafe layer, so we don't demo it live.

- **"Why not just Flash with retries?"** On cJSON Flash got there with a retry at lower thinking, so we don't claim it can't. It took about 2.5x longer with more tokens. On libexpat, at thinking high Flash hit its 65,536 thinking + output cap after 2,620 output tokens and produced no file; Argon finished the same input in one reply. Its retry at medium ended `DEADLINE_EXCEEDED`, which may be infrastructure, so we don't count it. n = 1 per model.
- **"Isn't Flash's failure a config problem? Couldn't Flash split it into pieces?"** Possibly. We ran one configuration, same prompts and settings for both, n = 1. The claim is only that in that configuration Flash hit its output cap and Argon produced a whole file. A chunked Flash pipeline might work; we didn't build or test one. Theseus would verify its output the same way.
- **"Would you merge this into my repo? What does the certificate certify?"** It certifies one thing: on the listed inputs, the C and the Rust produced the same output bytes and exit status through our driver, checked by our verifier on our host, including held-out inputs the model never saw. It does not certify every input, every API function, or performance. You merge it after your own review, with the reproduce command in hand.
- **"Why did the sandbox matter? It's three gh commands."** The PR step is the small part. The sandbox ran the real translate, build, verify and repair loop: it installed clang with ASan and Rust, built the C, and ran our verifier itself (517 s for Argon, 1,827 s for Flash; evidence in `results/cjson-sandbox-run/`). The host then rebuilt and re-verified the result. For the PR, the scoped token is injected at the egress proxy, so it never enters the sandbox.
- **"20,000 inputs is not better than 200 good ones."** Agreed, which is why count is not our argument. We have 980 targeted edge inputs (number boundaries, every escape, invalid UTF-8, nesting at the limit), 225 inputs Argon wrote specifically to break its own port, and a mutation check that plants single-site bugs in the C to confirm our inputs catch them. The targeted set is what caught Flash's divergence.
- **"Rust ports of cJSON already exist."** Yes, that's why cJSON is our floor, not our claim. The product is the repeatable migration workflow and the independent evidence. Rust ports of libexpat exist too (an experimental `expat` crate); we don't claim a first port.
- **"Did Argon just memorize an existing Rust port?"** We can't rule out training exposure: an experimental Rust `expat` crate exists, and we list that as a limitation. What we can say: the port had to implement our own driver interface, one function we defined today, and match libexpat's exact error codes, lines and columns on 1,253 held-out mutated inputs. A copied crate would not fit that interface as is.
- **"Most of your libexpat inputs are error cases."** True: 1,659 of 2,000 end in a parse error and are compared only on error code, line and column; events before the error are not compared. The other 341 match event by event, including 215 valid W3C conformance documents. We list this in Known limitations.
- **"Did you really port libexpat, or build something that passes your tests?"** We ported the parser core under one configuration: `XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call. Not the full public API, not incremental parsing, not a drop-in replacement. Inside that scope, inputs came from a seed the loop never saw (4242), and no behavior was repaired: the only changes after the first reply were 3 lines of compile fixes.
- **"Did it keep hash-flooding protection?"** Not verified. The hash salt is fixed for determinism, so the attack class behind CVE-2012-0876 is not exercised. It's a listed limitation.

- **"How do you know your verifier is right?"** Three controls: C against C must match on every input, a canary input with one known output must pass first, and a do-nothing stub scores near zero. Our first driver passed C-vs-C while broken. That's why the canary exists. Weak spots we admit: embedded NUL bytes in strings are compared only up to the NUL, and the canary set is small.
- **"Everything matched on the first try? Your tests are easy."** Argon's port matched 980/980 targeted inputs, 225/225 inputs Argon wrote to break it, and 20,000 extra fuzz inputs. Flash's port matched 961/980. That is agreement on those inputs, nothing more. Be precise: across all saved ports, Argon hit 980/980 in 1 of 4 and 961/980 in 3 of 4, Flash 961/980 in 2 of 2, always the same 19 inputs. That quirk is hard for both; never present it as Argon beating Flash.
- **"What does the match rate exclude?"** Inputs where C crashes are listed separately as safety improvements, never counted as preserved behaviors. Infra failures never count as matches. Oversized inputs are skipped and counted. It covers parse-and-print output and exit status through our driver, not every cJSON function or performance.
- **"Is this replaying a success story?"** No: the Try panel runs a judge's own input through the real C and the real Rust live, and the repair run is live. Name any replayed part out loud.
- **"Could the Rust cheat by calling the C binary?"** The build gate's text scan rejects process, filesystem, network and environment APIs in the generated Rust, and the crate has `#![forbid(unsafe_code)]` with no dependencies. It is a text scan, not AST analysis and not a sandbox.
- **"Does the CVE demo show Rust fixed it?"** No. It shows the vulnerable release crashing and the Rust rejecting the same bytes cleanly. cJSON fixed it in 1.7.18. Safe Rust removes the class of bug (out-of-bounds reads), which is the point for the next one.
- **"Why do you need 1M context?"** cJSON alone doesn't show it: Flash also reached 2,000/2,000. libexpat does: Argon read the whole parser core, about 18,500 lines of C, and wrote 6,554 lines of Rust in one reply, 73,653 output tokens. In our run Flash stopped at its 65,536 thinking + output cap after 2,620 output tokens. n = 1 per model, one configuration.
- **One divergence story, in depth:** Flash on `"\uZ\t\\"`. cJSON's two-pass string parser turns bad `\u` hex into U+0000 and then accepts a lone backslash before the closing quote. Flash's port checks for two remaining bytes instead of one, so it rejects; Argon's matches C. In the sandbox run, Argon itself first rejected `"\uZZZZ"` (C parses it as `""`), scored 0.9933 in round 1, and repaired it in round 2. Do not say Argon reliably gets this right: it missed the same 19 inputs in 3 of 4 ports. The point of the story is that Theseus catches it, whichever model wrote the port.
