# Pitch (3 min, no slides)

Mapped to the judging rubric: live demo 50%, problem and usefulness 25%, originality 15%, clarity 10%.
Lines marked **[verify]** must be fact-checked before we say them on stage.

## Rules for every line

- Lead with the user, not the model.
- Say "N tested behaviors preserved". Never "100% equivalent".
- Theseus is a migration tool with proof. Never call it a dashboard (banned project type).
- Say out loud what we built today and what we used: cJSON and libexpat are third-party inputs, not our code.
- Only claim "smaller models can't do this" if our Flash baseline actually shows it.

## Script

**0:00 Who and why (problem, 25%).**
"This is for the people who maintain C libraries that everything depends on, like libexpat, the XML parser inside Python and Firefox, maintained by a small volunteer team. **[verify: team size]** C lets one bad pointer become a remote takeover. Rust makes that whole class of bug impossible. Maintainers would migrate if they could trust the rewrite. Theseus rewrites their C into safe Rust and proves the new code behaves the same."

**0:25 Live, real input (live demo, 50%).**
Open the Try panel. Ask a judge for any JSON, or type one.
"Same bytes into the original C and into Argon's Rust. Byte-for-byte comparison."
Then the CVE preset `{"1":1,`: "Seven bytes. An older cJSON release crashes under AddressSanitizer: CVE-2023-53154, a heap over-read. Argon's Rust returns a clean parse error on the same seven bytes."

**1:05 The verification loop (live demo, 50%).**
Start a live run from Flash's first attempt (or fresh at medium thinking). Lanes light up.
"Argon writes the code. It never grades its own homework. Our verifier runs both versions on thousands of inputs. Every divergence goes back to Argon with the exact input and both outputs. Argon repairs. Repeat until it matches."
Click a red divergence: input, C output, Rust output, the function.

**1:45 Why only Argon (originality, 15%).**
Switch to the pre-run.
"cJSON is 3,000 lines. Argon got it right in one pass, 2,000 of 2,000 tested behaviors, but so might smaller models. libexpat is **[N] lines, [M] tokens**. Argon read the whole library in one long-context pass and wrote **[K] tokens** of Rust. Here's Argon vs Flash on the same task, same verifier: **[numbers]**."

**2:20 Managed Agents and human control.**
"Each run happens in its own Google-hosted sandbox. The agent runs our verifier itself, and we re-verify locally. Nothing ships without a human." Click Approve. "The PR opens with the code and the evidence report."

**2:40 Close (clarity, 10%).**
"Built today: the verifier, the pipeline, the UI. N tested behaviors preserved, in isolated sandboxes. Argon never grades its own homework."

## Q&A answers to have ready

- **"Rust ports of cJSON already exist."** Yes, that's why cJSON is our floor, not our claim. The product is the tool and the evidence. libexpat has no well-known Rust port. **[verify]**
- **"How do you know your verifier is right?"** Three controls: C against C scores 100%, a canary input with one known output, and a do-nothing stub scores near 0%. Our first driver passed C-vs-C while broken. That's why the canary exists.
- **"100% on the first try? Your tests are easy."** We built adversarial inputs, including ones Argon wrote to break its own port. Results: **[fill in]**.
- **"What does match rate exclude?"** Inputs where C crashes are listed separately as safety improvements, never counted as preserved behaviors.
- **"Is the Rust actually safe?"** The crate root has `#![forbid(unsafe_code)]`, which the compiler enforces, plus a build-time check. No dependencies.
- **One divergence story, in depth:** have the best one from docs/FRICTION-LOG.md ready.
