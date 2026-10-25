import json
import time
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from api import cost
from api.main import app

FIXTURES = Path(__file__).resolve().parents[2] / "contracts" / "fixtures"
MOCK_DURATION = 0.3  # seconds for the whole replay, instead of ~30


def fixture(name: str):
    return json.loads((FIXTURES / name).read_text())


@pytest.fixture(autouse=True)
def fast_mock(monkeypatch):
    monkeypatch.setenv("THESEUS_MODE", "mock")
    monkeypatch.setenv("THESEUS_MOCK_DURATION", str(MOCK_DURATION))


@pytest.fixture
def client():
    return TestClient(app)


def wait_for(client, job_id, statuses=("completed", "failed"), timeout=MOCK_DURATION + 5):
    deadline = time.monotonic() + timeout
    seen = set()
    while time.monotonic() < deadline:
        job = client.get(f"/jobs/{job_id}").json()
        seen.add(job["status"])
        if job["status"] in statuses:
            return job, seen
        time.sleep(0.02)
    raise AssertionError(f"job {job_id} stuck in {job['status']}")


def strip_volatile(job):
    return {k: v for k, v in job.items() if k not in ("id", "created_at", "updated_at")}


def test_health(client):
    r = client.get("/health")
    assert r.status_code == 200
    assert r.json() == {"status": "ok"}


def test_create_and_poll_until_completed(client):
    r = client.post("/jobs", json={"library": "cjson", "modules": ["lexer"]})
    assert r.status_code == 200
    job = r.json()
    assert job["status"] == "queued"
    assert job["id"].startswith("job_")
    assert set(job) == set(fixture("job-completed.json"))

    final, seen = wait_for(client, job["id"])
    assert {"running", "verifying"} <= seen
    expected = fixture("job-completed.json")
    expected["id"] = job["id"]
    assert strip_volatile(final) == strip_volatile(expected)

    events = client.get(f"/jobs/{job['id']}/events").json()
    fixture_events = fixture("job-events.json")
    assert [e["type"] for e in events] == [e["type"] for e in fixture_events]
    assert [e["seq"] for e in events] == list(range(1, len(fixture_events) + 1))
    assert all(e["job_id"] == job["id"] for e in events)
    assert set(events[0]) == set(fixture_events[0])
    assert events[-1]["data"]["evidence"] == f"/jobs/{job['id']}/result"


def test_agent_lanes_move_during_replay(client, monkeypatch):
    monkeypatch.setenv("THESEUS_MOCK_DURATION", "1.0")
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    printer_rates = set()
    deadline = time.monotonic() + 7
    while time.monotonic() < deadline:
        job = client.get(f"/jobs/{job_id}").json()
        printer = next(a for a in job["agents"] if a["id"] == "agent_printer")
        printer_rates.add(printer["match_rate"])
        if job["status"] == "completed":
            break
        time.sleep(0.01)
    assert 0.64 in printer_rates
    assert 1.0 in printer_rates


def test_events_after(client):
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    wait_for(client, job_id)
    all_events = client.get(f"/jobs/{job_id}/events").json()
    after = client.get(f"/jobs/{job_id}/events", params={"after": 10}).json()
    assert [e["seq"] for e in after] == [e["seq"] for e in all_events if e["seq"] > 10]
    assert after == all_events[10:]
    assert client.get(f"/jobs/{job_id}/events", params={"after": 999}).json() == []


def test_result_shape(client):
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    early = client.get(f"/jobs/{job_id}/result")
    assert early.status_code == 409
    wait_for(client, job_id)
    result_job = client.get(f"/jobs/{job_id}/result").json()
    expected = fixture("job-completed.json")
    assert set(result_job) == set(expected)
    assert set(result_job["result"]) == set(expected["result"])
    assert set(result_job["result"]["divergences"][0]) == set(expected["result"]["divergences"][0])
    assert set(result_job["result"]["crashes"][0]) == set(expected["result"]["crashes"][0])
    assert set(result_job["approval"]) == set(expected["approval"])
    assert set(result_job["agents"][0]) == set(expected["agents"][0])


def test_failed_scenario(client):
    job_id = client.post("/jobs", params={"scenario": "failed"}, json={"library": "cjson"}).json()["id"]
    final, _ = wait_for(client, job_id)
    expected = fixture("job-failed.json")
    expected["id"] = job_id
    assert strip_volatile(final) == strip_volatile(expected)
    events = client.get(f"/jobs/{job_id}/events").json()
    assert events[-1]["type"] == "job_status"
    assert events[-1]["data"]["status"] == "failed"
    assert client.get(f"/jobs/{job_id}/result").status_code == 409


def test_approve_happy_path(client):
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    wait_for(client, job_id)
    r = client.post(f"/jobs/{job_id}/approve", json={"approved_by": "edan"})
    assert r.status_code == 200
    approval = r.json()["approval"]
    assert approval["approved_by"] == "edan"
    assert approval["pr_url"].startswith("https://")
    events = client.get(f"/jobs/{job_id}/events").json()
    assert events[-1]["type"] == "approved"
    # Second approval is a conflict.
    assert client.post(f"/jobs/{job_id}/approve", json={"approved_by": "edan"}).status_code == 409


def test_approve_conflict_when_not_completed(client, monkeypatch):
    monkeypatch.setenv("THESEUS_MOCK_DURATION", "30")
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    r = client.post(f"/jobs/{job_id}/approve", json={"approved_by": "edan"})
    assert r.status_code == 409


def test_approve_conflict_when_failed(client):
    job_id = client.post("/jobs", params={"scenario": "failed"}, json={"library": "cjson"}).json()["id"]
    wait_for(client, job_id)
    assert client.post(f"/jobs/{job_id}/approve", json={"approved_by": "edan"}).status_code == 409


def test_404(client):
    for path in ("/jobs/nope", "/jobs/nope/events", "/jobs/nope/result", "/jobs/nope/cost"):
        assert client.get(path).status_code == 404, path
    assert client.post("/jobs/nope/approve", json={"approved_by": "edan"}).status_code == 404


def test_real_mode_fails_job_cleanly(client, monkeypatch):
    monkeypatch.setenv("THESEUS_MODE", "real")
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    final, _ = wait_for(client, job_id)
    assert final["status"] == "failed"
    assert final["error"]["code"] == "not_implemented"


def test_cost_math():
    job_id = "job_costtest"
    cost.record_usage(job_id, "gemini-4-argon", {
        "total_input_tokens": 1_000_000,
        "total_cached_tokens": 400_000,
        "total_output_tokens": 100_000,
        "total_thought_tokens": 50_000,
    })

    class Usage:  # SDK-object style, with a None field
        total_input_tokens = 500_000
        total_cached_tokens = None
        total_output_tokens = 20_000
        total_thought_tokens = 0

    cost.record_usage(job_id, "gemini-3.8-flash", Usage())
    out = cost.job_cost(job_id)
    by_model = {m["model"]: m for m in out["models"]}

    # 600k uncached * $2/M = 1.20; 400k cached * $0.10/M = 0.04; 150k out+thought * $10/M = 1.50
    assert by_model["gemini-4-argon"]["usd"] == pytest.approx(2.74)
    assert by_model["gemini-4-argon"]["thought_tokens"] == 50_000
    # 500k * $2/M = 1.00; 20k * $10/M = 0.20
    assert by_model["gemini-3.8-flash"]["usd"] == pytest.approx(1.20)
    assert by_model["gemini-3.8-flash"]["cached_tokens"] == 0
    assert out["total_usd"] == pytest.approx(3.94)


def test_cost_endpoint_in_mock(client):
    job_id = client.post("/jobs", json={"library": "cjson"}).json()["id"]
    wait_for(client, job_id)
    out = client.get(f"/jobs/{job_id}/cost").json()
    assert out["job_id"] == job_id
    assert out["total_usd"] > 0
    assert set(out["models"][0]) == {
        "model", "input_tokens", "cached_tokens", "output_tokens", "thought_tokens", "usd",
    }
