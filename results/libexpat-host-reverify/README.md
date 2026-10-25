# libexpat: host re-verify of Argon's port

Independent check by the systems side (C) of the port in `backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs` (sha256 prefix 418570361191d001), at 13:15 PT.

- Built with `scripts/build_rust_expat.sh`, including the banned-API guard: ok, no `unsafe`, no process/fs/net/env.
- Inputs: `gen_inputs_expat.py --seed 4242 --count 2000`. Not fully fresh: the 599 W3C conformance documents and 148 edge cases are the same in every seed, so they are the 747 inputs the repair loop already saw. Only the 1,253 mutations are new, and of those only 56 are well-formed (compared event by event); the rest end in an error and are compared on code, line and column.
- Result against libexpat R_2_9_0 under ASan + UBSan: **2,000/2,000 match**, 0 divergences, 0 crashes, 0 timeouts, 0 infra failures, 0 skipped (`verify-seed4242.json`).
- What was compared: 341 inputs parse successfully and match event for event (215 of them valid W3C documents, 70 edge cases with 230,474 events). The other 1,659 end in a parse error and match on the exact error code, line and column; events before an error are not compared.
- Argon's changes after its one-pass translation: 3 lines, both compile fixes (`argon-fresh-a/round1_to_round3.diff`).

Say: "N tested behaviors preserved, including the W3C conformance suite", never "equivalent".

Ran on a local runner, not in a managed sandbox. n = 1 per model.

## Known limitations

- Scope: libexpat 2.9.0 parser core under one configuration: `XML_ParserCreate(NULL)`, no namespaces, no external entities, one `XML_Parse` call, events before an error not compared (`backend/harness/expat/README.md`). Not the full public API, not incremental parsing, not a drop-in replacement.
- 1,659 of the 2,000 inputs are compared only on error code, line and column.
- Hash-flooding protection is untested: the hash salt is fixed for determinism, so the class of attack behind CVE-2012-0876 is not exercised.
- Possible training exposure: an experimental Rust `expat` crate exists. Mitigated only by the port having to match our own driver interface and exact error positions.
- n = 1: one Argon port, one new mutation seed.
- The banned-API guard is a text scan of the source, not AST analysis and not a sandbox.

