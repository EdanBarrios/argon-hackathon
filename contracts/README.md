# API contract

**The fixtures in `contracts/fixtures/` are the source of truth.** Contract changes are announced first, and C edits. Never change the contract silently.

Base URL: `NEXT_PUBLIC_API_URL` (default `http://localhost:8000`). All bodies are JSON.

## Endpoints

| Method | Path | Returns | Fixture |
|---|---|---|---|
| `POST` | `/jobs` | New job, status `queued` | |
| `GET` | `/jobs/{id}` | Job with live `agents` | `job-running.json`, `job-failed.json` |
| `GET` | `/jobs/{id}/events` | Ordered event list (supports `?after=<seq>`) | `job-events.json` |
| `GET` | `/jobs/{id}/result` | Job with `result` and `approval` filled in | `job-completed.json` |
| `POST` | `/jobs/{id}/approve` | Job with `approval.approved_by` and `approval.pr_url` set | |

`POST /jobs` body: `{"library": "cjson", "modules": ["lexer"]}` (`modules` optional; omit for the whole library).
`POST /jobs/{id}/approve` body: `{"approved_by": "<name>"}`.

The UI polls `GET /jobs/{id}` and `GET /jobs/{id}/events`. The backend never holds a request open waiting on Argon.

## Lifecycle

```
queued → running → verifying ⇄ running → completed | failed | cancelled
```

`verifying ⇄ running` repeats once per repair round. Each agent has its own `status` with the same values.

## Shapes

- **Job:** `id`, `library`, `status`, `created_at`, `updated_at`, `runner` (`local` | `remote`), `agents`, `result` (null until completed), `approval`, `error` (null unless failed)
- **Agent:** `{id, module, status, iteration, match_rate, interaction_id}`
- **Result:** `baseline_match_rate`, `final_match_rate`, `behaviors_tested`, `divergences: [{input, c_output, rust_output, function, resolved}]`, `crashes: [{input, sanitizer_report, rust_behavior}]`
- **Approval:** `{required, approved_by, pr_url}`
- **Error:** `{code, message, agent_id, retryable}`
- **Event:** `{seq, at, job_id, agent_id, type, data}`. `type` is one of `job_status`, `translate`, `verify`, `divergence`, `crash`, `repair`, `approval_required`, `approved`.

Match rates are floats in `[0, 1]`. Times are ISO 8601 with offset.
