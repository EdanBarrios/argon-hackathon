"""Sum the per-worker logs of diff_fuzz.py. Usage: python3 -I summarize_fuzz.py RUN_DIR OUT_JSON"""
import json
import os
import sys

run = sys.argv[1]
workers = []
for name in sorted(os.listdir(run)):
    if not name.startswith("log_w"):
        continue
    start = end = None
    for line in open(os.path.join(run, name)):
        rec = json.loads(line)
        if rec.get("event") == "start":
            start = rec
        elif rec.get("event") == "end":
            end = rec
    workers.append({"worker": name[4:-6], "start": start, "end": end})
counts = {}
for w in workers:
    for k, v in (w["end"] or {}).get("counts", {}).items():
        counts[k] = counts.get(k, 0) + v
findings = sorted(os.listdir(os.path.join(run, "findings")))
out = {
    "workers": len(workers),
    "execs_total": sum((w["end"] or {}).get("execs", 0) for w in workers),
    "new_inputs_total": sum((w["end"] or {}).get("new_inputs", 0) for w in workers),
    "guards": workers[0]["start"]["guards"] if workers else None,
    "seed_edges": [w["start"]["seed_edges"] for w in workers],
    "final_edges_per_worker": [(w["end"] or {}).get("edges") for w in workers],
    "classes": counts,
    "findings": [f for f in findings if f.endswith(".xml")],
}
json.dump(out, open(sys.argv[2], "w"), indent=1)
print(json.dumps(out, indent=1))
