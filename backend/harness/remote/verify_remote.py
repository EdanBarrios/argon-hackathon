"""Verify-only managed-agent run: the sandbox builds and verifies an existing port, no model writes code.

    run_verify_remote(library="expat", rust_file=..., seed=7777, count=2000) -> dict

The sandbox clones this public repo, fetches and builds libexpat R_2_9_0 with the
team scripts (ASan + UBSan), builds the given expat.rs with scripts/build_rust_expat.sh
(banned-API guard included), generates inputs with gen_inputs_expat.py at `seed`,
runs verify.sh --timeout 20, and pastes the verify summary. That is a CLAIM.

The host then regenerates the same seed's inputs, checks the inputs digest and the
expat.rs sha256 against the sandbox's, runs the same verify.sh locally against a
host-built binary, and records claim vs host (match_rates_agree).

The prompt forbids writing, translating or repairing code. Nothing secret goes in.
"""

from __future__ import annotations

import base64
import hashlib
import io
import json
import os
import re
import subprocess
import sys
import tarfile
import threading
import time
from datetime import datetime
from pathlib import Path

from ai.loop import run_json
from ai.sandbox_run import ALLOWLIST

from ._common import (AGENT, PUBLIC_REPO_URL, REPO_ROOT, Event, PreflightError, cost_usd, emit, get_client, poll,
                      run_preflight, step_types, usage_dict, write_transcript)
from .run_remote import BEGIN_VERIFY, END_VERIFY, parse_verify_block

EXPAT = REPO_ROOT / "backend/harness/expat"
XMLTS_TGZ = EXPAT / "vendor/xmlts20130923.tar.gz"
MOUNT = "/work/mount"
OUT = "/work/out"
DEFAULT_RUST = "backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs"

# Same digest the sandbox computes (the one-liner in the prompt must stay identical in meaning).
DIGEST_PY = ("import hashlib,os,sys;d=sys.argv[1];h=hashlib.sha256();ns=sorted(os.listdir(d));"
             "[h.update(n.encode()+b'\\0'+hashlib.sha256(open(os.path.join(d,n),'rb').read()).digest()) for n in ns];"
             "print(h.hexdigest(),len(ns))")

SUMMARY_PY = ("import json;r=json.load(open('/work/out/verify.json'));"
              "o={k:r.get(k) for k in ('match_rate','behaviors_tested','counts')};"
              "o['divergences']=(r.get('divergences') or [])[:5];o['crashes']=(r.get('crashes') or [])[:5];"
              "print(json.dumps(o,separators=(',',':')))")


def inputs_digest(d: Path) -> tuple[str, int]:
    """sha256 over sorted (name, sha256(bytes)) pairs. Must match DIGEST_PY."""
    h = hashlib.sha256()
    names = sorted(os.listdir(d))
    for n in names:
        h.update(n.encode() + b"\0" + hashlib.sha256((d / n).read_bytes()).digest())
    return h.hexdigest(), len(names)


VERIFY_TASK = """\
You are in a remote Linux sandbox with a shell. This is a VERIFICATION job only. \
You must NOT write, translate, repair, or edit any code, and must not modify any file \
under /work/theseus. You only run the commands below, in order, and report their output. \
If a step fails, report its command and output verbatim and stop. Shell state does not \
persist between commands, so start every command that needs Rust with `. "$HOME/.cargo/env";`.

Before each numbered step N, run `mkdir -p {out} && echo "step N $(date +%s)" >> {out}/stages.txt`.

1. Unless `clang` exists: `DEBIAN_FRONTEND=noninteractive apt-get update -qq && \
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq clang git curl python3 perl ca-certificates`
2. Unless `$HOME/.cargo/bin/rustc` exists: `curl --proto '=https' --tlsv1.2 -sSf \
https://sh.rustup.rs | sh -s -- -y --profile minimal`
3. Unless /work/theseus exists: `git clone --depth 1 --branch {branch} {repo} /work/theseus`. \
Run every later command from /work/theseus. Then `git rev-parse HEAD > {out}/commit.txt`.
4. `backend/harness/expat/fetch_expat.sh`. It downloads libexpat R_2_9_0 from github.com \
and the W3C suite from www.w3.org, both pinned (commit and sha256). If and only if the W3C \
download fails, run `mkdir -p backend/harness/expat/vendor && cp {mount}/xmlts20130923.tar.gz \
backend/harness/expat/vendor/` and run fetch_expat.sh again (it still checks the pinned \
sha256), and say so in the final reply as `XMLTS_SOURCE mounted`; otherwise `XMLTS_SOURCE w3.org`.
5. `backend/harness/expat/build_expat.sh`
6. `. "$HOME/.cargo/env"; scripts/build_rust_expat.sh {rust} sandbox | tee {out}/build.json`. \
It prints one JSON line; "binary" is the Rust driver. If "ok" is false, stop.
7. `sha256sum {rust} | tee {out}/expat_rs.sha256`
8. `python3 -I backend/harness/expat/gen_inputs_expat.py --seed {seed} --count {count} --out /work/inputs`
9. `python3 -I -c "{digest}" /work/inputs | tee {out}/inputs_digest.txt`
10. `nproc > {out}/nproc.txt; backend/harness/verify.sh --c backend/harness/expat/build/expat_driver \
--rust <binary from step 6> --inputs /work/inputs --timeout 20 --jobs $(nproc) --out {out}/verify.json > /dev/null`
11. `python3 -I -c "{summary}"`
12. `echo "done $(date +%s)" >> {out}/stages.txt`

verify.sh is the ONLY judge. Never state a match rate from your own reasoning.

Final reply, in this format and nothing else:
XMLTS_SOURCE <w3.org|mounted>
COMMIT <contents of {out}/commit.txt>
SHA256 <the hex from step 7>
INPUTS_DIGEST <the two values printed by step 9>
{begin}
<the one line of JSON printed by step 11, verbatim>
{end}
"""


def build_verify_request(rust_rel: str, seed: int, count: int, *, model: str = "gemini-4-argon",
                         branch: str = "main", mount_xmlts: bool = True) -> dict:
    task = VERIFY_TASK.format(out=OUT, mount=MOUNT, branch=branch, repo=PUBLIC_REPO_URL, rust=rust_rel, seed=seed,
                              count=count, digest=DIGEST_PY, summary=SUMMARY_PY, begin=BEGIN_VERIFY, end=END_VERIFY)
    env: dict = {"type": "remote",
                 "network": {"allowlist": [{"domain": d} for d in [*ALLOWLIST, "www.w3.org"]]}}
    if mount_xmlts and XMLTS_TGZ.is_file():
        env["sources"] = [{"type": "inline", "target": f"{MOUNT}/xmlts20130923.tar.gz", "encoding": "base64",
                           "content": base64.b64encode(XMLTS_TGZ.read_bytes()).decode()}]
    return {
        "agent": AGENT,
        "agent_config": {"type": "antigravity", "model": model},
        "system_instruction": "You run shell commands exactly as instructed and report their output verbatim.",
        "input": task,
        "environment": env,
        "background": True,
    }


def parse_claim(text: str | None) -> dict:
    t = text or ""
    out: dict = {"verify": parse_verify_block(t)}
    m = re.search(r"^\s*SHA256\s+([0-9a-f]{64})", t, re.MULTILINE)
    out["expat_rs_sha256"] = m.group(1) if m else None
    m = re.search(r"^\s*INPUTS_DIGEST\s+([0-9a-f]{64})\s+(\d+)", t, re.MULTILINE)
    out["inputs_digest"], out["inputs_count"] = (m.group(1), int(m.group(2))) if m else (None, None)
    m = re.search(r"^\s*XMLTS_SOURCE\s+(\S+)", t, re.MULTILINE)
    out["xmlts_source"] = m.group(1) if m else None
    m = re.search(r"^\s*COMMIT\s+([0-9a-f]{7,40})", t, re.MULTILINE)
    out["commit"] = m.group(1) if m else None
    v = out["verify"]
    out["match_rate"] = float(v["match_rate"]) if v and isinstance(v.get("match_rate"), (int, float)) else None
    return out


def compare(claim: dict, host: dict) -> dict:
    """Claim vs host check. Each field is True, False, or None (missing on one side)."""
    def eq(a, b):
        return None if a is None or b is None else a == b
    rate = (None if claim.get("match_rate") is None or host.get("match_rate") is None
            else abs(claim["match_rate"] - host["match_rate"]) < 1e-9)
    return {"match_rates_agree": rate,
            "inputs_digest_agree": eq(claim.get("inputs_digest"), host.get("inputs_digest")),
            "expat_rs_sha256_agree": eq(claim.get("expat_rs_sha256"), host.get("expat_rs_sha256")),
            "counts_agree": eq((claim.get("verify") or {}).get("counts"), host.get("counts"))}


def _pull_out(client, env_id: str, dest: Path, notes: list[str]) -> list[str]:
    for path in (OUT, OUT.lstrip("/")):
        try:
            data = client.environments.files.download(environment=env_id, path=path)
        except Exception as exc:
            notes.append(f"files.download({path!r}): {type(exc).__name__}: {str(exc)[:160]}")
            continue
        got = []
        try:
            with tarfile.open(fileobj=io.BytesIO(data)) as tar:
                for m in tar.getmembers():
                    name = Path(m.name).name
                    if m.isfile() and re.fullmatch(r"[A-Za-z0-9_.-]+", name):
                        fh = tar.extractfile(m)
                        if fh:
                            dest.mkdir(parents=True, exist_ok=True)
                            (dest / name).write_bytes(fh.read())
                            got.append(name)
        except tarfile.TarError:
            notes.append(f"files.download({path!r}) returned non-tar data ({len(data)} bytes)")
            continue
        notes.append(f"pulled {got} from sandbox {path}")
        return got
    return []


def host_check(rust_file: Path, binary: Path | None, seed: int, count: int, jd: Path,
               on_event: Event | None = None) -> dict:
    """Regenerate the same seed's inputs and run verify.sh locally. Independent of the sandbox."""
    t0 = time.monotonic()
    host: dict = {"expat_rs_sha256": hashlib.sha256(rust_file.read_bytes()).hexdigest(), "seed": seed, "count": count}
    if binary is None:
        build, raw = run_json([str(REPO_ROOT / "scripts/build_rust_expat.sh"), str(rust_file), f"host-{jd.name}"],
                              timeout=900)
        if not build or not build.get("ok"):
            host["error"] = f"host build failed: {(build or {}).get('errors') or raw[-1000:]}"
            return host
        binary = Path(build["binary"])
    host["binary"] = str(binary)
    driver = EXPAT / "build/expat_driver"
    subprocess.run([str(EXPAT / "build_expat.sh")], check=True, capture_output=True)
    inputs = jd / "host_inputs"
    subprocess.run([sys.executable, "-I", str(EXPAT / "gen_inputs_expat.py"), "--seed", str(seed), "--count",
                    str(count), "--out", str(inputs)], check=True, capture_output=True)
    host["inputs_digest"], host["inputs_count"] = inputs_digest(inputs)
    host["gen_seconds"] = round(time.monotonic() - t0, 1)
    emit(on_event, f"[host] inputs {host['inputs_count']} digest {host['inputs_digest'][:16]}, verifying")
    t1 = time.monotonic()
    report, raw = run_json([str(REPO_ROOT / "backend/harness/verify.sh"), "--c", str(driver), "--rust", str(binary),
                            "--inputs", str(inputs), "--timeout", "20", "--jobs", str(os.cpu_count() or 4),
                            "--out", str(jd / "verify_host.json")], timeout=3600)
    host["verify_seconds"] = round(time.monotonic() - t1, 1)
    if report is None:
        host["error"] = f"host verify printed no JSON: {raw[-1000:]}"
        return host
    host.update(match_rate=report.get("match_rate"), behaviors_tested=report.get("behaviors_tested"),
                counts=report.get("counts"))
    emit(on_event, f"[host] match_rate {host['match_rate']} in {host['verify_seconds']}s")
    return host


def run_verify_remote(library: str = "expat", rust_file: str = DEFAULT_RUST, seed: int = 7777, count: int = 2000, *,
                      model: str = "gemini-4-argon", host_binary: str | Path | None = None,
                      job_dir: str | Path | None = None, on_event: Event | None = None,
                      max_wait: float = 45 * 60, skip_preflight: bool = False) -> dict:
    """Sandbox claim, then host check, for an existing port. Never raises for sandbox failures."""
    if library not in ("expat", "libexpat"):
        raise ValueError("verify-only runs support library='expat' only")
    rust_path = (REPO_ROOT / rust_file).resolve()
    rust_rel = str(rust_path.relative_to(REPO_ROOT))
    jd = Path(job_dir) if job_dir else (
        REPO_ROOT / "runs/remote" / f"{datetime.now().strftime('%Y%m%d-%H%M%S')}-expat-verify")
    jd.mkdir(parents=True, exist_ok=True)
    timings: dict = {}
    res: dict = {"library": "expat", "mode": "verify_only", "model": model, "rust_file": rust_rel, "seed": seed,
                 "count": count, "job_dir": str(jd), "timings": timings, "notes": []}

    def save():
        (jd / "result.json").write_text(json.dumps(res, indent=2, default=str))

    if not skip_preflight:
        t = time.monotonic()
        try:
            res["preflight"] = run_preflight("expat", on_event=on_event)
        except PreflightError as exc:
            res["error"] = f"PREFLIGHT: {exc}"
            save()
            return res
        timings["preflight"] = round(time.monotonic() - t, 1)

    kwargs = build_verify_request(rust_rel, seed, count, model=model)
    (jd / "prompt.md").write_text(kwargs["system_instruction"] + "\n\n-----\n\n" + kwargs["input"])
    client = get_client()
    started = time.monotonic()
    try:
        interaction = client.interactions.create(**kwargs)
    except Exception as exc:
        res["notes"].append(f"create with mounted xmlts failed: {type(exc).__name__}: {str(exc)[:300]}; retrying without")
        kwargs = build_verify_request(rust_rel, seed, count, model=model, mount_xmlts=False)
        try:
            interaction = client.interactions.create(**kwargs)
        except Exception as exc2:
            res["error"] = f"create failed: {type(exc2).__name__}: {exc2}"
            save()
            return res
    res["xmlts_mounted"] = "sources" in kwargs["environment"]
    res["interaction_id"] = interaction.id
    res["created_at"] = datetime.now().isoformat()
    save()
    emit(on_event, f"[sandbox] interaction {interaction.id} {interaction.status}")

    # Host check runs while the sandbox works: it does not depend on the sandbox.
    host_box: dict = {}

    def host_job():
        t = time.monotonic()
        try:
            host_box.update(host_check(rust_path, Path(host_binary) if host_binary else None, seed, count, jd,
                                       on_event))
        except Exception as exc:
            host_box["error"] = f"{type(exc).__name__}: {exc}"
        host_box["seconds"] = round(time.monotonic() - t, 1)

    th = threading.Thread(target=host_job, daemon=True)
    th.start()

    interaction = poll(client, interaction, max_wait=max_wait, started=started, on_event=on_event, tag="[sandbox] ")
    timings["sandbox"] = round(time.monotonic() - started, 1)
    res["status"] = str(interaction.status)
    res["environment_id"] = getattr(interaction, "environment_id", None)
    res["usage"] = usage_dict(interaction)
    res["cost_usd"] = cost_usd(res["usage"])
    text = getattr(interaction, "output_text", None) or ""
    (jd / "agent_output.md").write_text(text)
    write_transcript(interaction, jd / "transcript.jsonl")
    res["notes"].append(f"steps: {step_types(interaction)}")
    if getattr(interaction, "errors", None):
        res["notes"].append(f"interaction errors: {interaction.errors}")
    res["sandbox_claim"] = parse_claim(text)
    if res["environment_id"]:
        res["sandbox_files"] = _pull_out(client, res["environment_id"], jd / "sandbox_out", res["notes"])
        fv = jd / "sandbox_out/verify.json"
        if fv.is_file():
            r = json.loads(fv.read_text())
            res["sandbox_verify_file"] = {"match_rate": r.get("match_rate"), "counts": r.get("counts"),
                                          "behaviors_tested": r.get("behaviors_tested")}
    save()

    th.join()
    res["host"] = host_box
    timings["host"] = host_box.get("seconds")
    res.update(compare(res["sandbox_claim"], host_box))
    save()
    emit(on_event, f"[done] sandbox {res['sandbox_claim'].get('match_rate')} host {host_box.get('match_rate')} "
                   f"agree {res['match_rates_agree']}")
    return res
