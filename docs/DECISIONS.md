# Decisions

Anyone may add rows. Newest at the bottom.

| Time (PT) | Decision | Reason | Owner |
|---|---|---|---|
| 10:13 | Hacking start is ~10:13 AM, not 10:30. Repo setup began after this. | Organizers announced hacking had started early (~10:13 AM PT). | Edan (C) |
| 10:15 | Project = Theseus (C→Rust migration with behavioral evidence) | Day-of plan, docs/PLAN.md section 2 | Team |
| 10:15 | Track 1 (Million Token Engineering) plus the Managed Agents prize | Day-of plan, docs/PLAN.md section 2 | Team |
| pending | Backend host | | C |
| 10:31 | Repair loops use thinking_level medium, lead planner high | Organizer guide; speed for live demo | C |
| 10:44 | Target: cJSON v1.7.19 core, parse+print first; fallback inih if not 100% by 12:30 | Security history, clean 3-module split shows whole-library planning; inih is one function | Team |
| 11:13 | **Decided (Edan, 11:16):** runner = **local**. Trying an apt allowlist fix in the background; switch to remote if clang + ASan work there. Managed agents only for the final verification and PR step, unless a C toolchain can be allowlisted in. | Smoke test (`scripts/sandbox_smoke.py`) took 145 s, under the 3 min bar, but the sandbox has no clang and no `cc`, so ASan builds and even `rustc` linking fail. apt mirrors return 403 through the allowlist proxy. rustup install took 12 s, git clone of the public repo and `fetch_cjson.sh` worked, and files plus rustup persisted to a follow-up turn when the same `environment_id` is reused. | A + C |
| 11:18 | Demo crash moment uses cJSON v1.7.17 (CVE-2023-53154), real ASan report | Reproduced a genuine heap-buffer-over-read in the parse path; input `{"1":1,` crashes v1.7.17 under ASan while v1.7.19 returns PARSE_ERROR cleanly. Evidence in backend/harness/oldcve/. | C |
| 11:17 | Edan (C) builds frontend scaffold temporarily; Keon takes over on return | Keon blocked on Wi-Fi; keep 11:30 checkpoint | Edan |
| 11:22 | **Runner = hybrid (Edan).** Live demo runs locally. Pre-runs (parallel modules, full library, libexpat) and the PR step run in managed-agent sandboxes. Turn on sandboxes only after the local loop works. | apt allowlist fix works: clang 18 + ASan + Rust + harness all run in the sandbox (192 s first run, ~46 s setup per new sandbox). Too slow for the live 90 s budget, fine for background pre-runs, and keeps the Managed Agents prize in play. | Edan |
