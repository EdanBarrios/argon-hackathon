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
"This is for the people who maintain C libraries that everything depends on, like libexpat, the XML parser behind Python's built-in XML modules and shipped in Firefox's source tree. (Do not quote a team size; we have no verified number.) C lets one bad pointer become a remote takeover. Rust makes that whole class of bug impossible. Maintainers would migrate if they could trust the rewrite. Theseus rewrites their C into safe Rust and proves the new code behaves the same."

**0:25 Live, real input (live demo, 50%).**
Open the Try panel. Ask a judge for any JSON, or type one.
"Same bytes into the original C and into Argon's Rust. Byte-for-byte comparison."
Then the CVE preset `{"1":1,`: "Seven bytes. On cJSON 1.7.17, parsed as an exact-length buffer, they trigger a heap over-read under AddressSanitizer: CVE-2023-53154, fixed in 1.7.18. Argon's Rust returns a clean parse error on the same seven bytes." (This uses a separate old-version driver; the normal reference is 1.7.19, which is already fixed. Present it as a safety demonstration, not proof that Rust alone supplied the fix.)

**1:05 The verification loop (live demo, 50%).**
Start a live run from Flash's first attempt (or fresh at medium thinking). Lanes light up.
"Argon writes the code. It never grades its own homework. Our verifier runs both versions on thousands of inputs. Every divergence goes back to Argon with the exact input and both outputs. Argon repairs. Repeat until it matches."
Click a red divergence: input, C output, Rust output, the function.

**1:45 Why only Argon (originality, 15%).**
Switch to the pre-run.
"cJSON is 3,000 lines. Both Argon and Flash end at 2,000 of 2,000 tested behaviors. The difference is how: Argon did it in one pass at full thinking in about 5.5 minutes. Flash hit its 64K output ceiling twice at high thinking and only finished by thinking less, about 14 minutes and 2.7x the tokens." Then, only if the libexpat runs exist: "libexpat is **[N] lines, [M] tokens**. Argon read the whole library in one long-context pass and wrote **[K] tokens** of Rust. Flash on the same task: **[result]**." Source for cJSON numbers: results/cjson-flash-vs-argon/README.md. Never fill [N]/[M]/[K] by estimate.

**2:20 Managed Agents and human control.**
"Pre-runs happen in Google-hosted sandboxes. The agent runs our verifier itself, and we re-verify locally, because its report is a claim, not evidence. Nothing ships without a human." Click Approve. Say "The PR opens with the code and the evidence report" only if the real PR step has been demonstrated today; otherwise say "Approval is the gate; the PR step runs in a sandbox with a scoped token" and show the dry run.

**2:40 Close (clarity, 10%).**
"Built today: the verifier, the pipeline, the UI. N tested behaviors preserved, in isolated sandboxes. Argon never grades its own homework."

## Q&A answers to have ready

- **"Rust ports of cJSON already exist."** Yes, that's why cJSON is our floor, not our claim. The product is the repeatable migration workflow and the independent evidence. Rust ports of libexpat exist too (the experimental `expat` crate); we don't claim a first port.
- **"How do you know your verifier is right?"** Three controls: C against C scores 100%, a canary input with one known output, and a do-nothing stub scores near 0%. Our first driver passed C-vs-C while broken. That's why the canary exists.
- **"100% on the first try? Your tests are easy."** We built adversarial inputs, including ones Argon wrote to break its own port. Results: Argon's and Flash's ports both matched 958/958 targeted edge inputs and 225/225 inputs Argon wrote to break them. That shows agreement on those corpora, not full equivalence, and it does not separate the two models.
- **"What does match rate exclude?"** Inputs where C crashes are listed separately as safety improvements, never counted as preserved behaviors. Oversized inputs are skipped and counted. It covers parse-and-print stdout and exit status through our driver, not every cJSON function or performance.
- **"Why do you need 1M context?"** Honest answer: cJSON alone doesn't prove it; Flash also reached 2,000/2,000. The case rests on libexpat (only say results if we have them) and on Flash hitting its 64K output ceiling on a 3,000-line library.
- **"Is this a dashboard replaying a success story?"** No: the Try panel runs a judge's own input through the real C and the real Rust live. Name any replayed part out loud.
- **"Could the Rust cheat by calling the C binary?"** The build gate rejects process, filesystem and network access in the generated code (once the hardening branch is merged; until then say a code audit found none in any generated port).
- **"Does the CVE demo prove Rust fixed it?"** It shows the vulnerable release crashing and the Rust returning an error on the same bytes. cJSON fixed it in 1.7.18 too. Rust removes the class of bug, not just this one.
- **"Is the Rust actually safe?"** The crate root has `#![forbid(unsafe_code)]`, which the compiler enforces, plus a build-time check. No dependencies.
- **"How do you know your verifier is right?" (weak spots to admit if pushed)** Embedded NUL bytes in strings are compared only up to the NUL; the canary set is small. Both are known and listed.
- **One divergence story, in depth:** have the best one from docs/FRICTION-LOG.md ready.
