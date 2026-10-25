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
| 11:00 | Edan (C) builds A's translate/repair temporarily; A takes over on return | A offline (venue Wi-Fi) | Edan |
| 11:05 | Output extraction: prompt asks for exactly one fenced rust block; no block raises TranslateError, an unclosed fence is reported as truncated, several blocks take the longest and log a note | Deterministic, easy to explain; truncation is the likely failure on long decodes | Edan (for A) |
| 11:05 | Repair feedback: compile errors in full; divergences deduped by a coarse pattern (accept/reject kind, error offset side, print diff location), shortest input per pattern, top 10 by frequency; Rust crashes and timeouts up to 10; safety_improvement dropped | One example per bug, not 10 copies of the same bug; keeps the repair prompt small | Edan (for A) |
| 11:05 | Repair budget: --rounds 5 = 1 translate + 4 repairs, stop early at match_rate 1.0; loops verify on 300 generated inputs (seed 1), full 2000 is opt-in via --final-inputs | Harness README: 300 inputs for repair loops; report honestly at the budget | Edan (for A) |
| 11:05 | max_output_tokens 131072 with background=True for every call; translate thinking high, repair medium, step down automatically if a model rejects a level (probe: Flash and Argon both accept high) | Rust parse+print is ~20K tokens and thinking counts toward the cap | Edan (for A) |
| 11:36 | The match rate we quote is the **held-out** rate: after the loop, the last good build is verified on inputs from seed+1000 minus any input the loop saw (`summary.json` → `holdout`). The loop's own rate is the rate on inputs its repair feedback came from | Repair sees these divergences, so its rate can overfit them (DARPA TRACTOR uses held-out tests for the same reason). "Argon never grades its own homework" needs inputs Argon never saw | A |
| 11:36 | Sandbox pre-runs (`python -m ai.sandbox_run`): one antigravity agent runs translate → build → verify → repair inside the sandbox with the same prompts. Its match rates are claims; the host rebuilds the final cjson.rs and verifies it on the same seed (`claim_holds`) and on the held-out seed | The agent runs the verifier itself, so its report is not evidence until the host reproduces it | A |
