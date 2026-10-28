"""Run CPython 3.13.5's XML test suites against a given pyexpat build and the stock one.

usage: python3 run_tests.py <out-dir-with-our-pyexpat> [--label ours] [--no-baseline]
Writes full logs + summary.json under runs/pyexpat/<label>/ and prints a table.
"""
import json, os, re, shutil, subprocess, sys, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE / "vendor" / "cpython" / "Lib" / "test"
SUITES = ["test_pyexpat", "test_sax", "test_minidom", "test_xml_etree", "test_xml_etree_c"]
PY = os.environ.get("PYTHON", sys.executable)
RUNS = Path(os.environ.get("RUNS_DIR", HERE.parents[2] / "runs" / "pyexpat"))


def overlay() -> Path:
    """A `test` package = stdlib test.support + v3.13.5 test files + xmltestdata."""
    import test as stdtest
    root = RUNS / "overlay"
    pkg = root / "test"
    if pkg.exists():
        shutil.rmtree(pkg)
    shutil.copytree(Path(stdtest.__file__).parent, pkg,
                    ignore=shutil.ignore_patterns("__pycache__"))
    for s in SUITES:
        shutil.copy(SRC / f"{s}.py", pkg / f"{s}.py")
    shutil.copytree(SRC / "xmltestdata", pkg / "xmltestdata")
    return root


def parse(out: str) -> dict:
    ran = re.findall(r"^Ran (\d+) tests?", out, re.M)
    r = {"run": int(ran[-1]) if ran else 0, "failures": 0, "errors": 0, "skipped": 0,
         "expected_failures": 0}
    tail = re.findall(r"^(OK|FAILED)(?: \((.*)\))?\s*$", out, re.M)
    if tail:
        for kv in (tail[-1][1] or "").split(","):
            if "=" in kv:
                k, v = kv.strip().split("=")
                k = {"expected failures": "expected_failures"}.get(k, k)
                r[k] = int(v)
    else:
        r["crashed"] = True
    r["passed"] = r["run"] - r["failures"] - r["errors"] - r["skipped"] - r["expected_failures"]
    return r


def run(label: str, extra_path: str | None, root: Path) -> dict:
    d = RUNS / label
    d.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    env["PYTHONPATH"] = os.pathsep.join(p for p in [extra_path, str(root)] if p)
    env.pop("PYTHONHOME", None)
    res = {}
    probe = subprocess.run([PY, "-c", "import pyexpat,_elementtree;print(pyexpat.__file__);"
                            "print(pyexpat.EXPAT_VERSION);print(_elementtree.__file__)"],
                           env=env, capture_output=True, text=True)
    (d / "probe.txt").write_text(probe.stdout + probe.stderr)
    if extra_path and probe.stdout.count(extra_path) < 2:
        sys.exit(f"{label}: our pyexpat/_elementtree did not load from {extra_path}:\n"
                 f"{probe.stdout}{probe.stderr}")
    for s in SUITES:
        t = time.time()
        try:
            p = subprocess.run([PY, "-m", "unittest", "-v", f"test.{s}"], env=env, cwd=root,
                               capture_output=True, text=True, timeout=600)
            out = p.stdout + p.stderr
            code = p.returncode
        except subprocess.TimeoutExpired as e:
            out = (e.stdout or b"").decode() + (e.stderr or b"").decode() + "\nTIMEOUT\n"
            code = "timeout"
        (d / f"{s}.log").write_text(out)
        r = parse(out)
        r["returncode"] = code
        r["seconds"] = round(time.time() - t, 1)
        res[s] = r
    (d / "summary.json").write_text(json.dumps({"probe": probe.stdout, "suites": res}, indent=2))
    return {"probe": probe.stdout.strip().splitlines(), "suites": res}


def table(label, r):
    print(f"\n== {label}: {' | '.join(r['probe'])}")
    print(f"{'suite':20} {'run':>5} {'pass':>5} {'fail':>5} {'err':>5} {'skip':>5} {'xfail':>5}")
    tot = dict.fromkeys(["run", "passed", "failures", "errors", "skipped", "expected_failures"], 0)
    for s, x in r["suites"].items():
        print(f"{s:20} {x['run']:>5} {x['passed']:>5} {x['failures']:>5} {x['errors']:>5} "
              f"{x['skipped']:>5} {x['expected_failures']:>5}" + ("  CRASHED" if x.get("crashed") else ""))
        for k in tot:
            tot[k] += x[k]
    print(f"{'TOTAL':20} {tot['run']:>5} {tot['passed']:>5} {tot['failures']:>5} {tot['errors']:>5} "
          f"{tot['skipped']:>5} {tot['expected_failures']:>5}")


if __name__ == "__main__":
    args = sys.argv[1:]
    out_dir = str(Path(args[0]).resolve())
    label = args[args.index("--label") + 1] if "--label" in args else "ours"
    root = overlay()
    ours = run(label, out_dir, root)
    table(label, ours)
    if "--no-baseline" not in args:
        base = run("baseline", None, root)
        table("baseline (stock pyexpat, expat 2.7.1)", base)
