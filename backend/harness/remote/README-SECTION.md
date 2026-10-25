<!-- Draft for the root README section "How Theseus uses Managed Agents". Edan owns the root README. -->

## How Theseus uses Managed Agents

Every module migration runs in its own isolated antigravity sandbox (`antigravity-preview-09-2026`,
model `gemini-4-argon`), one sandbox per module and per model, so an Argon run and a Flash baseline
run side by side without sharing a filesystem. Inside the sandbox the agent sets up its own
toolchain (clang with ASan, rustup), clones this repo, builds the original C library, writes the
safe-Rust translation, and then runs our deterministic differential verifier, `verify.sh`, itself.
It reads the divergences, repairs, and re-runs until the match rate is 1.0 or the round budget is
spent. The prompt says verify.sh is the only judge, and the agent must hand back the verifier's
JSON verbatim, not its own opinion.

We do not take the agent's word for it. When a sandbox finishes, the host pulls the sandbox's own
verify reports through the Environments files API, rebuilds the returned `cjson.rs` locally, and
re-runs `verify.sh` locally on the same inputs and on held-out inputs the agent never saw. The
sandbox claim and the local number are stored side by side, and any disagreement is flagged.
In our first real run, Argon reached match rate 1.0 on 300 inputs in 2 rounds and 517 seconds
inside the sandbox, and the host re-verify reproduced 1.0 on the same 300 inputs and on 64 unseen
ones.

Opening a pull request is the only step that touches a credential, and it needs a human. After
someone clicks "Open PR", a second managed agent clones the repo, commits `migrated/cjson/` (the
crate plus an `EVIDENCE.md` the host generated from verifier output), pushes a `theseus/<job>`
branch and opens the PR. The GitHub token is stored with the Credentials API and injected by the
sandbox's egress proxy on requests to github.com and api.github.com, the only two hosts on that
sandbox's allowlist. It never appears in the prompt, the sandbox filesystem, or its environment
variables, and the credentials are deleted when the step ends.

Code: `backend/harness/remote/` (built on `backend/ai/sandbox_run.py`).
