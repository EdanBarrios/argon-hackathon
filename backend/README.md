# Backend

| Folder | Owner | What lives here |
|---|---|---|
| `api/` | C (systems) | FastAPI `/jobs` endpoints, see `contracts/README.md` |
| `ai/` | A (ai) | Lead planner, managed-agent prompts, repair prompts |
| `harness/` | C (systems) | `verify.sh` runner glue, input generator, ASan |

`pyproject.toml` is shared and owned by C.

Run from this folder:

```
uv sync
uv run uvicorn api.main:app --reload --port 8000
```
