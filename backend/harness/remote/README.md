# backend/harness/remote

Owner: **C (systems)**. Managed-agent sandbox runs (antigravity-preview-09-2026) as functions the
API pipeline can call, parallel model baselines, and the human-approved PR step.

Built on Brendan's `backend/ai/sandbox_run.py`. Reused, not copied: the sandbox task prompt
(`prompts.sandbox_input`, `prompts.SYSTEM_INSTRUCTION`), the apt-mirror allowlist
(`sandbox_run.ENVIRONMENT`), `parse_claims`, `host_verify`, `extract_rust`, `ensure_inputs` and
`holdout_inputs`. Added here: a function interface, parallel jobs, per-job directories, step
transcripts, progress events, a pull of the sandbox's own verify reports through the
Environments files API, and the PR step.

## Interfaces

```python
from harness.remote import run_module_remote, run_parallel, RemoteSpec, open_pr_remote  # cwd: backend/

r = run_module_remote(library="cjson", model="gemini-4-argon", rounds=4, inputs=300,
                      job_dir=None, on_event=print)          # blocking, tens of minutes
r.local_match_rate, r.holdout_match_rate, r.sandbox_match_rate, r.match_rates_agree
r.interaction_id, r.environment_id, r.seconds, r.usage, r.cost_usd, r.rust_source, r.transcript, r.job_dir

rs = run_parallel([RemoteSpec(model="gemini-4-argon"), RemoteSpec(model="gemini-3.8-flash")],
                  job_root=None, on_event=print)            # one thread + one sandbox per spec

url = open_pr_remote(r.job_dir, approved_by="Edan", dry_run=False)   # only after a human clicks
```

Both run the verifier preflight gate (`scripts/preflight.sh <lib>`) before any model call:
`run_module_remote` returns status "error" with a `PREFLIGHT:` message, `run_parallel` raises
`PreflightError`. Pass `skip_preflight=True` only if the caller already ran it.
`open_pr_remote` also runs it and records the result in the certificate.

`run_module_remote` never raises for agent, build or verify failures: check `status`, `error`
and `local_match_rate`. `rounds` is total rounds (1 translate + rounds-1 repairs), the team
convention. `resume_remote(interaction_id, job_dir)` re-attaches if the client process dies.

## CLI

```bash
cd backend
uv run python ../scripts/run_remote.py --library cjson --models gemini-4-argon,gemini-3.8-flash --rounds 4
uv run python ../scripts/open_pr_remote.py runs/remote/<stamp>/cjson-gemini-4-argon --approved-by Edan --dry-run
```

Output: `runs/remote/<stamp>/summary.json` and one directory per job with `prompt.md`,
`transcript.jsonl` (every agent step), `agent_output.md`, `cjson.rs`, `sandbox_work/` (the
sandbox's own `verify_<N>.json` and `cjson.rs`), `verify_local.json`, `verify_holdout.json` and
`result.json`.

## How it works

1. **One isolated sandbox per module and model.** The agent installs clang and rustup, clones
   this public repo, fetches cJSON, builds the ASan/UBSan C driver, generates inputs, writes
   `cjson.rs` against `rust_crate/INTERFACE.md`, builds it with `scripts/build_rust.sh`, and runs
   `verify.sh` against the C driver, repairing until match_rate 1.0 or the budget runs out.
2. **verify.sh is the only judge.** The prompt says so, and the agent must paste the final
   verify.sh JSON summary verbatim between markers. We also pull the sandbox's
   `verify_<N>.json` files directly and compare.
3. **Trust but verify.** The host rebuilds the returned `cjson.rs` and re-runs `verify.sh`
   locally on the same seed (`local_match_rate`, must equal the sandbox claim; a mismatch is
   flagged `MISMATCH`) and on a held-out seed the agent never saw (`holdout_match_rate`, the
   number to quote).
4. **The PR is a migration certificate.** `migrated/cjson/` gets the crate, `EVIDENCE.md`,
   `certificate.json` (scope, model and interaction ids, harness commit, pinned C commit, the
   verify reports' `summary` counts for the same-seed and held-out sets, sandbox claim vs host
   re-verify, preflight status, known limitations) and `reproduce.sh` (one command from a fresh
   clone: fetch and build the C, build the crate, regenerate the same inputs, verify, compare).
5. **PR only after human approval.** `open_pr_remote` refuses without `approved_by` or a local
   match rate of 1.0. The host builds the crate and `EVIDENCE.md` from verified files and mounts
   them as inline sources; the agent only runs git and curl.
6. **Credentials at the proxy.** `GITHUB_TOKEN` is registered with `client.credentials.create`
   (github.com: Basic `x-oauth-basic:<token>`; api.github.com: Bearer) and attached to allowlist
   rules. The allowlist is github.com and api.github.com only. The token never enters the
   prompt or the sandbox, and the credentials are deleted when the step ends.

## Token for the PR step

Fine-grained personal access token, repository access **only EdanBarrios/argon-hackathon**,
permissions **Contents: Read and write** and **Pull requests: Read and write** (Metadata: Read is
added automatically). Put it in `.env` as `GITHUB_TOKEN=...`.

## Verify-only runs (expat)

```python
from harness.remote import run_verify_remote   # cwd: backend/
r = run_verify_remote("expat", rust_file="backend/ai/results/.../expat.rs", seed=7777, count=2000,
                      host_binary=None, on_event=print)
r["sandbox_claim"]["match_rate"], r["host"]["match_rate"], r["match_rates_agree"], r["inputs_digest_agree"]
```

No model writes code. The sandbox clones this repo, fetches and builds libexpat with the team
scripts, builds the given `expat.rs` with `scripts/build_rust_expat.sh`, generates inputs at `seed`,
runs `verify.sh --timeout 20`, and reports the expat.rs sha256, an inputs digest and the verify
summary. The host regenerates the same inputs, checks the digest and sha256, and runs the same
verify locally (in parallel with the sandbox). Output: `runs/remote/<stamp>-expat-verify/`.
The allowlist adds www.w3.org for the W3C suite; the pinned tarball is also mounted as a fallback.
Offline test: `cd backend && uv run python -m pytest harness/remote/test_verify_remote.py -q`.
