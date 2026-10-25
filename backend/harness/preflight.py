"""Verifier preflight gate: prove the verifier works before trusting any match rate.

Stdlib only. Run through scripts/preflight.sh <cjson|expat> [options].

"The verifier passed while broken" happened (FRICTION-LOG #1: a wrong buffer
length made the driver reject everything, and C vs C still scored 1.0). This
gate runs every check that would have caught it, on a small input set, and
must pass before any model call:

  canary   one known input must print the exact expected line(s)
  c_vs_c   the C driver against itself: match_rate 1.0, no infra failures
  stub     the C driver against the reject-everything stub crate: match_rate
           at most --stub-max (proves the verifier can tell two programs apart)
  mutants  (cjson, opt-in with --mutants: ~75 s idle, 400 s on a busy laptop) single-site cJSON.c
           mutants built and verified against the real driver: kill_rate at
           least --min-kill-rate (proves the inputs catch real C bugs)

Inputs: --count generated inputs at --seed into a private work dir under
runs/preflight/ (never the shared inputs/ dirs), plus for cjson the committed
adversarial/corpus/mutant_gaps/ files. Logs go to stderr; stdout gets exactly
one JSON line. Exit 0 if every check passed, 1 if a check failed, 2 on a
setup error (fetch, build, input generation).
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
VERIFY = os.path.join(HERE, "verify.sh")

CJSON_CANARY = (b'{"a": [1, 2.5, "x"], "b": null}', b'OK\t{"a":[1,2.5,"x"],"b":null}\n')
EXPAT_CANARY = (
    b'<?xml version="1.0"?>\n<!DOCTYPE d [<!ENTITY e "x&amp;y">]>\n<d b="&lt;" a="1"><!--c--><?p q?>'
    b'hi &e; <![CDATA[<z>]]>\xc3\xa9<e/></d>',
    b'XMLDECL "1.0" null -1\nDOCTYPE_START "d" null null 1\nENTITY "e" 0 VALUE "x&amp;y"\nDOCTYPE_END\n'
    b'START "d" "b"="<" "a"="1"\nCOMMENT "c"\nPI "p" "q"\nTEXT "hi x&y "\nCDATA_START\nTEXT "<z>"\n'
    b'CDATA_END\nTEXT "\\xc3\\xa9"\nSTART "e"\nEND "e"\nEND "d"\nOK\n',
)
EXPAT_ERROR_CANARY = (b"<a>\n<b></a>", b"ERROR 7 line 2 col 5\n")


class SetupError(Exception):
    pass


def log(msg):
    print("preflight: " + msg, file=sys.stderr, flush=True)


def run(cmd, env=None, timeout=900):
    p = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env, timeout=timeout)
    if p.returncode != 0:
        raise SetupError("%s failed (exit %d): %s" % (os.path.basename(cmd[0]), p.returncode,
                                                     p.stderr.decode("utf-8", "replace")[-1500:]))
    return p.stdout.decode("utf-8", "replace")


def verify(c_cmd, other_cmd, inputs, timeout, jobs, out):
    cmd = [VERIFY, "--c", c_cmd, "--rust", other_cmd, "--inputs", inputs, "--timeout", str(timeout), "--out", out]
    if jobs:
        cmd += ["--jobs", str(jobs)]
    return json.loads(run(cmd))


def canary(driver, cases):
    for given, want in cases:
        p = subprocess.run([driver], input=given, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=60)
        if p.stdout != want:
            return {"ok": False, "input": given.decode("latin-1")[:80],
                    "expected": want.decode("latin-1")[:200], "got": p.stdout.decode("latin-1")[:200]}
    return {"ok": True, "cases": len(cases)}


def stage_corpus(src, dest):
    """Copy a committed corpus (minus README/notes) into the input dir."""
    n = 0
    for name in sorted(os.listdir(src)):
        path = os.path.join(src, name)
        if os.path.isfile(path) and not name.startswith((".", "README")) and name != "REASONS.md":
            shutil.copyfile(path, os.path.join(dest, "gap_" + name))
            n += 1
    return n


def build_stub(script, stub_src, build_id):
    p = subprocess.run([script, stub_src, build_id], stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=900)
    try:
        res = json.loads(p.stdout.decode().strip().splitlines()[-1])
    except (ValueError, IndexError):
        raise SetupError("stub build printed no JSON: %s" % p.stderr.decode("utf-8", "replace")[-800:])
    if not res.get("ok"):
        raise SetupError("stub build failed: %s" % str(res.get("errors"))[:800])
    return res["binary"]


def rate_check(report, ok):
    s = report.get("summary") or {}
    return {"ok": ok, "match_rate": report.get("match_rate"), "scored": s.get("scored"),
            "matched": s.get("matched"), "infra_failures": s.get("infra_failures")}


def preflight(args, work):
    checks = {}
    inputs = os.path.join(work, "inputs")
    stamp = os.path.basename(work)
    if args.target == "cjson":
        run([os.path.join(HERE, "fetch_cjson.sh")])
        driver = os.path.join(work, "cjson_driver")
        run([os.path.join(HERE, "build_cjson.sh"), driver])
        cases = [CJSON_CANARY]
        run([sys.executable, "-I", os.path.join(HERE, "gen_inputs.py"), "--seed", str(args.seed),
             "--count", str(args.count), "--out", inputs])
        extra = stage_corpus(os.path.join(HERE, "adversarial", "corpus", "mutant_gaps"), inputs)
        build_script = os.path.join(ROOT, "scripts", "build_rust.sh")
        stub_src = os.path.join(HERE, "rust_crate", "src", "cjson.rs")
        timeout = args.timeout or 5
    else:
        e = os.path.join(HERE, "expat")
        run([os.path.join(e, "fetch_expat.sh")])
        run([os.path.join(e, "build_expat.sh")])
        driver = os.path.join(e, "build", "expat_driver")
        cases = [EXPAT_CANARY, EXPAT_ERROR_CANARY]
        run([sys.executable, "-I", os.path.join(e, "gen_inputs_expat.py"), "--seed", str(args.seed),
             "--count", str(args.count), "--out", inputs])
        extra = 0
        build_script = os.path.join(ROOT, "scripts", "build_rust_expat.sh")
        stub_src = os.path.join(e, "rust_crate", "src", "expat.rs")
        timeout = args.timeout or 20
    n_inputs = len([f for f in os.listdir(inputs) if not f.startswith(".")])

    t = time.monotonic()
    checks["canary"] = canary(driver, cases)
    log("canary %s" % ("ok" if checks["canary"]["ok"] else "FAILED: %s" % checks["canary"]))

    cc = verify(driver, driver, inputs, timeout, args.jobs, os.path.join(work, "c_vs_c.json"))
    s = cc.get("summary") or {}
    checks["c_vs_c"] = rate_check(cc, cc.get("match_rate") == 1.0 and cc["counts"].get("divergence") == 0
                                  and (s.get("scored") or 0) > 0 and not s.get("infra_failures"))
    log("C vs C match_rate %s on %s scored" % (cc.get("match_rate"), s.get("scored")))

    stub_bin = build_stub(build_script, stub_src, "preflight-stub-%s" % stamp)
    st = verify(driver, stub_bin, inputs, timeout, args.jobs, os.path.join(work, "c_vs_stub.json"))
    stub_ok = st.get("match_rate") is not None and st["match_rate"] <= args.stub_max
    checks["stub"] = dict(rate_check(st, stub_ok), max=args.stub_max)
    log("C vs stub match_rate %s (must be <= %s)" % (st.get("match_rate"), args.stub_max))

    if args.target == "cjson" and args.mutants:
        mt = time.monotonic()
        p = subprocess.run([sys.executable, "-I", os.path.join(HERE, "mutants.py"), "--inputs", inputs,
                            "--timeout", str(timeout)] + (["--jobs", str(args.jobs)] if args.jobs else []),
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=1800)
        if p.returncode not in (0, 1):
            raise SetupError("mutants.py failed: %s" % p.stderr.decode("utf-8", "replace")[-800:])
        m = json.loads(p.stdout)
        with open(os.path.join(work, "mutants.json"), "w") as fh:
            json.dump(m, fh, indent=2)
        checks["mutants"] = {"ok": m["kill_rate"] >= args.min_kill_rate, "kill_rate": m["kill_rate"],
                             "killed": m["killed"], "mutants": m["mutants"], "min": args.min_kill_rate,
                             "survived": [x["id"] for x in m["survived"]],
                             "seconds": round(time.monotonic() - mt, 1)}
        log("mutants killed %d/%d" % (m["killed"], m["mutants"]))
    else:
        checks["mutants"] = {"ok": True, "skipped": True,
                             "reason": "not requested (--mutants)" if args.target == "cjson" else "no mutants for expat"}
    return checks, n_inputs, extra


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("target", choices=["cjson", "expat"])
    ap.add_argument("--count", type=int, default=200, help="generated inputs (default 200)")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--timeout", type=float, help="seconds per run (default 5 cjson, 20 expat)")
    ap.add_argument("--jobs", type=int)
    ap.add_argument("--stub-max", type=float, default=0.05, help="max match_rate for the reject-everything stub")
    ap.add_argument("--mutants", action="store_true",
                    help="also run the cJSON mutant kill check (slow: 75 to 400 s, builds 15 ASan drivers)")
    ap.add_argument("--min-kill-rate", type=float, default=0.9,
                    help="with --mutants (200 inputs + mutant_gaps kill 14/15; lone_low_surrogate_range survives)")
    args = ap.parse_args(argv)

    started = time.monotonic()
    work = os.path.join(ROOT, "runs", "preflight", "%s-%s-%d" % (args.target, time.strftime("%Y%m%d-%H%M%S"),
                                                                 os.getpid()))
    os.makedirs(work)
    out = {"preflight": args.target, "ok": False, "seed": args.seed, "generated_inputs": args.count}
    code = 1
    try:
        checks, n_inputs, extra = preflight(args, work)
        out.update(ok=all(c["ok"] for c in checks.values()), inputs=n_inputs, extra_corpus_inputs=extra,
                   checks=checks, failed=[k for k, c in checks.items() if not c["ok"]])
        code = 0 if out["ok"] else 1
    except (SetupError, subprocess.TimeoutExpired, OSError, ValueError) as exc:
        out.update(error="%s: %s" % (type(exc).__name__, exc))
        code = 2
    out.update(seconds=round(time.monotonic() - started, 1), work_dir=os.path.relpath(work, ROOT))
    with open(os.path.join(work, "preflight.json"), "w") as fh:
        json.dump(out, fh, indent=2)
    if not out["ok"]:
        log("FAILED: %s" % (out.get("error") or ", ".join(out.get("failed", []))))
    print(json.dumps(out, separators=(",", ":")), flush=True)
    return code


if __name__ == "__main__":
    sys.exit(main())
