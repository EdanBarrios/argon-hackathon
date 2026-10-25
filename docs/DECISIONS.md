# Decisions

Anyone may add rows. Newest at the bottom.

| Time (PT) | Decision | Reason | Owner |
|---|---|---|---|
| 10:13 | Hacking start is ~10:13 AM, not 10:30. Repo setup began after this. | Organizers announced hacking had started early (~10:13 AM PT). | Edan (C) |
| 10:15 | Project = Theseus (C→Rust migration with behavioral evidence) | Day-of plan, docs/PLAN.md section 2 | Team |
| 10:15 | Track 1 (Million Token Engineering) plus the Managed Agents prize | Day-of plan, docs/PLAN.md section 2 | Team |
| pending (kickoff) | Target library (cJSON vs inih) | Requirements in docs/PLAN.md section 2 | Team |
| pending (11:15) | Sandbox (remote) vs local runner | Depends on A's smoke test | A + C |
| pending | Backend host | | C |
| 10:31 | Repair loops use thinking_level medium, lead planner high | Organizer guide; speed for live demo | C |
| 10:44 | Target: cJSON v1.7.19 core, parse+print first; fallback inih if not 100% by 12:30 | Security history, clean 3-module split shows whole-library planning; inih is one function | Team |
| 11:13 | Proposed, pending Edan: 11:15 runner = **local**. Managed agents only for the final verification and PR step, unless a C toolchain can be allowlisted in. | Smoke test (`scripts/sandbox_smoke.py`) took 145 s, under the 3 min bar, but the sandbox has no clang and no `cc`, so ASan builds and even `rustc` linking fail. apt mirrors return 403 through the allowlist proxy. rustup install took 12 s, git clone of the public repo and `fetch_cjson.sh` worked, and files plus rustup persisted to a follow-up turn when the same `environment_id` is reused. | A + C |
