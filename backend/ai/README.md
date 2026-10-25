# backend/ai

Owner: **A (ai)**. Lead Argon planner, managed-agent prompts, repair prompts. See docs/PLAN.md sections 3, 4, and 6.

| File | What |
|---|---|
| `prompts.py` | All prompt text: system instruction, translate, repair, sandbox task. Argon and Flash get identical words |
| `translate.py` | `translate()` / `repair()` through the Interactions API (background + poll) and feedback trimming |
| `loop.py` | Local translate → build → verify → repair loop, then a held-out check |
| `sandbox_run.py` | The same loop run by a managed agent in a remote sandbox, re-verified on the host |

```bash
cd backend
uv run python -m ai.loop --model gemini-4-argon --rounds 5          # local (live demo)
uv run python -m ai.sandbox_run --model gemini-4-argon --rounds 5   # managed-agent pre-run
uv run python -m ai.sandbox_run --dry-run                           # show the request, no API call
uv run pytest -q ai/tests                                           # offline tests
```

**Which number to quote.** `summary.json` → `holdout.match_rate` (local loop) or
`host_holdout.match_rate` (sandbox). Those inputs come from `--seed + 1000`, with every
input the loop saw removed, and the model never sees their results. The per-round rates
are on the inputs that repair feedback came from. In a sandbox run they are the agent's
own claims; `claim_holds` says whether the host reproduced the final one.
