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

## Core line

"Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them."

## Script

**Before 0:00.** Run view open, Flash's first-attempt start selected. Try panel one click away. PR #3 open in a background tab.

**0:00 Start the run (live demo).**
Click Start on the live repair run from Flash's first attempt. Talk over it.
"If you ship a C parser inside your product, one bad pointer in that parser is your vulnerability. Theseus migrates it to memory-safe Rust and gives you evidence your auditors can rerun. Theseus doesn't ask the model whether the port works. It runs the original and the replacement on the same inputs, exposes the differences, and makes the model repair them. That run just started. Let's use it while it works."

**0:20 Try panel: a judge's JSON (live demo).**
Open the Try panel and ask a judge for any JSON, or type one they call out.
"Same bytes into the original C and into the Rust. We compare raw output, byte for byte. Not a replay: your input, right now."
Moment: the two outputs side by side, identical.

**0:45 CVE preset (problem).**
Click the CVE preset `{"1":1,`.
"Seven bytes. Old cJSON, 1.7.17, reads past the end of its buffer on these, and AddressSanitizer catches it: CVE-2023-53154. The Rust port rejects this historical crashing input with a clean parse error. cJSON fixed this upstream in 1.7.18; the point is that safe Rust cannot express that read at all."
Moment: the red ASan report next to `PARSE_ERROR at 7`.

**1:05 Back to the run: repairing Flash (live demo).**
Switch to the run. By now it should show the divergences and Argon's repair round (a rehearsal took 166 s end to end).
"We started from Gemini 3.8 Flash's port. Our verifier found inputs where it disagrees with C. Click one."
Click the `"\uZ\t\\"` divergence.
"C accepts this odd string and prints an empty string. Flash's port rejects it. Cleaner, maybe, but not what C does, and behavior-preserving means preserving the weird parts too. That input and both outputs went to Argon. Argon repaired it. Now it matches every input, including a held-out set it never saw."
Moment: the divergence turning green.

**1:40 Why Argon (originality).**
"On cJSON, Argon wrote a working port on its first try, two runs out of two, about two and a half times faster than Flash with fewer tokens. Flash hit its 64K output cap mid-file and only finished by thinking less, and its port still diverged on 19 of our targeted edge inputs."
"The real target is libexpat, the XML parser: 18,494 lines, 204,000 tokens. Flash caps thinking plus output at 65,536 tokens. We estimate a full port needs over 100,000 output tokens. That's the structural reason it needs Argon. The libexpat runs are pending, so that is a prediction, not a result."

**2:05 Managed Agents (managed agents prize).**
Show a sandbox run card: the agent's claimed match rate next to the host check tick.
"Bigger runs happen in managed-agent sandboxes. The agent installs the compilers, translates, builds, runs our verifier and repairs, all inside the sandbox. Its report is a claim, so we rebuild its Rust and rerun the verifier ourselves, including inputs it never saw. Here they agree."
"Opening a PR needs a GitHub token. We tested with a fake token: GitHub received it through the sandbox's proxy, and the token never appeared inside the sandbox."

**2:30 Approve and the migration certificate.**
Click Approve. Switch to PR #3 (or the PR the click opens).
"Nothing ships without a person. The pull request is a migration certificate: the code, the evidence report from the verifier, the command to reproduce it, and what it does not cover." **[verify]** that the PR's EVIDENCE.md has a limitations section; if not, say "and the limits, which are in our README".

**2:45 Close.**
"Built today: the verifier, the repair pipeline, the sandbox runs, the UI. cJSON and libexpat are other people's code. Theseus doesn't ask the model whether the port works. It checks, and it shows you the receipts."

## Q&A answers to have ready

- **"Why not just Flash with retries?"** On cJSON Flash did get there with a retry at lower thinking, so we don't claim it can't. But it took about 2.5x longer and more tokens, and its finished port still had a real divergence on 19 targeted inputs. Retries don't move the output cap: Flash stops at 65,536 tokens of thinking plus output, and we estimate libexpat needs 100K+ output tokens. That libexpat comparison is pending.
- **"Would you merge this into my repo? What does the certificate certify?"** It certifies one thing: on the listed inputs, the C and the Rust produced the same output bytes and exit status through our driver, checked by our verifier on our host, including held-out inputs the model never saw. It does not certify every input, every API function, or performance. You merge it after your own review, with the reproduce command in hand.
- **"Why did the sandbox matter? It's three gh commands."** The PR step is the small part. The sandbox ran the real translate, build, verify and repair loop: it installed clang with ASan and Rust, built the C, and ran our verifier itself (517 s for Argon, 1,827 s for Flash in parallel). The host then rebuilt and re-verified the result. For the PR, the scoped token is injected at the egress proxy, so it never enters the sandbox.
- **"20,000 inputs is not better than 200 good ones."** Agreed, which is why count is not our argument. We have 980 targeted edge inputs (number boundaries, every escape, invalid UTF-8, nesting at the limit), 225 inputs Argon wrote specifically to break its own port, and a mutation check that plants single-site bugs in the C to confirm our inputs catch them. The targeted set is what caught Flash's divergence.
- **"Rust ports of cJSON already exist."** Yes, that's why cJSON is our floor, not our claim. The product is the repeatable migration workflow and the independent evidence. Rust ports of libexpat exist too (the experimental `expat` crate); we don't claim a first port.
- **"How do you know your verifier is right?"** Three controls: C against C must match on every input, a canary input with one known output must pass first, and a do-nothing stub scores near zero. Our first driver passed C-vs-C while broken. That's why the canary exists. Weak spots we admit: embedded NUL bytes in strings are compared only up to the NUL, and the canary set is small.
- **"Everything matched on the first try? Your tests are easy."** Argon's port matched 980/980 targeted inputs, 225/225 inputs Argon wrote to break it, and 20,000 extra fuzz inputs. Flash's port matched 961/980. That is agreement on those inputs, nothing more.
- **"What does the match rate exclude?"** Inputs where C crashes are listed separately as safety improvements, never counted as preserved behaviors. Infra failures never count as matches. Oversized inputs are skipped and counted. It covers parse-and-print output and exit status through our driver, not every cJSON function or performance.
- **"Is this replaying a success story?"** No: the Try panel runs a judge's own input through the real C and the real Rust live, and the repair run is live. Name any replayed part out loud.
- **"Could the Rust cheat by calling the C binary?"** The build gate rejects process, filesystem, network and environment APIs in the generated Rust, and the crate has `#![forbid(unsafe_code)]` with no dependencies.
- **"Does the CVE demo show Rust fixed it?"** No. It shows the vulnerable release crashing and the Rust rejecting the same bytes cleanly. cJSON fixed it in 1.7.18. Safe Rust removes the class of bug (out-of-bounds reads), which is the point for the next one.
- **"Why do you need 1M context?"** cJSON alone doesn't show it: Flash also reached 2,000/2,000. The case rests on libexpat (204K tokens of input, an estimated 100K+ tokens of output against Flash's 65,536 cap), and those runs are pending.
- **One divergence story, in depth:** Flash on `"\uZ\t\\"`. cJSON's two-pass string parser turns bad `\u` hex into U+0000 and then accepts a lone backslash before the closing quote. Flash's port checks for two remaining bytes instead of one, so it rejects; Argon's matches C. In the sandbox run, Argon itself first rejected `"\uZZZZ"` (C parses it as `""`), scored 0.9933 in round 1, and repaired it in round 2.
