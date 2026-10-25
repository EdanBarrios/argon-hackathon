# backend/ai

Owner: **A (ai)**. Lead Argon planner, managed-agent prompts, repair prompts. See docs/PLAN.md sections 3, 4, and 6.

| File | What |
|---|---|
| `prompts.py` | All prompt text: system instruction, translate, repair, sandbox task. Argon and Flash get identical words |
| `translate.py` | `translate()` / `repair()` through the Interactions API (background + poll) and feedback trimming |
| `loop.py` | Local translate → build → verify → repair loop, then a held-out check |
| `sandbox_run.py` | The same loop run by a managed agent in a remote sandbox, re-verified on the host |
| `quality.py` | rustfmt and clippy counts for a generated file (default lints, panic lints, arithmetic), and the host's `clippy --fix` + rustfmt pass |
| `report.py` | One table of every run's `summary.json`: held-out, adversarial, fmt, clippy, panic lints, crashes, seconds, USD, grouped for repeats |

```bash
cd backend
uv run python -m ai.loop --model gemini-4-argon --rounds 5          # local (live demo)
uv run python -m ai.sandbox_run --model gemini-4-argon --rounds 5   # managed-agent pre-run
uv run python -m ai.sandbox_run --dry-run                           # show the request, no API call
uv run python -m ai.report                                          # every run in one table
uv run pytest -q ai/tests                                           # offline tests
```

**Which number to quote.** `summary.json` → `holdout.match_rate` (local loop) or
`host_holdout.match_rate` (sandbox). Those inputs come from `--seed + 1000`, with every
input the loop saw removed, and the model never sees their results. The per-round rates
are on the inputs that repair feedback came from. In a sandbox run they are the agent's
own claims; `claim_holds` says whether the host reproduced the final one.

For cJSON, `summary.json` → `adversarial` also has the best build's rate on the committed
targeted and Argon-written corpora (C's `adversarial/run.sh`). Generated inputs give 1.0
for every port so far; the targeted corpus is what separates them. Quote both, with n.

**Quality.** `summary.json` → `quality` is the held-out build's tool counts: `fmt_ok`
(rustfmt on the generated file only), `clippy_warnings` (default lints), `panic_lints`
(`indexing_slicing`, `unwrap_used`, `expect_used`, `panic`), and `arithmetic_side_effects`
(reported, not a target). `host_fixed` says the host's `clippy --fix` + rustfmt pass made
that file, after a rebuild and reverify. `quality_first_build` is the model's first file
that built, for comparison. `panics` counts Rust crashes of the held-out build on the loop,
held-out and adversarial inputs. Once the best build matches every loop input, up to
`--polish-rounds` (3 for cJSON) more rounds run: lint-only polish turns, and a normal repair
turn after any polish that broke behavior. The best build is still picked by match rate first,
so a polish that changes behavior is never the one held out.

