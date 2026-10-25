"""POST /try with library=expat, GET /try/presets, GET /results/libexpat.

The live tests run the real libexpat 2.9.0 ASan driver and Argon's Rust port; they
skip when either is missing (no build attempted inside the test run).
"""

import os

import pytest
from fastapi.testclient import TestClient

from api import harness
from api.main import app


@pytest.fixture
def client():
    return TestClient(app)


def _have_real_expat() -> bool:
    rust = os.environ.get("THESEUS_EXPAT_RUST_BIN") or str(harness.EXPAT_RUST_BIN)
    return os.access(harness.EXPAT_C_DRIVER, os.X_OK) and os.access(rust, os.X_OK)


real_expat = pytest.mark.skipif(
    not _have_real_expat(),
    reason="expat C driver (backend/harness/expat/build_expat.sh) or Argon's Rust build "
           "(scripts/build_rust_expat.sh) not present",
)


def _preset(client, pid):
    presets = client.get("/try/presets", params={"library": "expat"}).json()
    return next(p["input"] for p in presets if p["id"] == pid)


def test_presets_shape(client):
    r = client.get("/try/presets?library=expat")
    assert r.status_code == 200
    ids = [p["id"] for p in r.json()]
    assert ids == ["rich", "billion_laughs"]
    assert all({"id", "title", "description", "input"} <= set(p) for p in r.json())
    assert client.get("/try/presets?library=cjson").json() == []
    assert client.get("/try/presets?library=zlib").status_code == 400


def test_expat_rejects_compare_old(client):
    assert client.post("/try", json={"library": "expat", "input": "<a/>", "compare_old": True}).status_code == 400


@real_expat
def test_expat_rich_document_matches(client):
    r = client.post("/try", json={"library": "expat", "input": _preset(client, "rich")})
    assert r.status_code == 200, r.text
    body = r.json()
    assert body["library"] == "expat"
    assert body["verdict"] == "match"
    assert body["first_difference"] is None
    assert body["c"]["stdout"] == body["rust"]["stdout"]
    out = body["c"]["stdout"]
    for line in ('XMLDECL "1.0" "UTF-8" -1', 'ENTITY "org" 0 VALUE', "COMMENT", 'PI "render"',
                 "CDATA_START", 'TEXT "Ship of Theseus & Argon"', '"lang"="en"'):
        assert line in out
    assert out.endswith("OK\n")
    assert body["c"]["sanitizer_summary"] is None


@real_expat
def test_expat_billion_laughs_error_43(client):
    body = client.post("/try", json={"library": "expat", "input": _preset(client, "billion_laughs")}).json()
    assert body["verdict"] == "match"
    assert body["c"]["stdout"].startswith("ERROR 43 line ")
    assert body["c"]["stdout"] == body["rust"]["stdout"]
    assert not body["c"]["timed_out"] and not body["rust"]["timed_out"]


@real_expat
def test_expat_small_sample_and_error_position(client):
    body = client.post("/try", json={"library": "expat", "input": '<a x="1">t<b/></a>'}).json()
    assert body["verdict"] == "match"
    assert body["c"]["stdout"] == 'START "a" "x"="1"\nTEXT "t"\nSTART "b"\nEND "b"\nEND "a"\nOK\n'
    body = client.post("/try", json={"library": "expat", "input": "<a>\n<b></a>"}).json()
    assert body["verdict"] == "match"
    assert body["rust"]["stdout"] == "ERROR 7 line 2 col 5\n"


def test_results_libexpat_fields(client):
    r = client.get("/results/libexpat")
    assert r.status_code == 200, r.text
    body = r.json()
    for key in ("c_lines", "c_lines_source", "rust_lines", "translate_seconds", "compile_fix_rounds",
                "lines_changed_after_translate", "loop", "holdout", "host_reverify", "flash_result", "sources"):
        assert key in body
    assert body["host_reverify"]["matched"] == body["host_reverify"]["tested"] == 2000
    assert body["holdout"]["tested"] == 1253 and body["loop"]["tested"] == 747
    assert body["compile_fix_rounds"] == 2
    assert body["flash_result"]["file"] is None and body["flash_result"]["reason"]
    assert "815566" not in str(body)
