"""Tally what never-executed lines inside big called functions refer to.
Usage: python3 -I zero_tally.py show_xmlparse.c.txt"""
import collections
import re
import sys

RANGES = {"doProlog": (5486, 6607), "doContent": (3581, 4096), "storeAtts": (4136, 4551)}
PAT = re.compile(r"m_[a-zA-Z]+Handler|m_ns\w*|m_paramEntityParsing|m_useForeignDTD|XML_ERROR_[A-Z_]+|reportDefault|m_parsingStatus|XML_SUSPENDED|XML_FINISHED")
lines = {}
for ln in open(sys.argv[1], errors="replace"):
    m = re.match(r"^\s*(\d+)\|\s*([0-9.]+[kMG]?)?\|(.*)$", ln)
    if m and m.group(2) is not None:
        lines[int(m.group(1))] = (m.group(2), m.group(3))
for name, (a, b) in RANGES.items():
    z = [(n, t) for n, (c, t) in lines.items() if a <= n <= b and c == "0"]
    code = [n for n, (c, t) in lines.items() if a <= n <= b]
    tally = collections.Counter(x for _, t in z for x in PAT.findall(t))
    print(f"{name}: {len(z)}/{len(code)} code lines never executed; refs: {tally.most_common(14)}")
