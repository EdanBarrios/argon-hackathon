from pathlib import Path

from ai import prompts
from ai.feedback_patterns import lua_divergence_pattern
from ai.targets import TARGETS


def test_lua_target_registered():
    t = TARGETS["lua"]
    assert t.rust_file == "lua.rs" and t.polish_rounds == 3 and t.max_output_tokens == 1048576
    assert "unsafe" not in prompts.LUA_SYSTEM_INSTRUCTION.split("Hard rules:")[0]


def test_lua_source_skips_out_of_scope_files(tmp_path: Path):
    src = tmp_path / "src"
    src.mkdir()
    for name in ("lvm.c", "lvm.h", "liolib.c", "lua.c", "ldump.c"):
        (src / name).write_text(f"/* {name} */")
    blob = prompts.load_lua_source(tmp_path)
    assert "lvm.c" in blob and "lvm.h" in blob
    assert "liolib.c" not in blob and "lua.c" not in blob and "ldump.c" not in blob
    assert blob.index("lvm.h") < blob.index("lvm.c")  # headers first


def test_lua_divergence_pattern():
    err = lambda m: f"1\nERROR input:3: {m}"
    assert lua_divergence_pattern({"c_output": err("attempt to index a nil value (global 'x')"),
                                   "rust_output": err("attempt to call a nil value")}) == \
        "C ERROR attempt to index vs Rust ERROR attempt to call"
    assert lua_divergence_pattern({"c_output": "ok", "rust_output": err("x y z")}) == \
        "C output vs Rust ERROR x y z"
    assert lua_divergence_pattern({"c_output": "TIMEOUT", "rust_output": "TIMEOUT"}) == "both TIMEOUT"
    assert "first differing line" in lua_divergence_pattern(
        {"c_output": "a\nb", "rust_output": "a\nc", "first_diff": {"c": "b", "rust": "c"}})
