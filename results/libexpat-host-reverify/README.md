# libexpat: host re-verify of Argon's port

Independent check by the systems side (C) of the port in `backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs` (sha256 prefix 418570361191d001), at 13:15 PT.

- Built with `scripts/build_rust_expat.sh`, including the banned-API guard: ok, no `unsafe`, no process/fs/net/env.
- Fresh inputs: `gen_inputs_expat.py --seed 4242 --count 2000`. Seed 4242 was never used by the repair loop (seed 1) or its held-out set (seed 1001). 599 W3C conformance documents and 148 edge cases are fixed across seeds; the 1,253 mutations are new.
- Result against libexpat R_2_9_0 under ASan + UBSan: **2,000/2,000 match**, 0 divergences, 0 crashes, 0 timeouts, 0 infra failures, 0 skipped (`verify-seed4242.json`).
- What was compared: 341 inputs parse successfully and match event for event (215 of them valid W3C documents, 70 edge cases with 230,474 events). The other 1,659 end in a parse error and match on the exact error code, line and column; events before an error are not compared.
- Argon's changes after its one-pass translation: 3 lines, both compile fixes (`argon-fresh-a/round1_to_round3.diff`).

Say: "N tested behaviors preserved, including the W3C conformance suite", never "equivalent".
