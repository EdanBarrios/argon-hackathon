"""Offline tests for the verify-only remote job (no network, no model call).

    cd backend && uv run python -m pytest harness/remote/test_verify_remote.py -q
"""

import subprocess
import sys

from harness.remote import verify_remote as vr


def test_request_is_verify_only_and_allows_w3c():
    k = vr.build_verify_request(vr.DEFAULT_RUST, 7777, 2000)
    text = k["input"]
    assert "--seed 7777 --count 2000" in text
    assert "--timeout 20" in text
    assert "scripts/build_rust_expat.sh " + vr.DEFAULT_RUST in text
    assert "must NOT write, translate, repair" in text
    hosts = {d["domain"] for d in k["environment"]["network"]["allowlist"]}
    assert {"github.com", "www.w3.org", "sh.rustup.rs"} <= hosts
    assert "GITHUB_TOKEN" not in text and "API_KEY" not in text


def test_digest_one_liner_matches_host(tmp_path):
    for name, data in [("b.xml", b"<b/>"), ("a.xml", b"\x00\xff"), ("c", b"")]:
        (tmp_path / name).write_bytes(data)
    want = vr.inputs_digest(tmp_path)
    out = subprocess.run([sys.executable, "-I", "-c", vr.DIGEST_PY, str(tmp_path)], capture_output=True,
                         text=True, check=True).stdout.split()
    assert (out[0], int(out[1])) == want


def test_parse_claim_and_compare():
    text = ("XMLTS_SOURCE w3.org\nCOMMIT abcdef1\nSHA256 " + "a" * 64 + "\nINPUTS_DIGEST " + "b" * 64 + " 2000\n"
            + vr.BEGIN_VERIFY + '\n{"match_rate":1.0,"behaviors_tested":2000,"counts":{"match":2000}}\n'
            + vr.END_VERIFY)
    c = vr.parse_claim(text)
    assert c["match_rate"] == 1.0 and c["inputs_count"] == 2000 and c["xmlts_source"] == "w3.org"
    host = {"match_rate": 1.0, "inputs_digest": "b" * 64, "expat_rs_sha256": "a" * 64, "counts": {"match": 2000}}
    assert vr.compare(c, host) == {"match_rates_agree": True, "inputs_digest_agree": True,
                                   "expat_rs_sha256_agree": True, "counts_agree": True}
    assert vr.compare(c, dict(host, match_rate=0.99))["match_rates_agree"] is False
    assert vr.compare(vr.parse_claim("no block"), host)["match_rates_agree"] is None
