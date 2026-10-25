"""Offline tests for ai.quality parsing and the loop's host fix and polish rounds. No API or cargo calls."""

import json

from ai import loop, quality
from ai.targets import CJSON
from ai.translate import TranslateResult


def _msg(level, code, file, line=1):
    return json.dumps({"reason": "compiler-message", "message": {
        "level": level, "code": {"code": code} if code else None, "message": "m", "rendered": f"{code} at {line}",
        "spans": [{"file_name": file, "line_start": line, "is_primary": True}]}})


def test_parse_clippy_counts_only_the_generated_file():
    out = "\n".join([
        _msg("warning", "clippy::needless_range_loop", "src/cjson.rs"),
        _msg("warning", "dead_code", "src/cjson.rs"),
        _msg("warning", "clippy::indexing_slicing", "src/cjson.rs"),
        _msg("warning", "clippy::unwrap_used", "src/cjson.rs"),
        _msg("warning", "clippy::arithmetic_side_effects", "src/cjson.rs"),
        _msg("warning", "clippy::needless_range_loop", "src/main.rs"),  # harness file: ignored
        json.dumps({"reason": "compiler-message", "message": {"level": "warning", "message": "2 warnings emitted",
                                                              "code": None, "spans": []}}),
        "not json",
    ])
    q = quality.summarize(quality.parse_clippy(out, "cjson.rs"))
    assert (q["clippy_warnings"], q["panic_lints"], q["arithmetic_side_effects"]) == (2, 2, 1)
    assert not quality.clean(dict(q, fmt_ok=True))
    assert quality.clean({"fmt_ok": True, "clippy_errors": 0, "clippy_warnings": 0, "panic_lints": 0})
    lints = quality.format_lints(dict(q, fmt_ok=True, fmt_diff_lines=0, diagnostics=quality.parse_clippy(out, "cjson.rs")))
    assert "clippy::indexing_slicing" in lints and "arithmetic_side_effects (" not in lints


def _q(clippy=0, panic=0, fmt=True):
    return {"fmt_ok": fmt, "fmt_diff_lines": 0, "clippy_errors": 0, "clippy_warnings": clippy, "panic_lints": panic,
            "arithmetic_side_effects": 0, "by_lint": {}, "diagnostics": []}


def _host_fix(monkeypatch, tmp_path, fixed_rate, fixed_q):
    monkeypatch.setattr(quality, "autofix", lambda root, target, src: (src + "// fixed\n", {"changed": True}))
    monkeypatch.setattr(quality, "measure", lambda root, target, src: fixed_q)
    bv = lambda src, bid: ({"ok": True, "binary": "fixed-bin"}, {"match_rate": fixed_rate, "counts": {}})
    cand = loop.candidate(1, "src\n", "bin", {"match_rate": 1.0, "counts": {}}, _q(clippy=3, fmt=False))
    return loop.host_fix(cand, tmp_path, CJSON, tmp_path, bv, "b1h")


def test_host_fix_kept_only_when_behavior_holds(monkeypatch, tmp_path):
    kept, info = _host_fix(monkeypatch, tmp_path, 1.0, _q(clippy=1))
    assert info["applied"] and kept["binary"] == "fixed-bin" and kept["host_fixed"]
    kept, info = _host_fix(monkeypatch, tmp_path, 0.99, _q(clippy=0))
    assert not info["applied"] and kept["binary"] == "bin"
    kept, info = _host_fix(monkeypatch, tmp_path, 1.0, _q(clippy=4))  # lints got worse
    assert not info["applied"]


def _run_loop(monkeypatch, tmp_path, rates, qualities, rounds="4"):
    for rel in ("scripts/build_rust.sh", "backend/harness/verify.sh", "backend/harness/vendor/cJSON/cJSON.c",
                "backend/harness/vendor/cJSON/cJSON.h", "backend/harness/build/cjson_driver"):
        (tmp_path / rel).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / rel).write_text("")
    (tmp_path / "inputs").mkdir()
    results = iter(TranslateResult(rust_source=f"// round {n}\n", interaction_id=f"i{n}", seconds=1.0, model="m",
                                   usage={}) for n in range(1, 9))
    sent = []
    monkeypatch.setattr(loop, "translate", lambda *a, **k: next(results))
    monkeypatch.setattr(loop, "polish", lambda prev, src, lints, **k: (sent.append(src), next(results))[1])
    monkeypatch.setattr(loop, "repair", lambda prev, fb, **k: (sent.append(("repair", prev.rust_source, fb["match_rate"])),
                                                               next(results))[1])
    monkeypatch.setattr(loop, "trim_feedback", lambda fb, pat: fb)
    rates, qualities = iter(rates), iter(qualities)
    monkeypatch.setattr(loop, "run_json", lambda cmd, timeout, cwd=None: (
        ({"ok": True, "binary": f"bin{cmd[2][-1]}"}, "") if cmd[0].endswith("build_rust.sh")
        else ({"match_rate": next(rates), "counts": {"match": 1}}, "")))
    monkeypatch.setattr(loop.quality, "measure", lambda root, target, src: next(qualities))
    out = tmp_path / "out"
    assert loop.main(["--root", str(tmp_path), "--inputs", str(tmp_path / "inputs"), "--out", str(out),
                      "--rounds", rounds, "--no-holdout", "--no-adversarial", "--no-host-fix"]) == 0
    return json.loads((out / "summary.json").read_text()), sent


def test_polish_round_replaces_best_when_lints_drop(monkeypatch, tmp_path):
    summary, sent = _run_loop(monkeypatch, tmp_path, [1.0, 1.0], [_q(clippy=3, panic=40), _q()])
    assert sent == ["// round 1\n"]  # the polish turn got the best build's file
    assert summary["best_round"] == 2 and summary["polish_rounds_run"] == 1
    assert summary["quality"]["clean"] and summary["quality_first_build"]["clippy_warnings"] == 3
    assert summary["panics"] == {"loop": 0, "total": 0}


def test_broken_polish_is_never_chosen_and_gets_a_repair(monkeypatch, tmp_path):
    summary, sent = _run_loop(monkeypatch, tmp_path, [1.0, 0.98, 1.0], [_q(clippy=3), _q(), _q()])
    assert [r["kind"] for r in summary["rounds"]] == ["translate", "polish", "polish_repair"]
    assert sent[1] == ("repair", "// round 2\n", 0.98)  # the repair fixes the polished file
    assert summary["best_round"] == 3 and summary["quality"]["clean"]


def test_polish_that_stays_broken_keeps_the_matching_build(monkeypatch, tmp_path):
    summary, _ = _run_loop(monkeypatch, tmp_path, [1.0, 0.98, 0.99, 0.97], [_q(clippy=3), _q(), _q(), _q()], rounds="6")
    assert summary["best_round"] == 1 and summary["best_match_rate"] == 1.0
    assert summary["rounds_run"] == 1 + CJSON.polish_rounds


def test_matching_round_that_is_not_cleaner_stops(monkeypatch, tmp_path):
    summary, _ = _run_loop(monkeypatch, tmp_path, [1.0, 1.0], [_q(clippy=3), _q(clippy=3)], rounds="5")
    assert summary["rounds_run"] == 2 and summary["best_round"] == 2  # a tie goes to the later round


def test_polish_stops_at_target_cap(monkeypatch, tmp_path):
    summary, _ = _run_loop(monkeypatch, tmp_path, [1.0] * 5, [_q(clippy=c) for c in (9, 6, 3, 2, 1)], rounds="6")
    assert summary["polish_rounds_run"] == CJSON.polish_rounds == 3 and summary["best_round"] == 4
