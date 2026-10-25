"""Independent recheck of one ai.loop run's claims. Imports nothing from backend/ai.

usage: python3 recheck.py <repo root> <run dir>
"""
import json, re, shutil, subprocess, sys, tempfile
from pathlib import Path

root, run = Path(sys.argv[1]), Path(sys.argv[2])
s = json.loads((run / "summary.json").read_text())
best = Path(s["best_file"])
src = best.read_text()
rows = []

def check(name, claimed, got):
    rows.append((name, claimed, got, "OK" if claimed == got else "MISMATCH"))

# 1. best file is byte-identical to the round file (or its host fix) it claims to be
r = s["best_round"]
origin = run / f"round_{r}" / ("host_fix" if s["best_host_fixed"] else "") / "cjson.rs"
check("best file == its round's file", True, origin.read_text() == src)
check("no 'unsafe' keyword", True, re.search(r"\bunsafe\b", src) is None)

# 2. fresh build, then verify.sh on the same held-out dir and the adversarial corpora
bid = f"recheck-{run.name}"
b = json.loads(subprocess.run([str(root / "scripts/build_rust.sh"), str(best), bid],
                              capture_output=True, text=True).stdout)
check("fresh build ok", True, b.get("ok"))
hv = json.loads(subprocess.run([str(root / "backend/harness/verify.sh"), "--c", str(root / "backend/harness/build/cjson_driver"),
                                "--rust", b["binary"], "--inputs", s["holdout"]["inputs"], "--timeout", "20"],
                               capture_output=True, text=True).stdout)
check("held-out match", s["holdout"]["counts"]["match"], hv["counts"]["match"])
check("held-out tested", s["holdout"]["behaviors_tested"], hv["behaviors_tested"])
check("held-out rust crashes", 0, hv["counts"].get("rust_crash", 0) + hv["counts"].get("both_crash", 0))
subprocess.run([str(root / "backend/harness/adversarial/run.sh"), b["binary"], bid, "targeted", "argon", "human"],
               capture_output=True, text=True)
for corpus in ("targeted", "argon"):
    rep = json.loads((root / f"runs/adversarial/{bid}-{corpus}.json").read_text())
    check(f"{corpus} match", s["adversarial"][corpus]["counts"]["match"], rep["counts"]["match"])
    check(f"{corpus} rust crashes", 0, rep["counts"].get("rust_crash", 0) + rep["counts"].get("both_crash", 0))

# 3. lints from plain cargo text output in a fresh crate copy
with tempfile.TemporaryDirectory() as t:
    crate = Path(t) / "c"
    shutil.copytree(root / "backend/harness/rust_crate", crate, ignore=shutil.ignore_patterns("target"))
    (crate / "src/cjson.rs").write_text(src)
    fmt = subprocess.run(["rustfmt", "--edition", "2021", "--check", "src/cjson.rs"], cwd=crate, capture_output=True)
    def lints(*extra):
        p = subprocess.run(["cargo", "clippy", "--offline", "--message-format=short", "--", *extra],
                           cwd=crate, capture_output=True, text=True)
        return [l for l in p.stderr.splitlines() if l.startswith("src/cjson.rs:") and ": warning:" in l]
    default = lints()
    panicky = [l for l in lints("-W", "clippy::indexing_slicing", "-W", "clippy::unwrap_used",
                                "-W", "clippy::expect_used", "-W", "clippy::panic")
               if re.search(r"(indexing|slicing) may panic|used `unwrap|used `expect|`panic` should not", l)]
q = s["quality"]
check("fmt ok", q["fmt_ok"], fmt.returncode == 0)
check("default clippy warnings", q["clippy_warnings"], len(default))
check("panic lints", q["panic_lints"], len(panicky))

w = max(len(x[0]) for x in rows)
for name, c, g, v in rows:
    print(f"{name:<{w}}  claimed={c!s:<6} recheck={g!s:<6} {v}")
print("ALL CLAIMS HOLD" if all(x[3] == "OK" for x in rows) else "SOME CLAIMS FAIL")
