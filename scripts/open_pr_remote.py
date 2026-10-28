"""Human-approved PR step: a managed agent pushes migrated/cjson/ and opens a PR.

Usage:
  cd backend && uv run python ../scripts/open_pr_remote.py runs/remote/<stamp>/cjson-gemini-4-argon --approved-by "Edan" --dry-run
  cd backend && uv run python ../scripts/open_pr_remote.py <job_dir> --approved-by "Edan"

The real run needs GITHUB_TOKEN in the repo .env: a fine-grained token on
EdanBarrios/theseus only, with Contents read/write and Pull requests
read/write. It is registered through the Gemini Credentials API and injected by
the sandbox's egress proxy; it never enters the prompt or the sandbox.
"""

from __future__ import annotations

import argparse
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "backend"))

from harness.remote.open_pr import PRError, open_pr_remote  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("job_dir", help="a run_remote job directory (holds result.json and cjson.rs)")
    ap.add_argument("--approved-by", required=True, help="the person who approved this PR")
    ap.add_argument("--dry-run", action="store_true", help="everything except git push and the pulls API call")
    ap.add_argument("--model", default="gemini-4-argon")
    ap.add_argument("--allow-partial", action="store_true", help="allow a local match_rate below 1.0")
    args = ap.parse_args()
    job = Path(args.job_dir)
    if not job.is_absolute():
        job = (ROOT / job) if (ROOT / job).exists() else job.resolve()
    try:
        url = open_pr_remote(job, args.approved_by, dry_run=args.dry_run, model=args.model,
                             allow_partial=args.allow_partial,
                             on_event=lambda m: print(f"{datetime.now():%H:%M:%S} {m}", flush=True))
    except PRError as exc:
        print(f"PR step refused or failed: {exc}", file=sys.stderr)
        return 1
    print(url)
    return 0


if __name__ == "__main__":
    sys.exit(main())
