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

## API modes

`THESEUS_MODE` selects the pipeline behind `/jobs`:

- `mock` (default): `POST /jobs` replays `contracts/fixtures/job-events.json` over about 30 s and ends equal to `job-completed.json`. Add `?scenario=failed` to end in the `job-failed.json` shape. `THESEUS_MOCK_DURATION=<seconds>` changes the replay length.
- `real`: `RealPipeline` in `api/pipeline.py`. Not wired yet, so jobs fail with `not_implemented`.

```
THESEUS_MODE=mock uv run uvicorn api.main:app --reload --port 8000
```

`GET /jobs/{id}/cost` (proposed, not yet in the contract) returns per-model token totals and USD.

## Tests

```
uv run pytest -q
```
