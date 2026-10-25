"""Human-approved PR step: a managed agent pushes the verified crate and opens a PR.

    open_pr_remote(job_dir, approved_by, dry_run=False) -> pr_url
    (dry run returns "dry-run:<branch>@<commit>")

Only call this after a person clicked "Open PR". approved_by is recorded in the
commit, EVIDENCE.md and the PR body.

How secrets are handled. GITHUB_TOKEN (fine-grained, this repo only: Contents
read/write, Pull requests read/write) is read from the environment or the repo
.env and registered with client.credentials.create. The egress proxy injects it
on requests to github.com (git push, Basic auth) and api.github.com (REST,
Bearer). The token never enters the prompt, the sandbox filesystem, or the
sandbox environment variables. The network allowlist is github.com and
api.github.com only. The credentials are deleted again when the step ends.

What we build locally, not the agent: the crate files and EVIDENCE.md come from
the job directory (the verified cjson.rs and our verify.sh reports) and are
mounted into the sandbox as inline sources. The agent only runs git and curl.
After a real run we confirm the PR exists with an unauthenticated GitHub API call.
"""

from __future__ import annotations

import base64
import hashlib
import json
import re
import subprocess
import time
from datetime import datetime
from pathlib import Path

from . import certificate as cert_mod
from ._common import (
    AGENT,
    HARNESS,
    PUBLIC_REPO,
    PUBLIC_REPO_URL,
    REPO_ROOT,
    Event,
    cost_usd,
    emit,
    env_value,
    get_client,
    poll,
    usage_dict,
    write_transcript,
    PreflightError,
    run_preflight,
)

CRED_GIT = "theseus-github-git"
CRED_API = "theseus-github-api"
PAYLOAD = "/workspace/payload"
BEGIN, END = "=== PR_RESULT_BEGIN ===", "=== PR_RESULT_END ==="
CVE_INPUT = HARNESS / "oldcve/inputs/cve_2023_53154.json"
OLD_DRIVER = HARNESS / "oldcve/build/cjson_driver_old"
MAX_WAIT = 20 * 60


class PRError(RuntimeError):
    pass


# ---------------------------------------------------------------- evidence


def _cve_check(rust_binary: str | None) -> dict:
    """Run the CVE-2023-53154 PoC through the old ASan C driver and through our Rust build."""
    out: dict = {"input": CVE_INPUT.read_bytes().decode("latin-1") if CVE_INPUT.exists() else None}
    if OLD_DRIVER.exists() and CVE_INPUT.exists():
        p = subprocess.run([str(OLD_DRIVER)], input=CVE_INPUT.read_bytes(), capture_output=True, timeout=60)
        summary = next((ln for ln in p.stderr.decode(errors="replace").splitlines()
                        if ln.startswith("SUMMARY: AddressSanitizer")), None)
        out["c_v1_7_17"] = {"exit_code": p.returncode, "asan_summary": summary}
    if rust_binary and Path(rust_binary).exists() and CVE_INPUT.exists():
        p = subprocess.run([rust_binary], input=CVE_INPUT.read_bytes(), capture_output=True, timeout=60)
        out["rust"] = {"exit_code": p.returncode, "stdout": p.stdout.decode(errors="replace").strip()}
    return out


def _round_rows(jd: Path, result: dict) -> list[dict]:
    rows = []
    claims = {c["round"]: c for c in result.get("claimed_rounds") or []}
    files = {int(re.search(r"\d+", p.name).group(0)): p for p in (jd / "sandbox_work").glob("verify_*.json")}
    for n in sorted(set(claims) | set(files)):
        row = {"round": n, "build": "ok" if claims.get(n, {}).get("build_ok", n in files) else "fail",
               "match_rate": claims.get(n, {}).get("match_rate"), "divergence": None, "rust_crash": None}
        if n in files:
            r = json.loads(files[n].read_text())
            row.update(match_rate=r.get("match_rate"), divergence=r["counts"].get("divergence"),
                       rust_crash=r["counts"].get("rust_crash"))
        rows.append(row)
    return rows


def _spec(jd: Path) -> dict:
    try:
        return json.loads((jd / "spec.json").read_text())
    except Exception:
        return {}


def _fixed_divergences(jd: Path, limit: int = 8) -> list[str]:
    """Divergent inputs from earlier sandbox rounds (the bugs the repair loop fixed)."""
    out = []
    files = sorted((jd / "sandbox_work").glob("verify_*.json"), key=lambda p: int(re.search(r"\d+", p.name).group(0)))
    for f in files[:-1]:
        for d in json.loads(f.read_text()).get("divergences", [])[:limit]:
            out.append(f"- {f.stem.replace('verify_', 'round ')}: input `{d.get('input')}`: C `{d.get('c_output')}`, "
                       f"Rust `{d.get('rust_output')}`")
    return out[:limit]


def _rate(x) -> str:
    return "n/a" if x is None else f"{x:.4f}"


def build_evidence(jd: Path, result: dict, approved_by: str, branch: str, cert: dict | None = None) -> str:
    cert = cert or cert_mod.build_certificate(jd, result, approved_by, branch)
    cert_head, cert_tail = cert_mod.evidence_sections(cert)
    local = result.get("local_verify") or {}
    held = result.get("holdout_verify") or {}
    lc, hc = local.get("counts") or {}, held.get("counts") or {}
    rows = _round_rows(jd, result)
    verify_label = result.get("verify_label") or (
        f"Host re-verify, seed {_spec(jd).get('seed', '?')} (same inputs as the sandbox)")
    cve = _cve_check((result.get("local_build") or {}).get("binary"))
    usage = result.get("usage") or {}
    lines = [
        "# EVIDENCE: cJSON v1.7.19 parse + print, C to safe Rust",
        "",
        "Every number below was produced by `backend/harness/verify.sh` (differential: same input bytes to the",
        "ASan/UBSan C driver and the Rust driver, raw stdout compared). The model did not grade itself.",
        "Agreement on the tested inputs below is evidence, not a proof of equivalence.",
        "",
        *cert_head,
        "## Result",
        "",
        "| | match rate | behaviors tested | divergences | Rust crashes | timeouts |",
        "|---|---|---|---|---|---|",
        f"| {verify_label} "
        f"| {_rate(local.get('match_rate'))} | {local.get('behaviors_tested')} | "
        f"{lc.get('divergence')} | {lc.get('rust_crash')} | {lc.get('timeout')} |",
        f"| Host held-out, seed {held.get('seed')} (inputs the agent never saw, {held.get('overlap_removed')} overlaps removed) | "
        f"{_rate(held.get('match_rate'))} | {held.get('behaviors_tested')} | {hc.get('divergence')} | "
        f"{hc.get('rust_crash')} | {hc.get('timeout')} |",
        f"| Sandbox claim (verify.sh JSON the agent pasted) | {_rate(result.get('sandbox_match_rate'))} | | | | |",
        "",
        f"Sandbox claim and host re-verify agree: **{result.get('match_rates_agree')}**.",
        "",
        f"## Divergences found and fixed, per round ({result.get('rounds_label') or 'sandbox verify.sh reports'})",
        "",
        "| round | build | match rate | divergent inputs | Rust crashes |",
        "|---|---|---|---|---|",
        *[f"| {r['round']} | {r['build']} | {_rate(r['match_rate'])} | {r['divergence']} | {r['rust_crash']} |" for r in rows],
        "",
        *(["Divergences the repair rounds fixed (input, C output, Rust output before the fix):", ""]
          + _fixed_divergences(jd) + [""] if _fixed_divergences(jd) else []),
        "## Crashes",
        "",
        f"Rust crashes (panic, signal) on the host run: {lc.get('rust_crash')}. Both crashed: {lc.get('both_crash')}. "
        f"C-only crashes where Rust exited cleanly (safety_improvement): {lc.get('safety_improvement')}.",
        "",
        "## Safety improvement: CVE-2023-53154",
        "",
        "cJSON v1.7.17 reads one byte past the end of the input buffer on an object that ends right after a comma",
        "(`{\"1\":1,`), fixed upstream in v1.7.18. Same 7-byte PoC, run on the host:",
        "",
        f"- C v1.7.17 with ASan: `{(cve.get('c_v1_7_17') or {}).get('asan_summary') or 'old driver not built here'}`",
        f"- This Rust crate: `{(cve.get('rust') or {}).get('stdout') or 'n/a'}` (exit {(cve.get('rust') or {}).get('exit_code')})",
        "",
        "Safe Rust cannot express the out-of-bounds read: indexing is bounds checked and the crate has",
        "`#![forbid(unsafe_code)]`; the build script rejects the word `unsafe` anywhere in cjson.rs.",
        "",
        "## Provenance",
        "",
        f"- Model: `{result.get('model')}` inside managed agent `{AGENT}`",
        f"- Sandbox seconds: {result.get('seconds')}; tokens: input {usage.get('total_input_tokens')}, output "
        f"{usage.get('total_output_tokens')}, thought {usage.get('total_thought_tokens')}, cached "
        f"{usage.get('total_cached_tokens')}; estimated cost ${result.get('cost_usd')}",
        f"- Interaction `{result.get('interaction_id')}`, environment `{result.get('environment_id')}`",
        f"- cjson.rs sha256 `{hashlib.sha256(Path(result['rust_source']).read_bytes()).hexdigest()}`",
        f"- Approved by **{approved_by}** at {datetime.now().isoformat(timespec='seconds')}; branch `{branch}`",
        "",
        *cert_tail,
    ]
    return "\n".join(lines)


def build_payload(jd: Path, result: dict, approved_by: str, branch: str) -> dict[str, bytes]:
    """The migrated/cjson/ files: crate, EVIDENCE.md, certificate.json and (when the
    job's inputs come from a seed) reproduce.sh. Cheap and deterministic: reads files only."""
    crate = HARNESS / "rust_crate"
    cert = cert_mod.build_certificate(jd, result, approved_by, branch)
    try:
        script = cert_mod.reproduce_script(cert)
    except ValueError as exc:
        script = None
        cert["reproduce"] = None
        cert["reproduce_unavailable"] = str(exc)
    files = {
        "Cargo.toml": (crate / "Cargo.toml").read_bytes(),
        "src/main.rs": (crate / "src/main.rs").read_bytes(),
        "src/cjson.rs": Path(result["rust_source"]).read_bytes(),
        ".gitignore": b"target/\n",
        "EVIDENCE.md": build_evidence(jd, result, approved_by, branch, cert).encode(),
        "certificate.json": (json.dumps(cert, indent=2) + "\n").encode(),
    }
    if script:
        files["reproduce.sh"] = script.encode()
    if (crate / "Cargo.lock").exists():
        files["Cargo.lock"] = (crate / "Cargo.lock").read_bytes()
    return files


# ---------------------------------------------------------------- credentials


def _register_credentials(client, token: str, notes: list[str]) -> None:
    """Store the GitHub token server-side. The proxy injects it; it is never returned."""
    basic = base64.b64encode(f"x-oauth-basic:{token}".encode()).decode()
    for cid, kwargs in (
        (CRED_GIT, {"type": "bearer_token", "token": basic, "prefix": "Basic"}),
        (CRED_API, {"type": "bearer_token", "token": token, "prefix": "Bearer"}),
    ):
        try:
            client.credentials.delete(id=cid)
        except Exception:
            pass
        try:
            cred = client.credentials.create(id=cid, **kwargs)
        except Exception as exc:
            # Never re-raise the SDK error as is: validation errors echo the request body (the token).
            raise PRError(f"credentials.create({cid}) failed: {type(exc).__name__}") from None
        notes.append(f"credential {cred.id} status {getattr(cred, 'status', None)}")


def _delete_credentials(client, notes: list[str]) -> None:
    for cid in (CRED_GIT, CRED_API):
        try:
            client.credentials.delete(id=cid)
            notes.append(f"credential {cid} deleted")
        except Exception as exc:
            notes.append(f"credential {cid} delete failed: {type(exc).__name__}")


# ---------------------------------------------------------------- prompt


def build_prompt(branch: str, approved_by: str, title: str, dry_run: bool) -> str:
    finish = (
        f"""5. DRY RUN: do NOT push and do NOT call the pulls API. Instead prove both hosts are reachable:
   `git ls-remote --heads origin main | head -1` and
   `curl -s -o /dev/null -w '%{{http_code}}\\n' https://api.github.com/repos/{PUBLIC_REPO}`
   Set "pushed": false and "pr_url": null."""
        if dry_run else
        f"""5. `git push origin {branch}`
6. `curl -sS -X POST -H 'Accept: application/vnd.github+json' https://api.github.com/repos/{PUBLIC_REPO}/pulls --data @{PAYLOAD}/pr.json`
   Do not add any Authorization header yourself: the network proxy adds it. Take "html_url" from the response.
   If push or the API call fails, report the error output verbatim and set "pr_url": null."""
    )
    return f"""You are the Theseus release agent. A human ({approved_by}) approved opening a pull request for a
verified C to Rust migration. Run these commands in the sandbox shell, exactly, and report their
output verbatim. Do not edit any file content: every file you commit is already prepared under
{PAYLOAD}/ and was verified by the host. Set GIT_TERMINAL_PROMPT=0 for every git command.

1. `git clone {PUBLIC_REPO_URL} /workspace/pr && cd /workspace/pr && git checkout -b {branch}`
2. `mkdir -p migrated/cjson && cp -R {PAYLOAD}/crate/. migrated/cjson/ && (test ! -f migrated/cjson/reproduce.sh || chmod +x migrated/cjson/reproduce.sh)`
3. `git add migrated/cjson && git -c user.name='Theseus agent' -c user.email='theseus-agent@users.noreply.github.com' commit -q -m '{title}' -m 'Approved-by: {approved_by}'`
4. `git log -1 --stat && sha256sum migrated/cjson/src/cjson.rs && git rev-parse HEAD`
{finish}

Final reply: after the command outputs, end with exactly this block (parsed by a program):
{BEGIN}
{{"branch": "{branch}", "commit": "<git rev-parse HEAD>", "cjson_rs_sha256": "<from sha256sum>", "files": [<paths committed>], "pushed": <true|false>, "pr_url": <"https://..." or null>}}
{END}
"""


# ---------------------------------------------------------------- main


def _parse_result(text: str) -> dict | None:
    i = text.rfind(BEGIN)
    j = text.find(END, i + 1) if i >= 0 else -1
    if i < 0 or j < 0:
        return None
    m = re.search(r"\{.*\}", text[i + len(BEGIN):j], re.DOTALL)
    try:
        return json.loads(m.group(0)) if m else None
    except json.JSONDecodeError:
        return None


def _confirm_pr(branch: str) -> str | None:
    """Unauthenticated GitHub API check that the PR really exists (public repo)."""
    import httpx

    owner = PUBLIC_REPO.split("/")[0]
    try:
        r = httpx.get(f"https://api.github.com/repos/{PUBLIC_REPO}/pulls",
                      params={"head": f"{owner}:{branch}", "state": "all"}, timeout=20)
        prs = r.json() if r.status_code == 200 else []
        return prs[0]["html_url"] if prs else None
    except Exception:
        return None


def open_pr_remote(
    job_dir: str | Path,
    approved_by: str,
    *,
    dry_run: bool = False,
    model: str = "gemini-4-argon",
    allow_partial: bool = False,
    on_event: Event | None = None,
    skip_preflight: bool = False,
    refresh_reports: bool = True,
) -> str:
    """Push migrated/cjson/ (crate + EVIDENCE.md) on branch theseus/<job> and open a PR against main.

    job_dir is a run_module_remote job directory (it holds result.json and cjson.rs).
    Raises PRError if the job was not verified locally, approval is missing, the
    token is missing (real run), or the agent did not report a PR. Returns the PR
    URL, or "dry-run:<branch>@<commit>" with dry_run=True.
    """
    jd = Path(job_dir).resolve()
    # Shell-safe: the name goes into a git command line inside the sandbox.
    approved_by = re.sub(r"[^A-Za-z0-9 ._@+-]", "", approved_by or "").strip()
    if not approved_by:
        raise PRError("approved_by is required: a person must approve before any PR")
    result = json.loads((jd / "result.json").read_text())
    if result.get("local_match_rate") is None or not result.get("rust_source"):
        raise PRError("job has no locally re-verified cjson.rs; refusing to open a PR")
    if result["local_match_rate"] != 1.0 and not allow_partial:
        raise PRError(f"local match_rate is {result['local_match_rate']}, not 1.0 (pass allow_partial=True to override)")
    token = env_value("GITHUB_TOKEN")
    if not dry_run and not token:
        raise PRError("GITHUB_TOKEN is not set (fine-grained token on this repo: Contents RW, Pull requests RW)")

    # The certificate records whether the verifier itself was healthy when we signed off.
    if not skip_preflight:
        try:
            pf = run_preflight(result.get("library") or "cjson", on_event=on_event)
        except PreflightError as exc:
            raise PRError(str(exc)) from None
        (jd / "preflight-pr.json").write_text(json.dumps(pf, indent=2))
    # Older host reports predate verify.py's `summary` object: re-run the host verify
    # (same seed, same held-out rule) so the certificate's counts come from a real report.
    if refresh_reports and (jd / "inputs").is_dir() and (jd / "spec.json").exists():
        stale = [n for n in ("verify_local.json", "verify_holdout.json")
                 if not (cert_mod._read_json(jd / n) or {}).get("summary")]
        if stale:
            from .run_remote import reverify_job

            emit(on_event, f"[pr] {', '.join(stale)} has no summary object: re-running host verify")
            reverify_job(jd)
            result = json.loads((jd / "result.json").read_text())
            if result.get("local_match_rate") != 1.0 and not allow_partial:
                raise PRError(f"host re-verify gave match_rate {result.get('local_match_rate')}, not 1.0")

    job = re.sub(r"[^A-Za-z0-9._-]+", "-", f"{jd.parent.name}-{jd.name}").strip("-")
    branch = f"theseus/{job}"
    title = f"Theseus: migrate cJSON parse+print to safe Rust ({result.get('model')})"
    out = jd / ("pr-dry-run" if dry_run else "pr")
    out.mkdir(exist_ok=True)
    notes: list[str] = []

    files = build_payload(jd, result, approved_by, branch)
    for rel, data in files.items():
        (out / "crate" / rel).parent.mkdir(parents=True, exist_ok=True)
        (out / "crate" / rel).write_bytes(data)
    evidence = files["EVIDENCE.md"].decode()
    pr_body = evidence + f"\n---\nOpened by the Theseus release agent after approval by **{approved_by}**.\n"
    pr_json = json.dumps({"title": title, "head": branch, "base": "main", "body": pr_body}).encode()
    sources = [{"type": "inline", "target": f"{PAYLOAD}/crate/{rel}", "encoding": "base64",
                "content": base64.b64encode(data).decode()} for rel, data in files.items()]
    sources.append({"type": "inline", "target": f"{PAYLOAD}/pr.json", "encoding": "base64",
                    "content": base64.b64encode(pr_json).decode()})
    (out / "approval.json").write_text(json.dumps(
        {"approved_by": approved_by, "at": datetime.now().isoformat(timespec="seconds"),
         "branch": branch, "dry_run": dry_run}, indent=2))

    client = get_client()
    use_creds = bool(token)
    if use_creds:
        _register_credentials(client, token, notes)
    else:
        notes.append("GITHUB_TOKEN not set: dry run without credentials (public clone only)")
    allowlist = [
        {"domain": "github.com", **({"credential": CRED_GIT} if use_creds else {})},
        {"domain": "api.github.com", **({"credential": CRED_API} if use_creds else {})},
    ]
    prompt = build_prompt(branch, approved_by, title, dry_run)
    (out / "prompt.md").write_text(prompt)
    started = time.monotonic()
    try:
        interaction = client.interactions.create(
            agent=AGENT,
            agent_config={"type": "antigravity", "model": model},
            input=prompt,
            environment={"type": "remote", "sources": sources, "network": {"allowlist": allowlist}},
            background=True,
        )
        emit(on_event, f"[pr] interaction {interaction.id} {interaction.status}")
        interaction = poll(client, interaction, max_wait=MAX_WAIT, started=started, on_event=on_event, tag="[pr] ")
    finally:
        if use_creds:
            _delete_credentials(client, notes)

    text = getattr(interaction, "output_text", None) or ""
    (out / "agent_output.md").write_text(text)
    write_transcript(interaction, out / "transcript.jsonl")
    usage = usage_dict(interaction)
    parsed = _parse_result(text)
    local_sha = hashlib.sha256(files["src/cjson.rs"]).hexdigest()
    record = {"status": interaction.status, "interaction_id": interaction.id,
              "environment_id": getattr(interaction, "environment_id", None),
              "seconds": round(time.monotonic() - started, 1), "usage": usage, "cost_usd": cost_usd(usage),
              "agent_result": parsed, "local_cjson_rs_sha256": local_sha,
              "sha_matches": bool(parsed) and parsed.get("cjson_rs_sha256") == local_sha,
              "branch": branch, "dry_run": dry_run, "notes": notes}
    if not dry_run:
        record["confirmed_pr_url"] = _confirm_pr(branch)
    (out / "pr_result.json").write_text(json.dumps(record, indent=2, default=str))

    if not parsed:
        raise PRError(f"agent did not return a PR_RESULT block (status {interaction.status}); see {out}")
    if dry_run:
        return f"dry-run:{branch}@{parsed.get('commit')}"
    url = record.get("confirmed_pr_url") or parsed.get("pr_url")
    if not url:
        raise PRError(f"no PR was opened; see {out / 'agent_output.md'}")
    return url
