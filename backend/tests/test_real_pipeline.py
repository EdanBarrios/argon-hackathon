"""RealPipeline with fakes for Argon, cargo and verify.sh (no network, no builds)."""

import time
from dataclasses import dataclass, field
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from api import cost, harness, pipeline
from api.main import app
from api.pipeline import RealPipeline


@dataclass
class FakeResult:
    rust_source: str
    interaction_id: str
    usage: dict = field(default_factory=lambda: {"total_input_tokens": 1000, "total_output_tokens": 200,
                                                 "total_thought_tokens": 300, "total_cached_tokens": 0})
    seconds: float = 0.01
    model: str = "gemini-4-argon"


def report(rate, divergences=(), crashes=()):
    n = 10
    return {
        "match_rate": rate,
        "behaviors_tested": n,
        "counts": {"match": round(rate * n), "divergence": len(divergences), "safety_improvement": 0,
                   "rust_crash": len(crashes), "both_crash": 0, "timeout": 0},
        "divergences": list(divergences),
        "crashes": list(crashes),
        "timeouts": [],
    }


DIV = {"input": "[1e999]", "file": "num_1", "c_output": "OK\t[null]", "rust_output": "OK\t[inf]",
       "c_exit": 0, "rust_exit": 0, "function": None}


@pytest.fixture
def fake(monkeypatch, tmp_path):
    monkeypatch.setenv("THESEUS_MODE", "real")
    calls = {"translate": 0, "repair": [], "verify": [], "builds": 0}
    reports = [report(0.9, [DIV]), report(1.0)]

    def translate_step(self, c_source, model, thinking):
        calls["translate"] += 1
        return FakeResult("// v1\n", "int_1", model=model)

    def repair_step(self, prev, feedback, model, thinking, c_source):
        calls["repair"].append((prev.interaction_id, feedback))
        return FakeResult("// v2\n", "int_2", model=model)

    def build_step(self, rust_path, build_id):
        calls["builds"] += 1
        return {"ok": True, "binary": str(tmp_path / f"bin{calls['builds']}"), "build_seconds": 0}

    def verify_step(self, binary, inputs_dir):
        calls["verify"].append(Path(inputs_dir).name)
        if Path(inputs_dir).name == "holdout":
            return report(1.0)
        return reports.pop(0) if reports else report(1.0)

    monkeypatch.setattr(RealPipeline, "translate_step", translate_step)
    monkeypatch.setattr(RealPipeline, "repair_step", repair_step)
    monkeypatch.setattr(RealPipeline, "build_step", build_step)
    monkeypatch.setattr(RealPipeline, "verify_step", verify_step)
    monkeypatch.setattr(RealPipeline, "inputs_step", lambda self, n: tmp_path)
    monkeypatch.setattr(RealPipeline, "holdout_step", lambda self, n, seen: (tmp_path / "holdout", 3))
    monkeypatch.setattr(RealPipeline, "c_source_step", lambda self: "/* cJSON */")
    monkeypatch.setattr(RealPipeline, "cve_step", lambda self, b: {
        "input": '{"1":1,', "cve": "CVE-2023-53154", "old_verdict": "safety_improvement",
        "old_c": {"stderr_head": "ERROR: AddressSanitizer: heap-buffer-overflow",
                  "sanitizer_summary": "SUMMARY: AddressSanitizer: heap-buffer-overflow", "exit": "exit 1"},
        "rust": {"stdout": "PARSE_ERROR at 7\n"}, "c": {"stdout": "PARSE_ERROR at 7\n"},
    })
    monkeypatch.setattr(harness, "RUNS_AI", tmp_path / "runs")
    return calls


@pytest.fixture
def client():
    return TestClient(app)


def wait(client, job_id, timeout=10):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        job = client.get(f"/jobs/{job_id}").json()
        if job["status"] in ("completed", "failed"):
            return job
        time.sleep(0.02)
    raise AssertionError("job did not finish")


def test_fresh_run_translates_repairs_and_completes(client, fake):
    r = client.post("/jobs", json={"library": "cjson", "rounds": 3, "inputs": 10})
    assert r.status_code == 200
    job = r.json()
    assert job["agents"][0]["module"] == "cjson parse+print"
    final = wait(client, job["id"])
    assert final["status"] == "completed", final["error"]
    res = final["result"]
    assert res["baseline_match_rate"] == 0.9
    assert res["final_match_rate"] == 1.0
    assert res["behaviors_tested"] == 10
    assert res["divergences"] == [{"input": "[1e999]", "c_output": "OK\t[null]", "rust_output": "OK\t[inf]",
                                   "function": "print_number", "resolved": True}]
    assert res["crashes"][0]["rust_behavior"] == "clean error: PARSE_ERROR at 7"
    assert final["agents"][0]["status"] == "completed"
    assert final["agents"][0]["match_rate"] == 1.0
    assert fake["translate"] == 1 and len(fake["repair"]) == 1
    assert fake["repair"][0][0] == "int_1"  # repair continues the translate interaction

    events = client.get(f"/jobs/{job['id']}/events").json()
    types = [e["type"] for e in events]
    for t in ("translate", "verify", "divergence", "repair", "crash", "approval_required"):
        assert t in types, t
    verifies = [e["data"] for e in events if e["type"] == "verify"]
    assert [v["match_rate"] for v in verifies] == [0.9, 1.0, 1.0]
    assert verifies[-1]["holdout"] is True
    done = [e for e in events if e["type"] == "job_status" and e["data"]["status"] == "completed"][0]
    assert {t["stage"] for t in done["data"]["timings"]} >= {"translate", "build", "verify", "repair"}
    assert client.get(f"/jobs/{job['id']}/result").status_code == 200
    usage = cost.job_cost(job["id"])
    assert usage["models"][0]["input_tokens"] == 2000


def test_start_from_candidate_skips_translate(client, fake, monkeypatch, tmp_path):
    rdir = tmp_path / "runs" / "flash-2" / "round_1"
    rdir.mkdir(parents=True)
    (rdir / "cjson.rs").write_text("// flash\n")
    (rdir / "meta.json").write_text('{"interaction_id": "flash_int", "match_rate": 1.0, "behaviors_tested": 300}')
    (tmp_path / "runs" / "flash-2" / "summary.json").write_text('{"model": "gemini-3.8-flash"}')
    job_id = client.post("/jobs", json={"start_from": "flash-2", "rounds": 2}).json()["id"]
    final = wait(client, job_id)
    assert final["status"] == "completed"
    assert fake["translate"] == 0
    assert fake["repair"][0][0] == "flash_int"
    events = client.get(f"/jobs/{job_id}/events").json()
    tr = [e["data"] for e in events if e["type"] == "translate"][0]
    assert tr["start_from"] == "flash-2"
    assert "Starting from Flash's first attempt" in tr["summary"]


def test_budget_exhausted_completes_with_best(client, fake, monkeypatch):
    monkeypatch.setattr(RealPipeline, "verify_step", lambda self, b, d: report(0.9, [DIV]))
    job_id = client.post("/jobs", json={"rounds": 1, "holdout": False, "cve_check": False}).json()["id"]
    final = wait(client, job_id)
    assert final["status"] == "completed"
    assert final["result"]["final_match_rate"] == 0.9
    assert final["result"]["divergences"][0]["resolved"] is False
    assert final["result"]["crashes"] == []


def test_compile_errors_feed_repair(client, fake, monkeypatch, tmp_path):
    builds = iter([{"ok": False, "errors": "error[E0308]: mismatched types"},
                   {"ok": True, "binary": str(tmp_path / "b")}])
    monkeypatch.setattr(RealPipeline, "build_step", lambda self, p, i: next(builds))
    monkeypatch.setattr(RealPipeline, "verify_step", lambda self, b, d: report(1.0))
    job_id = client.post("/jobs", json={"rounds": 2}).json()["id"]
    final = wait(client, job_id)
    assert final["status"] == "completed"
    assert fake["repair"][0][1]["compile_errors"].startswith("error[E0308]")
    events = client.get(f"/jobs/{job_id}/events").json()
    assert any(e["type"] == "verify" and e["data"].get("compile_ok") is False for e in events)


def test_translate_failure_fails_job(client, fake, monkeypatch):
    def boom(self, *a):
        raise RuntimeError("quota")

    monkeypatch.setattr(RealPipeline, "translate_step", boom)
    job_id = client.post("/jobs", json={}).json()["id"]
    final = wait(client, job_id)
    assert final["status"] == "failed"
    assert final["error"]["code"] == "translate_failed"


def test_guess_function():
    assert pipeline.guess_function("OK/OK print differs in string: C next=quote Rust next=alpha") == "print_string_ptr"
    assert pipeline.guess_function("C OK vs Rust ERR_AT, input starts with [") == "parse_array"


def test_approve_without_token_is_an_honest_dry_run(client, fake, monkeypatch):
    from harness.remote import open_pr as op

    def no_agent(*a, **k):
        raise AssertionError("dry run must not start a managed agent")

    monkeypatch.delenv("GITHUB_TOKEN", raising=False)
    monkeypatch.setattr(op, "env_value", lambda name: None)  # ignore the repo .env
    monkeypatch.setattr(op, "_cve_check", lambda binary: {})
    monkeypatch.setattr(op, "open_pr_remote", no_agent)
    job = client.post("/jobs", json={"library": "cjson", "rounds": 3, "inputs": 10}).json()
    assert wait(client, job["id"])["status"] == "completed"

    r = client.post(f"/jobs/{job['id']}/approve", json={"approved_by": "edan"})
    assert r.status_code == 200, r.text
    approval = r.json()["approval"]
    assert approval["approved_by"] == "edan"
    assert approval["pr_url"] is None  # never a fake URL
    assert approval["pr_status"] == "dry_run"
    assert approval["pr_branch"].startswith("theseus/live-")
    assert "GITHUB_TOKEN is not set" in approval["pr_detail"]
    events = client.get(f"/jobs/{job['id']}/events").json()
    assert "migrated/cjson/EVIDENCE.md" in events[-1]["data"]["pr_files"]
    # the payload holds the round that reached 1.0 (round 2, "// v2")
    payload = next(harness.RUNS_AI.glob(f"live-*-{job['id']}/release/pr-dry-run-local/crate"))
    assert (payload / "src/cjson.rs").read_text() == "// v2\n"
    assert "Approved by **edan**" in (payload / "EVIDENCE.md").read_text()


def test_extra_inputs_merge_corpus(monkeypatch, tmp_path):
    corpus = tmp_path / "corpus" / "targeted"
    corpus.mkdir(parents=True)
    for name in ("str_a.bin", "str_B.bin", "num_1.bin", "README.md", ".hidden"):
        (corpus / name).write_text(name)
    base = tmp_path / "runs" / "inputs-c2-s1"
    base.mkdir(parents=True)
    (base / "x").write_text("1")
    (base / "y").write_text("2")
    monkeypatch.setattr(harness, "ADVERSARIAL", tmp_path / "corpus")
    monkeypatch.setattr(harness, "RUNS_AI", tmp_path / "runs")
    out = harness.merged_inputs(base, ["targeted:str_"])
    assert sorted(p.name for p in out.iterdir()) == ["adv-targeted-str_B.bin", "adv-targeted-str_a.bin", "x", "y"]
    with pytest.raises(harness.HarnessError):
        harness.corpus_files("nope")


def test_extra_inputs_reach_verify(client, fake, monkeypatch, tmp_path):
    seen = []
    monkeypatch.setattr(RealPipeline, "extra_inputs_step",
                        lambda self, base, specs: seen.append(specs) or tmp_path / "merged")
    monkeypatch.setattr(harness, "corpus_files", lambda spec: [])
    job = client.post("/jobs", json={"rounds": 3, "inputs": 10, "extra_inputs": ["targeted:str_"]}).json()
    assert wait(client, job["id"])["status"] == "completed"
    assert seen == [["targeted:str_"]]
    assert fake["verify"][0] == "merged"


def test_unknown_extra_inputs_is_400(client, fake):
    assert client.post("/jobs", json={"extra_inputs": ["../etc"]}).status_code == 400
