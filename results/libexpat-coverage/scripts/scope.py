"""Raw and in-scope line/function coverage for libexpat from llvm-cov output.

Usage: python3 -I scope.py COVDIR OUT_JSON
Reads COVDIR/show_<file>.txt (llvm-cov show text), COVDIR/funcs.json
(per-function spans from funcs.py) and COVDIR/summary_export.json.
"""
import json
import os
import re
import sys

D = sys.argv[1]
FILES = ["xmlparse.c", "xmltok.c", "xmltok_impl.c", "xmltok_ns.c", "xmlrole.c"]

DEBUG = ["unsignedCharToPrintable", "accountingReportDiff", "expat_heap_stat",
         "testingAccountingGetCountBytesDirect", "testingAccountingGetCountBytesIndirect"]
NS = ["addBinding", "is_rfc3986_uri_char"]
EXTERNAL = ["XML_ExternalEntityParserCreate", "dtdCopy", "copyEntityTable", "getContext",
            "setContext", "externalEntityInitProcessor", "externalEntityInitProcessor2",
            "externalEntityInitProcessor3", "externalEntityContentProcessor",
            "externalParEntInitProcessor", "externalParEntProcessor",
            "entityValueInitProcessor", "entityValueProcessor", "doIgnoreSection",
            "ignoreSectionProcessor", "poolCopyStringN", "poolCopyStringNoFinish"]
INCREMENTAL = ["dtdReset", "hashTableClear", "moveToFreeBindingList", "moveEntityList",
               "cdataSectionProcessor", "errorProcessor"]
UNSET_HANDLER = ["reportDefault", "build_model", "nextScaffoldPart",
                 "callUnknownEncodingConvert", "callUnknownEncodingRelease", "copyString",
                 "generate_hash_secret_salt", "xmlSetHashSalt",
                 "setAllocTrackerActivationThreshold", "setAllocTrackerMaximumAmplification",
                 "setBillionLaughsAttackProtectionActivationThreshold",
                 "setBillionLaughsAttackProtectionMaximumAmplification",
                 "setReparseDeferralEnabled"]
EXCLUDE = {"xmlparse.c": {}}
for cat, names in [("debug/test-only accounting", DEBUG), ("namespaces (XML_ParserCreate(NULL))", NS),
                   ("external entities / external subset / conditional sections", EXTERNAL),
                   ("suspend, reset, re-parse after error, or multi-call parsing", INCREMENTAL),
                   ("handler or option the driver never sets (default handler, element-decl "
                    "content model, unknown-encoding handler, protocol encoding, entropy salt)",
                    UNSET_HANDLER)]:
    for n in names:
        EXCLUDE["xmlparse.c"][n] = cat
EXCLUDE["xmltok.c"] = {n: "unknown-encoding handler not set" for n in
                       ["XmlInitUnknownEncoding", "unknown_toUtf8", "unknown_toUtf16",
                        "unknown_isName", "unknown_isNmstrt", "unknown_isInvalid",
                        "XmlSizeOfUnknownEncoding", "XmlInitUnknownEncodingNS"]}
for n in ["utf8_toUtf16", "little2_toUtf16", "big2_toUtf16", "latin1_toUtf16", "XmlUtf16Encode"]:
    EXCLUDE["xmltok.c"][n] = "UTF-16 XML_Char output (XML_UNICODE builds only)"
EXCLUDE["xmlrole.c"] = {n: "external subset / conditional sections" for n in
                        ["externalSubset0", "externalSubset1", "condSect0", "condSect1",
                         "condSect2", "XmlPrologStateInitExternalEntity"]}
# xmltok_impl.c is a template included 3x; lines are merged across instantiations,
# so only exclude ignoreSectionTok (all three instantiations unreachable).
EXCLUDE["xmltok_impl.c"] = {"normal_ignoreSectionTok": "conditional sections (external subset only)"}

funcs = json.load(open(os.path.join(D, "funcs.json")))
LINE = re.compile(r"^\s*(\d+)\|\s*([0-9.]+[kMG]?)?\|")


def parse_count(s):
    if s is None:
        return None
    mult = {"k": 1e3, "M": 1e6, "G": 1e9}
    if s[-1] in mult:
        return float(s[:-1]) * mult[s[-1]]
    return float(s)


out = {"files": {}, "excluded_functions": []}
tot = {"raw_lines": 0, "raw_lines_hit": 0, "scope_lines": 0, "scope_lines_hit": 0}
for f in FILES:
    lines = {}
    for ln in open(os.path.join(D, f"show_{f}.txt"), errors="replace"):
        m = LINE.match(ln)
        if m and m.group(2) is not None:
            lines[int(m.group(1))] = parse_count(m.group(2))
    excl_lines = set()
    fr = [r for r in funcs if r["file"] == f]
    ex = EXCLUDE.get(f, {})
    pub_unused = []
    for r in fr:
        reason = ex.get(r["name"])
        if reason is None and f == "xmlparse.c" and r["name"].startswith("XML_") and r["count"] == 0:
            reason = "public API function the driver never calls"
            pub_unused.append(r["name"])
        if reason is None and f == "xmltok_ns.c" and r["count"] == 0:
            reason = "namespace-aware encoding init (XML_ParserCreate(NULL))"
        if reason is None and f == "xmltok_impl.c" and r["name"].endswith("_ignoreSectionTok"):
            reason = "conditional sections (external subset only)"
        if reason:
            if f != "xmltok_ns.c":  # NS and non-NS instantiations share source lines
                excl_lines.update(range(r["start"], r["end"] + 1))
            if r["count"] != 0:
                print("WARNING excluded function was executed:", f, r["name"], file=sys.stderr)
            out["excluded_functions"].append({"file": f, "name": r["name"], "reason": reason,
                                              "lines_span": r["span"]})
    raw = len(lines)
    raw_hit = sum(1 for c in lines.values() if c > 0)
    sc = {k: v for k, v in lines.items() if k not in excl_lines}
    sc_hit = sum(1 for c in sc.values() if c > 0)
    nf = len({r["name"] for r in fr})
    fn_hit = len({r["name"] for r in fr if r["count"] > 0})
    excl_names = {e["name"] for e in out["excluded_functions"] if e["file"] == f}
    sc_f = [r for r in fr if r["name"] not in excl_names]
    out["files"][f] = {
        "raw_lines": raw, "raw_lines_hit": raw_hit, "raw_line_pct": round(100 * raw_hit / raw, 2),
        "scope_lines": len(sc), "scope_lines_hit": sc_hit,
        "scope_line_pct": round(100 * sc_hit / len(sc), 2) if sc else None,
        "functions_instantiated": nf, "functions_instantiated_hit": fn_hit,
        "scope_functions": len(sc_f), "scope_functions_hit": sum(1 for r in sc_f if r["count"] > 0),
    }
    out["files"][f]["in_scope_never_called"] = sorted(r["name"] for r in sc_f if r["count"] == 0)
    for k in tot:
        tot[k] += out["files"][f][k]
tot["raw_line_pct"] = round(100 * tot["raw_lines_hit"] / tot["raw_lines"], 2)
tot["scope_line_pct"] = round(100 * tot["scope_lines_hit"] / tot["scope_lines"], 2)
out["total_parser_files"] = tot
json.dump(out, open(sys.argv[2], "w"), indent=1)
for f, v in out["files"].items():
    print(f, v)
print("TOTAL", tot)
