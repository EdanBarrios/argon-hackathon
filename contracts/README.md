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
| `GET` | `/jobs` | Jobs this server has run, newest first (in memory; for "Load pre-run") | |
| `GET` | `/jobs/{id}/cost` | Token usage and USD per model for the job | |
| `POST` | `/try` | One input run through C, the current Rust binary, and old C for CVE inputs (`library`: `cjson` \| `expat`) | |
| `GET` | `/try/presets?library=expat` | Example inputs for the /try box | |
| `GET` | `/results/libexpat` | Recorded libexpat numbers, read from committed result files | |
| `GET` | `/remote-runs` | Managed-agent sandbox runs: sandbox claim vs host re-verify vs held-out (`?include_stubs=true` adds dev stubs) | |

`POST /jobs` body: `{"library": "cjson", "modules": ["lexer"]}` (`modules` optional; omit for the whole library).
`POST /jobs/{id}/approve` body: `{"approved_by": "<name>"}`.

### `POST /jobs` real-mode fields (`THESEUS_MODE=real`; the mock replay ignores them)

All optional.

| Field | Default | Meaning |
|---|---|---|
| `library` | `"cjson"` | Only `cjson` in real mode. |
| `model` | `"gemini-4-argon"` | Model for translate and repair calls. |
| `thinking` | `"medium"` | `low` \| `medium` \| `high`. |
| `rounds` | `4` | Model-call budget (a fresh translate counts as one). |
| `inputs` | `300` | Generated inputs per verify, cached under `runs/ai/inputs-c<N>-s1`. |
| `final_full` | `false` | Also verify the best build on 2000 inputs at the end. |
| `start_from` | `"fresh"` | `fresh` \| `argon-1` \| `argon-2` \| `flash-2` \| path to a `cjson.rs`. A saved candidate skips the translate call; its `translate` event has `phase: "loaded"`, `start_from`, `source_label` and a `summary` such as "Starting from Flash's first attempt ...". Unknown values return 400. |
| `inputs_dir` | `null` | Verify against this directory instead of generated inputs. |
| `extra_inputs` | `[]` | Adversarial corpora added to the verify set (and excluded from the held-out set): `targeted`, `argon`, `human`, or `<corpus>:<prefix>` such as `targeted:str_` (278 string inputs). Unknown names return 400. |
| `holdout` | `true` | Verify the best build once more on unseen inputs (seed 1001, overlap with the loop set removed). |
| `cve_check` | `true` | Run the CVE-2023-53154 input on old cJSON v1.7.17 and the final Rust; a real ASan crash becomes a `crash` event and a `result.crashes` entry. |

Real-mode events carry extra `data` keys the UI may show: `summary` (one human sentence), `phase` (`started` \| `done` \| `loaded` \| `failed` on translate/repair), `seconds`, `compile_ok` and `errors` on a failed build's `verify` event (no `match_rate`), `holdout: true` or `final_full: true` on extra `verify` events, `inputs_found`, `scored`, `skipped`, `infra_failures` on `verify` (from verify.py's `summary`), `pattern` and `function_source` on `divergence` (the function is a guess from the divergence pattern). The final `job_status: completed` event also has `timings` (seconds per stage) and `holdout`. There is one agent, `agent_cjson` (module `cjson parse+print`). `POST /jobs/{id}/approve` in real mode runs the managed-agent release step (`backend/api/pr.py` -> `harness/remote/open_pr.py`) synchronously, so it can take a few minutes. `approval` gains `pr_status` (`opened` \| `dry_run`), `pr_branch` and `pr_detail`; the `approved` event also carries `pr_files`. Without `GITHUB_TOKEN` (or with `THESEUS_PR_DRY_RUN=1`) it is a local dry run: `pr_status: "dry_run"`, `pr_url: null`, and `pr_detail` says what would be pushed. `pr_url` is only ever a real PR. 409 if the best build is below 1.0, 502 if the agent fails.

### `GET /jobs/{id}/cost`

`{"job_id", "models": [{"model", "input_tokens", "cached_tokens", "output_tokens", "thought_tokens", "usd"}], "total_usd"}`. Cached input is billed at 5%, thought tokens at the output rate (see `backend/api/cost.py`). Empty `models` when no model call was made (e.g. `start_from` a saved candidate that already matches).

### `GET /remote-runs`

Newest first, one row per `runs/remote/<stamp>/<library>-<model>/result.json`:
`run_id`, `library`, `model`, `status`, `seconds`, `cost_usd`, `rounds_used`, `claimed_rounds` (`[{round, build_ok, match_rate}]`),
`sandbox_match_rate` + `sandbox_behaviors` (the agent's own verify.sh claim), `host_match_rate` + `host_behaviors`
(host rebuild, same seed), `holdout_match_rate` + `holdout_behaviors` + `holdout_seed` (host, unseen inputs),
`match_rates_agree` (claim == host), `interaction_id`, `environment_id`, `error`. The UI line is
"sandbox claims {sandbox_match_rate} -> host re-verified {host_match_rate}" with a check when `match_rates_agree`.

### `POST /try`

Body: `{"library": "cjson", "input": "<text>"}` or `{"library": "cjson", "input_b64": "<base64>"}`, plus optional `"compare_old": true`. Exactly one of `input` / `input_b64`. Max 64 KB (413 above). 400 for a bad body, 503 if no binary is available.

Runs the bytes on stdin through the cJSON v1.7.19 C driver (ASan) and the current Rust binary (`THESEUS_RUST_BIN`, else the last real job that finished at 100%, else `runs/ai/argon-1`), plus the old cJSON v1.7.17 driver when `compare_old` is true or the input equals a known CVE file in `backend/harness/oldcve/inputs/`. Uses `verify.py`'s `run_one` and `classify` with `LC_ALL=C` and a 20 s timeout.

Response:

```json
{
  "library": "cjson", "input_bytes": 7, "input": "{\"1\":1,", "input_hex": null, "cve": "CVE-2023-53154",
  "c":     {"label", "version", "binary", "stdout", "stdout_hex", "exit_code", "signal", "timed_out",
            "crashed", "exit", "sanitizer_summary", "stderr_head", "seconds"},
  "rust":  { same shape },
  "old_c": { same shape } | null,
  "verdict": "match",                 // C v1.7.19 vs Rust
  "old_verdict": "safety_improvement" // old C vs Rust, null when old C did not run
}
```

`first_difference` (both libraries): `null` when C and Rust stdout are identical, else `{"offset", "line", "c_bytes", "rust_bytes"}` (first differing byte offset, its 1-based line in C's stdout, and each side's stdout length).

#### `POST /try` with `library: "expat"`

Body: `{"library": "expat", "input": "<xml>"}` (or `input_b64`). Same limits (64 KiB, 413 above), same response shape, with `library: "expat"`, `cve: null`, `old_c: null`, `old_verdict: null` (`compare_old: true` is a 400). C side: libexpat 2.9.0 driver under ASan+UBSan (`c.version` = `"libexpat 2.9.0 (R_2_9_0)"`). Rust side: Argon's port (`argon-fresh-a/round_3/expat.rs`), binary from `THESEUS_EXPAT_RUST_BIN`, else `runs/build-expat/edan-reverify-a/`, else built on first use. `stdout` is the driver's event lines (`START "a" "x"="1"`, `TEXT "..."`, ..., then `OK`) or a single `ERROR <code> line <L> col <C>`. Timeout 20 s per side; billion laughs takes about 1 s for C under ASan.

#### `GET /try/presets?library=expat`

`[{"id", "title", "description", "input"}]`. For `expat`: `rich` (XML decl, internal DTD with an entity and ATTLIST default, attributes, comment, PI, CDATA, entity reference; matches event for event, 26 lines) and `billion_laughs` (9 levels; both sides print `ERROR 43 line 14 col 6`). `library=cjson` returns `[]`; anything else is 400. Send `input` as-is to `POST /try`.

#### `GET /results/libexpat`

```json
{
  "library": "expat", "c_version": "libexpat 2.9.0 (R_2_9_0)",
  "c_lines": 18494, "c_files": 36, "c_lines_source": "wc -l of .../lib/*.c *.h (vendored R_2_9_0)",
  "rust_lines": 6555, "rust_lines_translate": 6554, "model": "gemini-4-argon",
  "translate_seconds": 1089.4, "translate_output_tokens": 73653,
  "compile_fix_rounds": 2, "repair_seconds": 1170.7,
  "lines_changed_after_translate": {"added": 3, "removed": 2, "hunks": 2},
  "loop":          {"matched": 747,  "tested": 747,  "match_rate": 1.0, "seed": 1},
  "holdout":       {"matched": 1253, "tested": 1253, "match_rate": 1.0, "seed": 1001},
  "host_reverify": {"matched": 2000, "tested": 2000, "match_rate": 1.0, "seed": 4242},
  "flash_result": {"model": "gemini-3.8-flash", "status": "failed", "file": null, "reason": "<error text>"},
  "sources": {"translate", "repair_and_holdout", "loop_verify", "holdout_verify", "diff",
              "host_reverify", "host_reverify_readme", "flash", "rust_source"}   // repo-relative paths
}
```

Values shown are what the committed files hold today; every one is read from `sources` at request time. `c_lines_source` falls back to `backend/harness/expat/README.md` when the vendored source is not fetched. 503 if a file is unreadable. Say "N tested behaviors preserved", never "equivalent".

`verdict` / `old_verdict` are verify.py classes: `match`, `divergence`, `safety_improvement`, `rust_crash`, `both_crash`, `timeout`. `stdout` is raw (invalid UTF-8 as `\x` escapes); `stdout_hex` is set only when stdout is not printable.

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
