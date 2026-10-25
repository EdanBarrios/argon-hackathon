"""List each failing/erroring test in a run with the last line of its traceback.
usage: python3 failures.py runs/pyexpat/<label>"""
import re, sys
from pathlib import Path

for log in sorted(Path(sys.argv[1]).glob("test_*.log")):
    text = log.read_text()
    blocks = re.split(r"^={50,}\n", text, flags=re.M)[1:]
    for b in blocks:
        head = b.splitlines()[0]
        m = re.match(r"(FAIL|ERROR): (\S+) \((.+?)\)", head)
        if not m:
            continue
        body = b.split("-" * 70, 1)[-1].strip().splitlines()
        last = next((l for l in reversed(body) if l.strip() and not l.startswith("Ran ")
                     and not l.startswith(("FAILED", "OK"))), "")
        last = re.split(r"^-{50,}$", last)[0]
        print(f"{log.stem}\t{m[1]}\t{m[3].split('.')[-1]}.{m[2]}\t{last.strip()[:160]}")
