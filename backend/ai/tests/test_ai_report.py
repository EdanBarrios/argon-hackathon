"""Offline tests for best-build selection, the adversarial check and ai.report. No API calls."""

import json
import stat

from ai import loop, report
from ai.translate import TranslateResult


def _result(n):
    return TranslateResult(rust_source=f"// round {n}\n", interaction_id=f"i{n}", seconds=1.0, model="m",
                           usage={"total_input_tokens": 1_000_000, "total_output_tokens": 100_000})


def test_holdout_uses_best_build_not_last(tmp_path, monkeypatch):
    for rel in ("scripts/build_rust.sh", "backend/harness/verify.sh", "backend/harness/vendor/cJSON/cJSON.c",
                "backend/harness/vendor/cJSON/cJSON.h",
                "backend/harness/build/cjson_driver"):
        (tmp_path / rel).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / rel).write_text("")
    inputs = tmp_path / "inputs"
    inputs.mkdir()
    calls = iter([_result(n) for n in range(1, 4)])
    monkeypatch.setattr(loop, "translate", lambda *a, **k: next(calls))
    monkeypatch.setattr(loop, "repair", lambda *a, **k: next(calls))
    monkeypatch.setattr(loop, "trim_feedback", lambda fb, pat: fb)
    rates = iter([0.9, 0.5])  # round 2 regresses; round 3 fails to compile
    final_binary = []

    def fake_run_json(cmd, timeout, cwd=None):
        if cmd[0].endswith("build_rust.sh"):
            n = cmd[2][-1]
            return ({"ok": True, "binary": f"bin{n}"} if n != "3" else {"ok": False, "errors": "E"}), ""
        if "--inputs" in cmd and cmd[cmd.index("--inputs") + 1].endswith("final"):
            final_binary.append(cmd[cmd.index("--rust") + 1])
        return {"match_rate": next(rates, 1.0), "counts": {}}, ""

    monkeypatch.setattr(loop, "run_json", fake_run_json)
    final = tmp_path / "final"
    final.mkdir()
    out = tmp_path / "out"
    rc = loop.main(["--root", str(tmp_path), "--inputs", str(inputs), "--out", str(out), "--rounds", "3",
                    "--no-holdout", "--no-adversarial", "--final-inputs", str(final)])
    assert rc == 0
    summary = json.loads((out / "summary.json").read_text())
    assert final_binary == ["bin1"]
    assert summary["best_round"] == 1 and summary["best_match_rate"] == 0.9
    assert summary["usd"] == 9.0  # 3 calls x ($2 in + $1 out)


def test_run_adversarial_reads_reports_and_drops_stale(tmp_path):
    script = tmp_path / "run.sh"
    script.write_text(
        "#!/bin/sh\n"
        'printf \'{"match_rate": 0.5, "behaviors_tested": 2, "counts": {"match": 1}}\' '
        f'> "{tmp_path}/runs/adversarial/$2-targeted.json"\n'
    )
    script.chmod(script.stat().st_mode | stat.S_IEXEC)
    adv = tmp_path / "runs/adversarial"
    adv.mkdir(parents=True)
    (adv / "lbl-argon.json").write_text('{"match_rate": 1.0}')  # stale, from an earlier run
    (adv / "lbl-x-targeted.json").write_text('{"match_rate": 1.0}')  # another label sharing the prefix
    out = loop.run_adversarial(tmp_path, "run.sh", "bin", "lbl")
    assert set(out) == {"targeted"}
    assert out["targeted"]["counts"] == {"match": 1}


def test_failure_reason_and_cost_of_incomplete_call():
    err = ("interaction v1_X ended with status 'incomplete' after 917.0s, usage {'total_input_tokens': 1000000, "
           "'total_output_tokens': 0, 'total_thought_tokens': 100000, 'total_cached_tokens': 0}")
    assert report.failure(err) == ("incomplete after 917.0s", 3.0)
    assert report.failure("GEMINI_API_KEY not found")[1] is None


def test_report_rows_and_groups(tmp_path):
    def write(name, s):
        (tmp_path / name).mkdir()
        (tmp_path / name / "summary.json").write_text(json.dumps(s))

    base = {"model": "m", "target": "cjson", "rounds_budget": 5, "rounds_run": 1, "final_match_rate": 1.0,
            "usd": 0.5, "rounds": []}
    write("a", dict(base, holdout={"match_rate": 1.0, "behaviors_tested": 763, "counts": {"match": 763}},
                    adversarial={"targeted": {"match_rate": 0.98, "behaviors_tested": 980, "counts": {"match": 961}}}))
    write("b", dict(base, holdout={"match_rate": 0.99, "behaviors_tested": 763, "counts": {"match": 755}}))
    rows = report.collect([tmp_path])
    assert [r["targeted"] for r in rows] == ["961/980", "-"]
    (g,) = report.groups(rows)
    assert (g["n"], g["holdout_min"], g["reached_1.0"], g["usd"]) == (2, 0.99, 1, 1.0)
