import pytest

from ai import patch, translate
from ai.patch import PatchError, apply_patch

SRC = "fn a() {\n    let x = 1;\n    x\n}\n\nfn b() {\n    let x = 1;\n    x + 1\n}\n"


def blocks(*pairs: tuple[str, str]) -> str:
    return "\n".join(f"<<<<<<< SEARCH\n{s}\n=======\n{r}\n>>>>>>> REPLACE" for s, r in pairs)


def test_applies_one_block():
    out, stats = apply_patch(SRC, blocks(("    x + 1", "    x + 2")))
    assert out == SRC.replace("x + 1", "x + 2")
    assert (stats.blocks, stats.lines_removed, stats.lines_added) == (1, 1, 1)


def test_ignores_prose_around_blocks_and_trailing_whitespace():
    reply = "Here is the fix:\n" + blocks(("fn a() {   ", "fn a() -> i32 {")) + "\nDone."
    out, _ = apply_patch(SRC, reply)
    assert out.startswith("fn a() -> i32 {\n")


def test_ambiguous_search_is_rejected():
    with pytest.raises(PatchError, match="matches 2 places"):
        apply_patch(SRC, blocks(("    let x = 1;", "    let x = 2;")))


def test_context_lines_disambiguate():
    out, _ = apply_patch(SRC, blocks(("fn b() {\n    let x = 1;", "fn b() {\n    let x = 5;")))
    assert out.count("let x = 1;") == 1 and "let x = 5;" in out


def test_missing_search_is_rejected():
    with pytest.raises(PatchError, match="not found"):
        apply_patch(SRC, blocks(("fn nope() {}", "")))


def test_blank_search_is_rejected():
    with pytest.raises(PatchError, match="empty or blank"):
        apply_patch(SRC, blocks(("", "fn c() {}")))


def test_all_or_nothing_and_blocks_apply_in_order():
    good, bad = ("    x + 1", "    x + 3"), ("fn nope() {}", "")
    with pytest.raises(PatchError):
        apply_patch(SRC, blocks(good, bad))
    # a later block can search text an earlier block wrote
    out, stats = apply_patch(SRC, blocks(good, ("    x + 3", "    x + 4")))
    assert "x + 4" in out and stats.blocks == 2


def test_delete_with_empty_replacement():
    out, _ = apply_patch(SRC, blocks(("fn b() {\n    let x = 1;\n    x + 1\n}", "")))
    assert "fn b" not in out


def test_no_blocks_and_truncated_reply():
    with pytest.raises(PatchError, match="no SEARCH/REPLACE"):
        apply_patch(SRC, "```rust\nfn a() {}\n```")
    with pytest.raises(PatchError, match="truncated"):
        apply_patch(SRC, "<<<<<<< SEARCH\n    x + 1\n=======\n    x + 2\n")
    with pytest.raises(PatchError, match="empty"):
        apply_patch(SRC, None)


def _result(src: str, **kw) -> translate.TranslateResult:
    usage = {k: 0 for k in translate.USAGE_FIELDS}
    usage["total_output_tokens"] = kw.pop("out", 0)
    return translate.TranslateResult(rust_source=src, interaction_id="i1", usage=usage,
                                     seconds=kw.pop("seconds", 1.0), model="m", **kw)


def test_repair_patch_mode_and_fallback(monkeypatch):
    prev = _result(SRC)

    # Patch applies: one call, repair_mode "patch".
    def run_ok(model, message, thinking, previous_id, target=None, parse=translate.extract_rust):
        assert "SEARCH" in message and SRC.strip() in message
        src = parse(blocks(("    x + 1", "    x + 9")), [])
        return _result(src, out=40, seconds=2.0)

    monkeypatch.setattr(translate, "_run", run_ok)
    res = translate.repair(prev, {"divergences": []}, patch=True)
    assert res.repair_mode == "patch" and "x + 9" in res.rust_source

    # Patch does not apply: the full-file call runs and the result adds up both calls.
    seen = []

    def run_bad_then_full(model, message, thinking, previous_id, target=None, parse=translate.extract_rust):
        seen.append(parse)
        if parse is translate.extract_rust:
            return _result("fn full() {}\n", out=1000, seconds=30.0)
        try:
            parse(blocks(("fn nope() {}", "")), [])
        except PatchError as exc:
            usage = {k: 0 for k in translate.USAGE_FIELDS}
            usage["total_output_tokens"] = 40
            raise translate.PatchFailed(str(exc), usage, 2.0, []) from exc

    monkeypatch.setattr(translate, "_run", run_bad_then_full)
    res = translate.repair(prev, {"divergences": []}, patch=True)
    assert res.repair_mode == "patch_fallback" and res.rust_source == "fn full() {}\n"
    assert res.usage["total_output_tokens"] == 1040 and res.seconds == 32.0
    assert len(seen) == 2


def test_patch_module_exports():
    assert patch.PatchError is PatchError
