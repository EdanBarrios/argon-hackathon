"""Combine llvm-cov summary export, per-function data and scope estimate into one JSON.
Usage: python3 -I summarize_cov.py COVDIR OUT_JSON
"""
import json
import os
import sys

D, OUT = sys.argv[1], sys.argv[2]
d = json.load(open(os.path.join(D, "summary_export.json")))["data"][0]
sc = json.load(open(os.path.join(D, "scope.json")))
fu = json.load(open(os.path.join(D, "funcs.json")))
KEYS = ("lines", "functions", "branches", "regions")
PARSER = ("xmlparse.c", "xmltok.c", "xmltok_impl.c", "xmltok_ns.c", "xmlrole.c")
files = {}
for f in d["files"]:
    n = os.path.basename(f["filename"])
    s = f["summary"]
    if n in PARSER:
        files[n] = {k: {"covered": s[k]["covered"], "count": s[k]["count"],
                        "pct": round(s[k]["percent"], 2)} for k in KEYS}


def combine(names):
    t = {k: {"covered": sum(files[n][k]["covered"] for n in names),
             "count": sum(files[n][k]["count"] for n in names)} for k in KEYS}
    for k in t:
        t[k]["pct"] = round(100 * t[k]["covered"] / t[k]["count"], 2)
    return t


hot = sorted([r for r in fu if r["file"] == "xmlparse.c" and r["count"] > 0],
             key=lambda r: -(r["regions_total"] - r["regions_covered"]))[:10]
out = {
    "what": "clang source-based coverage of libexpat R_2_9_0 (a76b1174) under drivers/expat_driver.c, seed 4242, 2000 inputs",
    "build": "clang -g -O0 -fprofile-instr-generate -fcoverage-mapping -std=c99; xmlparse.c xmlrole.c xmltok.c xcs.c "
             "random_dev_urandom.c random_arc4random_buf.c + drivers/expat_driver.c; config/expat_config.h",
    "toolchain": "Apple clang / llvm-cov 17.0.0 (xcrun)",
    "inputs": {"seed": 4242, "count": 2000, "runs": 2000, "nonzero_exits": 0, "profraw_files": 2000},
    "raw_llvm_cov_report": {"files": files, "xmltok_combined": combine(PARSER[1:4]),
                            "parser_total": combine(PARSER)},
    "in_scope_estimate_lines": sc,
    "xmlparse_called_functions_most_missed_regions": [
        {"name": r["name"], "regions_missed": r["regions_total"] - r["regions_covered"],
         "regions_total": r["regions_total"], "calls": r["count"]} for r in hot],
}
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(out["raw_llvm_cov_report"]["xmltok_combined"]))
print(json.dumps(out["raw_llvm_cov_report"]["parser_total"]))
for h in out["xmlparse_called_functions_most_missed_regions"]:
    print(h)
