# cJSON live repair runs (Flash's first port, repaired by Argon)

`result.json` and per-round `verify.json` for the 8 live runs of the UI pipeline on Oct 25 2026, copied from the gitignored `runs/ai/`. Each run starts from Flash's first port, verifies it, sends divergences to Argon and verifies again.

| Run | Inputs | Round 1 divergences | Round 2 divergences | Stage time (sum of timings) |
|---|---|---|---|---|
| job_3a01c4, job_262b76 | 300 | 0 | n/a | 61 s, 92 s |
| job_2210d0, job_a130a1 | 514 | 19 | 0 | 168 s, 166 s |
| job_2fc7b6 | 578 | 19 | 0 | 145 s |
| job_be9796 | 578 | 19 | 0 | 173 s |
| job_532323, job_29a12b | 578 | 19 | 0 | 318 s, 412 s |

The first two runs used a port that already matched on the 300 plain inputs; the later runs add the 278 targeted string inputs (`targeted:str_`), which is where the 19 divergences come from. The last two runs were slower because the machine was under heavy load.
