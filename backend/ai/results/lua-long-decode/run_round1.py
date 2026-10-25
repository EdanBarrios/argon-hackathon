"""Lua 5.4.7 round 1: one Argon translate call, before the systems harness exists.

Same code path as ai.loop's round 1 (translate.translate with targets.LUA, thinking high),
with prompts.LUA_CRATE_DIR / LUA_DRIVER_C pointed at a scratch crate and driver that
follow backend/ai/lua/HARNESS_SPEC.md. Then builds the reply in that crate.

usage (from backend/): uv run python run_round1.py <lua-run dir> <vendor lua-5.4.7 dir>
"""
import json, re, shutil, subprocess, sys, time
from pathlib import Path

from ai import loop, prompts, translate
from ai.targets import LUA

run, vendor = Path(sys.argv[1]), Path(sys.argv[2])
prompts.LUA_CRATE_DIR = run / "rust_crate"
prompts.LUA_DRIVER_C = run / "drivers" / "lua_driver.c"
out = run / "out" / "round_1"
out.mkdir(parents=True, exist_ok=True)

c_source = LUA.load_source(vendor)
(out / "prompt_user.txt").write_text(LUA.translate_input(c_source))
(out / "prompt_system.txt").write_text(LUA.system_instruction)
started = time.strftime("%H:%M:%S")
try:
    r = translate.translate(c_source, "gemini-4-argon", LUA)
except Exception as e:  # record the failure, never hide it
    (out / "meta.json").write_text(json.dumps({"started": started, "error": repr(e)}, indent=2))
    raise
(out / "lua.rs").write_text(r.rust_source)
meta = {"model": r.model, "interaction_id": r.interaction_id, "thinking_level": r.thinking_level,
        "started": started, "seconds": r.seconds, "usage": r.usage, "usd": loop.usd(r.usage),
        "notes": r.notes, "continuations": r.continuations,
        "rust_lines": r.rust_source.count("\n"), "has_unsafe_word": bool(re.search(r"\bunsafe\b", r.rust_source))}

# Build in a copy of the scratch crate.
crate = run / "build" / "crate-r1"
if crate.exists():
    shutil.rmtree(crate)
shutil.copytree(run / "rust_crate", crate)
(crate / "src" / "lua.rs").write_text(r.rust_source)
t = time.monotonic()
b = subprocess.run(["cargo", "build", "--release", "--offline", "--message-format=short"],
                   cwd=crate, capture_output=True, text=True)
errs = [l for l in b.stderr.splitlines() if l.startswith("src/lua.rs:") and ": error" in l]
meta["build"] = {"ok": b.returncode == 0, "seconds": round(time.monotonic() - t, 1),
                 "errors": len(errs), "codes": sorted(set(re.findall(r"error\[(E\d+)\]", b.stderr)))}
(out / "build_stderr.txt").write_text(b.stderr)
(out / "meta.json").write_text(json.dumps(meta, indent=2))
print(json.dumps({k: meta[k] for k in ("seconds", "usage", "usd", "rust_lines", "build")}))
