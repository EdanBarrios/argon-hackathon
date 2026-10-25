"""Independent recheck of one libexpat ai.loop run's claims. Imports nothing from backend/ai.

Adapted from results/cjson-quality/recheck.py: libexpat has no adversarial corpus, so
the behavior checks are the loop inputs and the held-out set instead.

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
origin = run / f"round_{r}" / ("host_fix" if s["best_host_fixed"] else "") / "expat.rs"
check("best file == its round's file", True, origin.read_text() == src)
check("no 'unsafe' keyword", True, re.search(r"\bunsafe\b", src) is None)

# 2. fresh build, then verify.sh on the same loop and held-out dirs
bid = f"recheck-expat-{run.name}"
b = json.loads(subprocess.run([str(root / "scripts/build_rust_expat.sh"), str(best), bid],
                              capture_output=True, text=True).stdout)
check("fresh build ok", True, b.get("ok"))

def verify(inputs):
    return json.loads(subprocess.run([str(root / "backend/harness/verify.sh"),
                                      "--c", str(root / "backend/harness/expat/build/expat_driver"),
                                      "--rust", b["binary"], "--inputs", inputs, "--timeout", "20"],
                                     capture_output=True, text=True).stdout)

def crashes(counts):
    return counts.get("rust_crash", 0) + counts.get("both_crash", 0)

loop_claim = json.loads((run / f"round_{r}" / ("host_fix" if s["best_host_fixed"] else "") / "verify.json").read_text())
lv = verify(s["inputs"])
check("loop match", loop_claim["counts"]["match"], lv["counts"]["match"])
check("loop rust crashes", 0, crashes(lv["counts"]))
hv = verify(s["holdout"]["inputs"])
check("held-out match", s["holdout"]["counts"]["match"], hv["counts"]["match"])
check("held-out tested", s["holdout"]["behaviors_tested"], hv["behaviors_tested"])
check("held-out rust crashes", 0, crashes(hv["counts"]))

# 3. lints from plain cargo text output in a fresh crate copy
with tempfile.TemporaryDirectory() as t:
    crate = Path(t) / "c"
    shutil.copytree(root / "backend/harness/expat/rust_crate", crate, ignore=shutil.ignore_patterns("target"))
    (crate / "src/expat.rs").write_text(src)
    fmt = subprocess.run(["rustfmt", "--edition", "2021", "--check", "src/expat.rs"], cwd=crate, capture_output=True)
    # JSON, keyed by full primary span: short format prints only the start position, so
    # nested indexing like a[i].b[j] (two sites, same start) collapses to one line there.
    def lints(*extra):
        p = subprocess.run(["cargo", "clippy", "--offline", "--message-format=json", "--", *extra],
                           cwd=crate, capture_output=True, text=True)
        out = set()
        for line in p.stdout.splitlines():
            m = json.loads(line)
            msg = m.get("message") or {}
            if m.get("reason") != "compiler-message" or msg.get("level") != "warning":
                continue
            sp = next((x for x in msg.get("spans") or [] if x["is_primary"]), None)
            if sp and sp["file_name"] == "src/expat.rs":
                out.add(((msg.get("code") or {}).get("code"), sp["line_start"], sp["column_start"],
                         sp["line_end"], sp["column_end"]))
        return out
    default = lints()
    panicky = [l for l in lints("-W", "clippy::indexing_slicing", "-W", "clippy::unwrap_used",
                                "-W", "clippy::expect_used", "-W", "clippy::panic")
               if l[0] in ("clippy::indexing_slicing", "clippy::unwrap_used", "clippy::expect_used", "clippy::panic")]
q = s["quality"]
check("fmt ok", q["fmt_ok"], fmt.returncode == 0)
check("default clippy warnings", q["clippy_warnings"], len(default))
check("panic lints", q["panic_lints"], len(panicky))

w = max(len(x[0]) for x in rows)
for name, c, g, v in rows:
    print(f"{name:<{w}}  claimed={c!s:<6} recheck={g!s:<6} {v}")
print("ALL CLAIMS HOLD" if all(x[3] == "OK" for x in rows) else "SOME CLAIMS FAIL")
