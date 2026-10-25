"""Migration certificate: the facts a PR carries, machine-readable, plus a reproduce script.

    build_certificate(jd, result, approved_by, branch) -> dict   (certificate.json)
    evidence_sections(cert) -> list[str]                          (Markdown lines for EVIDENCE.md)
    reproduce_script(cert) -> str                                 (reproduce.sh)

Every value comes from files a real run wrote (result.json, spec.json, the
verify.sh reports, preflight.json) or from git. A value that is not on disk is
null, never estimated. Language rule: "agreement on N tested inputs", never
"proves" or "equivalent".
"""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from datetime import datetime
from pathlib import Path

from ._common import AGENT, HARNESS, REPO_ROOT

SCOPE = "cJSON v1.7.19 parse+print through our driver"
CJSON_TAG = "v1.7.19"
SUMMARY_KEYS = ("attempted", "scored", "matched", "skipped", "safety_improvements", "infra_failures")
LIMITATIONS = [
    "Covers parse+print only (cJSON_ParseWithLengthOpts and cJSON_PrintUnformatted through our driver), "
    "not the full cJSON API or its C ABI.",
    "Output after an embedded NUL byte is not compared: the C driver prints with fputs, so both sides are "
    "compared only up to the first NUL.",
    "The banned-API guard (no std::process, std::fs, std::net, Command, include!, ...) is a text scan of "
    "cjson.rs, not a sandbox.",
    "Agreement on the tested inputs is not a proof of equivalence: inputs nobody generated can still diverge.",
    "Experimental: produced by a model and checked by our differential verifier; it needs maintainer review "
    "before anyone relies on it.",
]


def _read_json(path: Path) -> dict | None:
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return None


def _git(*args: str) -> str | None:
    try:
        p = subprocess.run(["git", "-C", str(REPO_ROOT), *args], capture_output=True, text=True, timeout=20)
        return p.stdout.strip() if p.returncode == 0 else None
    except (OSError, subprocess.TimeoutExpired):
        return None


def inputs_digest(directory: Path) -> str | None:
    """sha256 over sorted (file name, sha256 of bytes): the same rule reproduce.sh uses."""
    if not directory.is_dir():
        return None
    h = hashlib.sha256()
    for p in sorted(directory.iterdir(), key=lambda q: q.name):
        if p.is_file() and not p.name.startswith("."):
            h.update(p.name.encode() + b"\0" + hashlib.sha256(p.read_bytes()).hexdigest().encode() + b"\n")
    return h.hexdigest()


def _pinned_cjson_commit() -> str | None:
    m = re.search(r'CJSON_COMMIT="([0-9a-f]{40})"', (HARNESS / "fetch_cjson.sh").read_text())
    return m.group(1) if m else None


def _report(paths: list[Path]) -> tuple[dict | None, str | None]:
    for p in paths:
        r = _read_json(p)
        if r and "match_rate" in r:
            return r, str(p.relative_to(REPO_ROOT)) if p.is_relative_to(REPO_ROOT) else str(p)
    return None, None


def _run_block(report: dict | None, fallback: dict | None, path: str | None) -> dict:
    """match rate plus the verify report's own `summary` counts (null when the report has none)."""
    fb = fallback or {}
    s = (report or {}).get("summary") or fb.get("summary")
    return {
        "match_rate": (report or {}).get("match_rate", fb.get("match_rate")),
        **{k: (s or {}).get(k) for k in SUMMARY_KEYS},
        "summary_source": path if s else "not available (report has no summary object)",
        "timeout_seconds": ((report or {}).get("config") or {}).get("timeout"),
    }


def build_certificate(jd: Path, result: dict, approved_by: str, branch: str) -> dict:
    spec = _read_json(jd / "spec.json") or {}
    rust_src = Path(result["rust_source"])
    local_report, local_path = _report([jd / "verify_local.json", rust_src.parent / "verify.json"])
    held_report, held_path = _report([jd / "verify_holdout.json"])
    held_fb = result.get("holdout_verify") or {}
    preflight = _read_json(jd / "preflight-pr.json") or _read_json(jd / "preflight.json")
    prov = (local_report or {}).get("provenance") or {}
    seed = spec.get("seed")
    count = spec.get("inputs")

    held = _run_block(held_report, held_fb, held_path)
    held.update(seed=held_fb.get("seed"), overlap_removed=held_fb.get("overlap_removed"),
                rule="generated with seed + 1000, minus any input whose bytes the repair loop saw")
    local = _run_block(local_report, result.get("local_verify"), local_path)
    local.update(seed=seed, generated_count=count, inputs_sha256=inputs_digest(jd / "inputs"))
    held["inputs_sha256"] = inputs_digest(jd / "holdout_inputs")

    return {
        "certificate_version": 1,
        "scope": SCOPE,
        "claim": (f"agreement on {local.get('scored')} tested inputs (seed {seed}) and {held.get('scored')} "
                  f"held-out inputs; not a proof of equivalence") if local.get("scored") is not None else None,
        "source": {
            "repo": "EdanBarrios/argon-hackathon",
            "harness_commit": _git("rev-parse", "HEAD"),
            "harness_dirty": bool(_git("status", "--porcelain", "--", "backend/harness", "scripts")),
            "c_library": "cJSON",
            "c_tag": CJSON_TAG,
            "c_commit": prov.get("cjson_commit") or _pinned_cjson_commit(),
        },
        "model": {
            "model": result.get("model"),
            "agent": AGENT if result.get("interaction_id") else None,
            "interaction_id": result.get("interaction_id"),
            "environment_id": result.get("environment_id"),
            "rounds_used": result.get("rounds_used"),
            "seconds": result.get("seconds"),
            "usage": result.get("usage"),
            "cost_usd_estimate": result.get("cost_usd"),
        },
        "rust": {
            "cjson_rs_sha256": hashlib.sha256(rust_src.read_bytes()).hexdigest(),
            "lines": len(rust_src.read_text(errors="replace").splitlines()),
            "unsafe_keyword_present": bool(re.search(r"\bunsafe\b", rust_src.read_text(errors="replace"))),
        },
        "verification": {
            "same_seed": local,
            "held_out": held,
            "sandbox_claimed_match_rate": result.get("sandbox_match_rate"),
            "host_reverified_match_rate": result.get("local_match_rate"),
            "claim_matches_host": result.get("match_rates_agree"),
            "verifier": "backend/harness/verify.sh (raw stdout bytes and exit codes compared, C side under ASan+UBSan)",
        },
        "preflight": None if not preflight else {
            "ok": preflight.get("ok"),
            "checks": {k: {kk: vv for kk, vv in v.items() if kk in ("ok", "match_rate", "scored", "kill_rate", "skipped")}
                       for k, v in (preflight.get("checks") or {}).items()},
            "seconds": preflight.get("seconds"),
            "work_dir": preflight.get("work_dir"),
        },
        "known_limitations": LIMITATIONS,
        "approval": {"approved_by": approved_by, "at": datetime.now().isoformat(timespec="seconds"),
                     "branch": branch},
        "status": "experimental; needs maintainer review",
        "reproduce": "bash migrated/cjson/reproduce.sh (from the repo root of a fresh clone)",
    }


def _rate(x) -> str:
    return "n/a" if x is None else f"{x:.4f}"


def evidence_sections(cert: dict) -> tuple[list[str], list[str]]:
    """(head, tail) Markdown lines: scope and certificate up top, limitations and reproduce at the end."""
    v, src, m = cert["verification"], cert["source"], cert["model"]
    s, h = v["same_seed"], v["held_out"]
    pf = cert.get("preflight")
    if pf:
        pf_line = "passed" if pf["ok"] else "FAILED"
        pf_line += " (" + ", ".join(f"{k}: {'skipped' if c.get('skipped') else ('ok' if c.get('ok') else 'FAILED')}"
                                    for k, c in pf["checks"].items()) + f"; {pf.get('seconds')} s)"
    else:
        pf_line = "not recorded for this job"

    def counts(b: dict) -> str:
        if b.get("scored") is None:
            return f"counts: {b['summary_source']}"
        return (f"attempted {b['attempted']}, scored {b['scored']}, matched {b['matched']}, skipped {b['skipped']}, "
                f"safety_improvements {b['safety_improvements']}, infra_failures {b['infra_failures']}")

    head = [
        f"**Scope:** {cert['scope']}. **Status:** {cert['status']}.",
        "",
        f"**Claim:** {cert['claim'] or 'no verify summary available'}.",
        "",
        "## Certificate",
        "",
        f"- Source: `{src['repo']}` harness commit `{src['harness_commit']}`"
        f"{' (uncommitted harness changes at certificate time)' if src['harness_dirty'] else ''}; "
        f"reference C: cJSON `{src['c_tag']}` at `{src['c_commit']}`",
        f"- Model: `{m['model']}`; interaction `{m['interaction_id']}`; environment `{m['environment_id']}`",
        f"- Same-seed inputs (seed {s.get('seed')}, {s.get('generated_count')} generated): match rate "
        f"{_rate(s['match_rate'])}; {counts(s)}",
        f"- Held-out inputs (seed {h.get('seed')}, {h.get('overlap_removed')} overlaps removed): match rate "
        f"{_rate(h['match_rate'])}; {counts(h)}",
        f"- Sandbox claimed match rate {_rate(v['sandbox_claimed_match_rate'])}; host re-verified "
        f"{_rate(v['host_reverified_match_rate'])}; agree: {v['claim_matches_host']}",
        f"- Verifier preflight (canary, C vs C, reject-everything stub): {pf_line}",
        "- Machine-readable copy: `certificate.json`",
        "",
    ]
    tail = [
        "## Known limitations",
        "",
        *[f"- {x}" for x in cert["known_limitations"]],
        "",
        "## Reproduce",
        "",
    ]
    if not cert.get("reproduce"):
        return head, tail + [f"No reproduce.sh for this job: {cert.get('reproduce_unavailable')}", ""]
    tail += [
        "From the repo root of a fresh clone (needs clang with ASan, cargo, python3, git):",
        "",
        "```bash",
        "bash migrated/cjson/reproduce.sh",
        "```",
        "",
        "It fetches and builds the pinned C, builds this crate through `scripts/build_rust.sh` (with the",
        "banned-API guard), regenerates the same inputs from the same seeds, runs `verify.sh`, and prints the",
        "match rates next to the ones recorded in `certificate.json`.",
        "",
    ]
    return head, tail


REPRODUCE = r'''#!/bin/bash
# Reproduce the Theseus migration certificate for this crate.
# Run from anywhere inside a fresh clone:  bash migrated/cjson/reproduce.sh
# Needs: git, clang (ASan/UBSan), cargo, python3. Bash 3.2 compatible.
# Fetches cJSON @C_TAG@ (@C_COMMIT@), builds the ASan C driver, builds
# src/cjson.rs with scripts/build_rust.sh (safe-Rust and banned-API guard),
# regenerates the same inputs (seed @SEED@, @COUNT@ inputs; held-out seed
# @HSEED@ minus overlaps), runs verify.sh, and compares with certificate.json.
# Exit 0 only if every match rate equals the recorded one.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd -P)
ROOT=$(cd "$HERE/../.." && pwd -P)
H="$ROOT/backend/harness"
SEED=@SEED@
COUNT=@COUNT@
HSEED=@HSEED@
TIMEOUT=@TIMEOUT@
WORK="$ROOT/runs/reproduce-$(date +%Y%m%d-%H%M%S)-$$"
mkdir -p "$WORK"
echo "reproduce: work dir $WORK" >&2

"$H/fetch_cjson.sh" >&2
"$H/build_cjson.sh" "$WORK/cjson_driver" >&2
BUILD=$("$ROOT/scripts/build_rust.sh" "$HERE/src/cjson.rs" "reproduce-$(basename "$WORK")")
RUST=$(python3 -I -c 'import json,sys; d=json.loads(sys.argv[1]); print(d["binary"]) if d["ok"] else sys.exit("rust build failed: "+d["errors"])' "$BUILD")

python3 -I "$H/gen_inputs.py" --seed "$SEED" --count "$COUNT" --out "$WORK/inputs" --cjson "$H/vendor/cJSON" >&2
python3 -I "$H/gen_inputs.py" --seed "$HSEED" --count "$COUNT" --out "$WORK/holdout_raw" --cjson "$H/vendor/cJSON" >&2
python3 -I - "$WORK" <<'PY'
import hashlib, os, shutil, sys
w = sys.argv[1]
seen = {hashlib.sha256(open(os.path.join(w, "inputs", n), "rb").read()).digest() for n in os.listdir(os.path.join(w, "inputs"))}
os.makedirs(os.path.join(w, "holdout_inputs"))
for n in sorted(os.listdir(os.path.join(w, "holdout_raw"))):
    data = open(os.path.join(w, "holdout_raw", n), "rb").read()
    if hashlib.sha256(data).digest() not in seen:
        open(os.path.join(w, "holdout_inputs", n), "wb").write(data)
shutil.rmtree(os.path.join(w, "holdout_raw"))
PY

"$H/verify.sh" --c "$WORK/cjson_driver" --rust "$RUST" --inputs "$WORK/inputs" --timeout "$TIMEOUT" --out "$WORK/verify_same_seed.json" >/dev/null
"$H/verify.sh" --c "$WORK/cjson_driver" --rust "$RUST" --inputs "$WORK/holdout_inputs" --timeout "$TIMEOUT" --out "$WORK/verify_held_out.json" >/dev/null

python3 -I - "$HERE/certificate.json" "$WORK" "$HERE/src/cjson.rs" <<'PY'
import hashlib, json, os, sys
cert, work, rs = json.load(open(sys.argv[1])), sys.argv[2], sys.argv[3]

def digest(d):
    h = hashlib.sha256()
    for n in sorted(os.listdir(d)):
        p = os.path.join(d, n)
        if os.path.isfile(p) and not n.startswith("."):
            h.update(n.encode() + b"\0" + hashlib.sha256(open(p, "rb").read()).hexdigest().encode() + b"\n")
    return h.hexdigest()

ok = True
sha = hashlib.sha256(open(rs, "rb").read()).hexdigest()
print("cjson.rs sha256 %s (certificate %s)" % (sha, "same" if sha == cert["rust"]["cjson_rs_sha256"] else "DIFFERENT"))
for key, rep, inp in (("same_seed", "verify_same_seed.json", "inputs"), ("held_out", "verify_held_out.json", "holdout_inputs")):
    r = json.load(open(os.path.join(work, rep)))
    s, want = r["summary"], cert["verification"][key]
    same_inputs = want.get("inputs_sha256") in (None, digest(os.path.join(work, inp)))
    good = r["match_rate"] == want["match_rate"]
    ok = ok and good
    print("%-9s match_rate %s (certificate %s) %s; scored %d, matched %d, skipped %d, infra_failures %d; inputs %s"
          % (key, r["match_rate"], want["match_rate"], "OK" if good else "DIFFERENT", s["scored"], s["matched"],
             s["skipped"], s["infra_failures"],
             "unknown" if want.get("inputs_sha256") is None else ("identical" if same_inputs else "DIFFERENT")))
print("reproduced" if ok else "NOT reproduced", "| reports in", work)
sys.exit(0 if ok else 1)
PY
'''


def reproduce_script(cert: dict) -> str:
    s, h = cert["verification"]["same_seed"], cert["verification"]["held_out"]
    seed, count = s.get("seed"), s.get("generated_count")
    if seed is None or count is None:
        raise ValueError("spec.json has no seed/inputs: cannot write a reproduce script with the same inputs")
    hseed = h.get("seed") if h.get("seed") is not None else seed + 1000
    timeout = s.get("timeout_seconds") or 20
    out = REPRODUCE
    for k, val in (("@SEED@", seed), ("@COUNT@", count), ("@HSEED@", hseed), ("@TIMEOUT@", timeout),
                   ("@C_TAG@", cert["source"]["c_tag"]), ("@C_COMMIT@", cert["source"]["c_commit"])):
        out = out.replace(k, str(val))
    return out
