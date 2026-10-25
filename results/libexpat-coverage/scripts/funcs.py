"""Per-function coverage from `llvm-cov export` JSON.

For each function: file, name, executed?, source line span, lines covered
(line-level via segments is expensive; we use region-based line spans).
Prints uncovered functions sorted by span, per file.
Usage: python3 -I funcs.py full_export.json out.json
"""
import json
import sys
import os

data = json.load(open(sys.argv[1]))["data"][0]
rows = []
for f in data["functions"]:
    fname = os.path.basename(f["filenames"][0])
    regs = f["regions"]
    # region: [lineStart, colStart, lineEnd, colEnd, count, fileId, expandedFileId, kind]
    main = [r for r in regs if r[5] == 0]
    if not main:
        continue
    start = min(r[0] for r in main)
    end = max(r[2] for r in main)
    covered_regions = sum(1 for r in main if r[4] > 0 and r[7] == 0)
    total_regions = sum(1 for r in main if r[7] == 0)
    name = f["name"].split(":")[-1]
    rows.append({
        "file": fname, "name": name, "count": f["count"],
        "start": start, "end": end, "span": end - start + 1,
        "regions_covered": covered_regions, "regions_total": total_regions,
    })
json.dump(rows, open(sys.argv[2], "w"), indent=1)
for file in sorted({r["file"] for r in rows}):
    rs = [r for r in rows if r["file"] == file]
    un = sorted([r for r in rs if r["count"] == 0], key=lambda r: -r["span"])
    print(f"== {file}: {len(rs)} fns, {len(un)} never called")
    for r in un:
        print(f"   {r['span']:5d} lines  {r['name']}  (L{r['start']})")
    part = sorted([r for r in rs if r["count"] > 0 and r["regions_total"]],
                  key=lambda r: -(r["regions_total"] - r["regions_covered"]))[:12]
    print("   most-missed regions in called fns:")
    for r in part:
        print(f"   {r['regions_total']-r['regions_covered']:5d}/{r['regions_total']} regions missed  {r['name']} (calls {r['count']})")
