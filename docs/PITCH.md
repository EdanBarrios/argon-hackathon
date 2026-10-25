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
- libexpat model results are pending. Never fill them in by estimate.
- Never say "any reviewer would approve it"; say "your CI would merge it" (provable: it passes all the tests).
- The cJSON `\uZZZZ` behavior is already publicly documented (independent cJSON analysis, RFC 8259 requires four hex digits). Never claim we discovered it. Say "differential testing surfaced it".
- Never say "proves", "mathematically", "audited" or "equivalence", even if a reviewer suggests it.

## Core line

"Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them."

## Script

Timing is built around measured numbers: the live run takes ~146 s (round 1 verify lands at ~25 s, Argon's repair ~90 s, round 2 plus held-out by ~2:25). A PR takes ~125 s, so we never open one on stage; we show the already-opened PR and say it came from an earlier run.

**Before 0:00.** Run view open with "Start live run" (from Flash's port) ready. Try panel one click away. The PR tab open in the background. A finished recorded run loaded as backup.

**0:00 The villain (click Start as you speak).**
"This Rust port of cJSON compiles. It's memory-safe. It passes all 2,000 of our standard tests. Your CI would merge it. Watch."
Round 1 lands at ~0:25: 96%, 19 divergences. Click the `"\uZ\t\\"` divergence.
"It's wrong. Nine bytes: the original C says one thing, this rewrite says another. That's the bug that breaks someone downstream six months from now."

**0:35 What Theseus is.**
"AI-written code is cheap now. Trusting it is the bottleneck. Theseus is the trust layer: it runs the old code and the new code on the same inputs and makes every behavior change visible. This divergence just went to Argon, with the exact input and both outputs, and Argon is fixing it right now. Argon never grades its own homework."

**0:55 Try panel: a judge's input (keep it to ~10 s).**
"Give me a short JSON." Run it. "Your input, the real C and the real Rust, same bytes out."

**1:05 CVE preset.**
Click `{"1":1,`. "A published CVE in cJSON 1.7.17, fixed upstream since. The old C reads past its buffer, there's AddressSanitizer catching it. The Rust rejects it. That whole class of bug can't be written in safe Rust."

**1:25 Why Argon (15 s, measured facts only).**
If libexpat cleared the bar (compiles, no unsafe, stated agreement on stated inputs): say that one result and where Flash's single pass stopped.
Otherwise: "Argon wrote this library's port right the first time in both runs, about 2.5x faster than Flash. Flash's single pass ran out of output room. The next target is libexpat, the XML parser inside Python, 18,000 lines."

**1:40 Managed Agents (recorded run, say so).**
Show the sandbox run's claim-vs-check: "Bigger runs happen in Google's managed sandboxes. The agent compiles, tests and repairs on its own, but its report is only a claim, so our host rebuilds and re-runs it: the sandbox said 1.0, we got 1.0, including inputs it never saw."
Switch to the PR tab. "After a human approves, the sandbox opens the pull request: the code, the evidence, a command to reproduce it, and what it doesn't cover. This PR came from an earlier run."

**2:10 Back to the live run.**
Repair lands green, round 2 at 100%, held-out check passing. "That's the live run: Argon's repair, re-checked on inputs the loop never saw."
If it isn't finished: "Still running; here's the same flow from the run we recorded earlier," and show the recorded run, labeled as recorded.

**2:35 The tradeoff, then close.**
"One more thing. Flash's version was arguably more correct: the JSON spec says that input is invalid. But a drop-in replacement that silently changes behavior breaks people. Theseus doesn't decide that silently. It makes it a human's decision."
"AI can already rewrite your C. Theseus is how you trust it. Built today: the verifier, the repair loop, the sandbox runs, the UI."

## Q&A answers to have ready

- **"Why not just Flash with retries?"** On cJSON Flash did get there with a retry at lower thinking, so we don't claim it can't. But it took about 2.5x longer and more tokens, and its finished port still had a real divergence on 19 targeted inputs. Retries don't move the output cap: Flash stops at 65,536 tokens of thinking plus output, and we estimate libexpat needs 100K+ output tokens. That libexpat comparison is pending.
- **"Would you merge this into my repo? What does the certificate certify?"** It certifies one thing: on the listed inputs, the C and the Rust produced the same output bytes and exit status through our driver, checked by our verifier on our host, including held-out inputs the model never saw. It does not certify every input, every API function, or performance. You merge it after your own review, with the reproduce command in hand.
- **"Why did the sandbox matter? It's three gh commands."** The PR step is the small part. The sandbox ran the real translate, build, verify and repair loop: it installed clang with ASan and Rust, built the C, and ran our verifier itself (517 s for Argon, 1,827 s for Flash in parallel). The host then rebuilt and re-verified the result. For the PR, the scoped token is injected at the egress proxy, so it never enters the sandbox.
- **"20,000 inputs is not better than 200 good ones."** Agreed, which is why count is not our argument. We have 980 targeted edge inputs (number boundaries, every escape, invalid UTF-8, nesting at the limit), 225 inputs Argon wrote specifically to break its own port, and a mutation check that plants single-site bugs in the C to confirm our inputs catch them. The targeted set is what caught Flash's divergence.
- **"Rust ports of cJSON already exist."** Yes, that's why cJSON is our floor, not our claim. The product is the repeatable migration workflow and the independent evidence. Rust ports of libexpat exist too (the experimental `expat` crate); we don't claim a first port.
- **"How do you know your verifier is right?"** Three controls: C against C must match on every input, a canary input with one known output must pass first, and a do-nothing stub scores near zero. Our first driver passed C-vs-C while broken. That's why the canary exists. Weak spots we admit: embedded NUL bytes in strings are compared only up to the NUL, and the canary set is small.
- **"Everything matched on the first try? Your tests are easy."** Argon's port matched 980/980 targeted inputs, 225/225 inputs Argon wrote to break it, and 20,000 extra fuzz inputs. Flash's port matched 961/980. That is agreement on those inputs, nothing more. Be precise: across all saved ports, Argon hit 980/980 in 1 of 4 and 961/980 in 3 of 4, Flash 961/980 in 2 of 2, always the same 19 inputs. That quirk is hard for both; never present it as Argon beating Flash.
- **"What does the match rate exclude?"** Inputs where C crashes are listed separately as safety improvements, never counted as preserved behaviors. Infra failures never count as matches. Oversized inputs are skipped and counted. It covers parse-and-print output and exit status through our driver, not every cJSON function or performance.
- **"Is this replaying a success story?"** No: the Try panel runs a judge's own input through the real C and the real Rust live, and the repair run is live. Name any replayed part out loud.
- **"Could the Rust cheat by calling the C binary?"** The build gate rejects process, filesystem, network and environment APIs in the generated Rust, and the crate has `#![forbid(unsafe_code)]` with no dependencies.
- **"Does the CVE demo show Rust fixed it?"** No. It shows the vulnerable release crashing and the Rust rejecting the same bytes cleanly. cJSON fixed it in 1.7.18. Safe Rust removes the class of bug (out-of-bounds reads), which is the point for the next one.
- **"Why do you need 1M context?"** cJSON alone doesn't show it: Flash also reached 2,000/2,000. The case rests on libexpat (204K tokens of input, an estimated 100K+ tokens of output against Flash's 65,536 cap), and those runs are pending.
- **One divergence story, in depth:** Flash on `"\uZ\t\\"`. cJSON's two-pass string parser turns bad `\u` hex into U+0000 and then accepts a lone backslash before the closing quote. Flash's port checks for two remaining bytes instead of one, so it rejects; Argon's matches C. In the sandbox run, Argon itself first rejected `"\uZZZZ"` (C parses it as `""`), scored 0.9933 in round 1, and repaired it in round 2. Do not say Argon reliably gets this right: it missed the same 19 inputs in 3 of 4 ports. The point of the story is that Theseus catches it, whichever model wrote the port.
