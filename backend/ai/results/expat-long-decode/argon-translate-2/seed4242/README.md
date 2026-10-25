# Translation 2: re-verify on seed 4242

This is the same check the systems side ran on translation 1 (`results/libexpat-host-reverify/`), at 15:05 PT on 2026-10-25.

- **Build:** `argon-translate-2/round_2/host_fix/expat.rs`, the loop's chosen build, sha256 `cd7aaf3b3e719ec9caffe6d72020af536c0e0ea742836f17ce038921aeeee8ca`. Built with `scripts/build_rust_expat.sh`, which includes the banned-API guard; the build passed.
- **Inputs:** `gen_inputs_expat.py --seed 4242 --count 2000`. That is 599 W3C conformance documents and 148 edge cases (the same in every seed, so the loop saw them) plus 1,253 seed-4242 mutations.
- **Result against libexpat R_2_9_0 (ASan driver):** **2,000/2,000 match**. 0 divergences, 0 crashes, 0 timeouts, 0 infra failures, 0 skipped (`verify-seed4242.json`).
- **What was compared:**
  - 341 inputs parse successfully and match event by event.
  - The other 1,659 end in a parse error and match on error code, line and column.
  - This split is the same as the translation-1 re-verify (341 / 1,659), which is consistent with an identical input set.
- **Change from Argon's one-pass translation:** 1 line, a compile fix (`../round1_to_round2.diff`).

Together with translation 1, two independent one-pass Argon translations of libexpat each pass 2,000/2,000 on seed 4242 after compile-only fixes. Say "N tested behaviors preserved", never "equivalent". Run on a local runner.
