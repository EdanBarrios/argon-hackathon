"""POST /try with fake driver scripts (deterministic, no cargo, no ASan)."""

import base64
import os
import stat

import pytest
from fastapi.testclient import TestClient

from api import harness, try_input
from api.main import app


def script(path, body):
    path.write_text("#!/bin/sh\n" + body + "\n")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)
    return path


@pytest.fixture
def drivers(tmp_path, monkeypatch):
    c = script(tmp_path / "c_driver", 'input=$(cat); if [ "$input" = "[1e999]" ]; then printf "OK\\t[null]\\n"; else printf "PARSE_ERROR at 7\\n"; fi')
    rust = script(tmp_path / "rust_driver", 'input=$(cat); if [ "$input" = "[1e999]" ]; then printf "OK\\t[inf]\\n"; else printf "PARSE_ERROR at 7\\n"; fi')
    old = script(tmp_path / "old_driver", 'cat >/dev/null; echo "==1==ERROR: AddressSanitizer: heap-buffer-overflow" >&2; '
                 'echo "SUMMARY: AddressSanitizer: heap-buffer-overflow cJSON.c:786 in parse_string" >&2; exit 1')
    cves = tmp_path / "cve_inputs"
    cves.mkdir()
    (cves / "cve_2023_53154.json").write_bytes(b'{"1":1,')
    monkeypatch.setattr(harness, "C_DRIVER", c)
    monkeypatch.setattr(harness, "OLD_C_DRIVER", old)
    monkeypatch.setattr(harness, "OLD_CVE_INPUTS", cves)
    monkeypatch.setenv("THESEUS_RUST_BIN", str(rust))
    return tmp_path


@pytest.fixture
def client():
    return TestClient(app)


def test_try_divergence_shows_both_sides(client, drivers):
    r = client.post("/try", json={"library": "cjson", "input": "[1e999]"})
    assert r.status_code == 200, r.text
    body = r.json()
    assert body["verdict"] == "divergence"
    assert body["c"]["stdout"] == "OK\t[null]\n"
    assert body["rust"]["stdout"] == "OK\t[inf]\n"
    assert body["old_c"] is None and body["cve"] is None
    assert body["c"]["exit_code"] == 0 and body["c"]["sanitizer_summary"] is None


def test_try_known_cve_auto_compares_old(client, drivers):
    body = client.post("/try", json={"input": '{"1":1,'}).json()
    assert body["cve"] == "CVE-2023-53154"
    assert body["verdict"] == "match"
    assert body["old_verdict"] == "safety_improvement"
    assert body["old_c"]["crashed"] is True
    assert body["old_c"]["sanitizer_summary"].startswith("SUMMARY: AddressSanitizer: heap-buffer-overflow")


def test_try_compare_old_flag(client, drivers):
    body = client.post("/try", json={"input": "[1e999]", "compare_old": True}).json()
    assert body["old_c"] is not None
    assert body["old_verdict"] == "safety_improvement"


def test_try_b64_and_hex_view(client, drivers):
    raw = b"\xff\x00["
    body = client.post("/try", json={"input_b64": base64.b64encode(raw).decode()}).json()
    assert body["input_bytes"] == 3
    assert body["input_hex"] == "ff 00 5b"
    assert body["c"]["stdout_hex"] is None  # "PARSE_ERROR at 7\n" is printable


def test_try_rejects_bad_requests(client, drivers):
    assert client.post("/try", json={}).status_code == 400
    assert client.post("/try", json={"input": "a", "input_b64": "YQ=="}).status_code == 400
    assert client.post("/try", json={"input_b64": "!!!"}).status_code == 400
    assert client.post("/try", json={"library": "zlib", "input": "a"}).status_code == 400
    big = "x" * (try_input.MAX_INPUT_BYTES + 1)
    assert client.post("/try", json={"input": big}).status_code == 413


def test_try_rust_crash_classified(client, drivers, monkeypatch):
    crash = script(drivers / "panic", 'cat >/dev/null; echo "thread \'main\' panicked at src/cjson.rs:1:1" >&2; exit 101')
    monkeypatch.setenv("THESEUS_RUST_BIN", str(crash))
    body = client.post("/try", json={"input": "[1]"}).json()
    assert body["verdict"] == "rust_crash"
    assert "panicked at" in body["rust"]["sanitizer_summary"]


def test_try_missing_rust_binary_is_503(client, drivers, monkeypatch):
    monkeypatch.setenv("THESEUS_RUST_BIN", str(drivers / "nope"))
    assert client.post("/try", json={"input": "[1]"}).status_code == 503


def test_rust_binary_falls_back_to_committed_port(tmp_path, monkeypatch):
    """A fresh clone has no runs/ai/argon-1; the Try panel must use the committed port."""
    monkeypatch.delenv("THESEUS_RUST_BIN", raising=False)
    built = tmp_path / "committed"
    built.write_text("#!/bin/sh\n")
    built.chmod(0o755)
    monkeypatch.setattr(try_input, "_live_best", {})
    monkeypatch.setattr(harness, "candidate_binary", lambda name, build=True: None)
    monkeypatch.setattr(harness, "build_rust", lambda path, build_id: {"ok": True, "binary": str(built)})
    binary, label = try_input.rust_binary()
    assert binary == built and "committed" in label


@pytest.mark.skipif(
    not (os.access(harness.C_DRIVER, os.X_OK) and harness.candidate_binary("argon-1", build=False)),
    reason="real C driver or argon-1 build not present",
)
def test_try_real_binaries(client, monkeypatch):
    monkeypatch.delenv("THESEUS_RUST_BIN", raising=False)
    body = client.post("/try", json={"input": "[1e999]"}).json()
    assert body["verdict"] == "match"
    assert body["c"]["stdout"] == "OK\t[null]\n"
