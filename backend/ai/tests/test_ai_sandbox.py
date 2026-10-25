"""Offline tests for the held-out check and the sandbox pre-run. No API calls."""

import argparse

from ai import loop, prompts, sandbox_run


def test_parse_claims_reads_round_lines_only():
    text = (
        "setup done\n"
        "ROUND 1 build=fail match_rate=-\n"
        "ROUND 2 build=ok match_rate=0.93\n"
        "  ROUND 3 build=ok match_rate=1.0\n"
        "```rust\nfn main() {}\n```\n"
    )
    assert sandbox_run.parse_claims(text) == [
        {"round": 1, "build_ok": False, "match_rate": None},
        {"round": 2, "build_ok": True, "match_rate": 0.93},
        {"round": 3, "build_ok": True, "match_rate": 1.0},
    ]
    assert sandbox_run.parse_claims(None) == []


def _args(**kw):
    base = dict(model="gemini-4-argon", branch="main", seed=1, count=300, rounds=5, environment=None)
    base.update(kw)
    return argparse.Namespace(**base)


def test_request_uses_allowlist_or_reuses_environment():
    fresh = sandbox_run.request(_args(), "C SOURCE")
    assert fresh["agent"] == sandbox_run.AGENT
    assert fresh["agent_config"] == {"type": "antigravity", "model": "gemini-4-argon"}
    assert fresh["background"] is True
    domains = {e["domain"] for e in fresh["environment"]["network"]["allowlist"]}
    assert {"github.com", "archive.ubuntu.com", "sh.rustup.rs"} <= domains
    # Same system instruction as the local loop, so Flash/Argon comparisons stay fair.
    assert fresh["system_instruction"] == prompts.SYSTEM_INSTRUCTION
    assert "C SOURCE" in fresh["input"]

    reused = sandbox_run.request(_args(environment="env-123"), "C SOURCE")
    assert reused["environment"] == "env-123"


def test_sandbox_prompt_names_branch_seed_and_rounds():
    text = prompts.sandbox_input("C", branch="feat/x", seed=7, count=42, rounds=3)
    assert "--branch feat/x" in text
    assert "--seed 7 --count 42" in text
    assert "at most 3 rounds" in text
    assert prompts.REPO_URL in text
    # translate_input still ends with its own instruction after the refactor.
    assert prompts.translate_input("C").endswith("containing the whole file.")


def test_holdout_removes_inputs_the_loop_saw(tmp_path, monkeypatch):
    seen = tmp_path / "seen"
    seen.mkdir()
    (seen / "a.json").write_bytes(b"[1]")
    (seen / "b.json").write_bytes(b"{}")
    generated = tmp_path / "gen"
    generated.mkdir()
    (generated / "x.json").write_bytes(b"[1]")  # same bytes as a.json, different name
    (generated / "y.json").write_bytes(b"[2]")
    (generated / "z.json").write_bytes(b"{}")
    monkeypatch.setattr(loop, "ensure_inputs", lambda root, count, seed, target=None: generated)

    out, removed = loop.holdout_inputs(tmp_path, 3, 1001, seen)
    assert removed == 2
    assert sorted(p.name for p in out.iterdir()) == ["y.json"]

    # Re-running replaces the directory rather than accumulating stale files.
    out2, _ = loop.holdout_inputs(tmp_path, 3, 1001, seen)
    assert out2 == out and sorted(p.name for p in out.iterdir()) == ["y.json"]


def test_dry_run_makes_no_api_call(tmp_path, monkeypatch, capsys):
    vendor = tmp_path / "backend/harness/vendor/cJSON"
    vendor.mkdir(parents=True)
    (vendor / "cJSON.c").write_text("int x;\n")
    (vendor / "cJSON.h").write_text("/* h */\n")

    def no_client():
        raise AssertionError("dry run must not create a client")

    monkeypatch.setattr(sandbox_run, "_get_client", no_client)
    assert sandbox_run.main(["--root", str(tmp_path), "--dry-run", "--branch", "feat/x"]) == 0
    out = capsys.readouterr().out
    assert '"agent": "antigravity-preview-09-2026"' in out
    assert "--branch feat/x" in out


def test_wait_gives_up_after_max_seconds(monkeypatch):
    import types

    import pytest

    from ai import translate

    stuck = types.SimpleNamespace(id="i-1", status="in_progress")
    client = types.SimpleNamespace(interactions=types.SimpleNamespace(get=lambda _id: stuck))
    monkeypatch.setattr(translate, "POLL_SECONDS", 0)
    with pytest.raises(translate.TranslateError, match="still 'in_progress'"):
        translate._wait(client, stuck, max_seconds=0.05)


def test_cjson_target_keeps_the_original_prompt_words():
    from ai.targets import CJSON

    assert CJSON.system_instruction == prompts.SYSTEM_INSTRUCTION
    assert CJSON.translate_input("C") == prompts.translate_input("C")
    assert CJSON.repair_input("fb") == prompts.repair_input("fb")


def test_first_diff_and_expat_patterns():
    from ai.feedback_patterns import expat_divergence_pattern

    assert loop.first_diff(b"START \"a\"\nTEXT \"x\"\nOK\n", b"START \"a\"\nTEXT \"y\"\nOK\n") == \
        {"line": 2, "c": 'TEXT "x"', "rust": 'TEXT "y"'}
    assert loop.first_diff(b"A\nOK\n", b"A\n")["c"] == "OK"
    pat = expat_divergence_pattern
    assert pat({"c_output": "ERROR 4 line 1 col 2", "rust_output": "ERROR 2 line 1 col 0"}) == \
        "both reject: C ERROR 4, Rust ERROR 2"
    assert pat({"c_output": "ERROR 7 line 2 col 5", "rust_output": "ERROR 7 line 2 col 4"}) == \
        "both reject with ERROR 7, line/col differ"
    assert pat({"c_output": "START \"a\"", "rust_output": "ERROR 2 line 1 col 0"}) == "C events vs Rust ERROR 2"
    assert pat({"c_output": "START", "rust_output": "START",
                "first_diff": {"line": 3, "c": "END \"a\"", "rust": "TEXT \"b\""}}) == \
        "both accept, first differing line: C END vs Rust TEXT"


def test_continuation_joins_a_cut_off_reply(monkeypatch):
    import dataclasses
    import types

    from ai import translate
    from ai.targets import EXPAT

    replies = iter([
        types.SimpleNamespace(id="i1", status="incomplete", output_text="```rust\nfn a() {}\nfn b(", usage=None),
        types.SimpleNamespace(id="i2", status="completed", output_text="```rust\n) {}\n```\n", usage=None),
    ])
    sent = []

    def create(**kw):
        sent.append(kw)
        return next(replies)

    client = types.SimpleNamespace(interactions=types.SimpleNamespace(create=create, get=None))
    monkeypatch.setattr(translate, "_get_client", lambda: client)
    res = translate._run("m", "go", "high", None, target=dataclasses.replace(EXPAT, continue_turns=2))
    assert res.rust_source == "fn a() {}\nfn b() {}\n"
    assert res.continuations == 1 and res.interaction_id == "i2"
    assert sent[1]["previous_interaction_id"] == "i1" and sent[1]["input"] == prompts.CONTINUE_INPUT
