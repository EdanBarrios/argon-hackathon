# Lua round 1 + 2 hand compile fixes: informal smoke test

**Ours, not Argon's, and not the verifier.**

- **The fixes:** Argon's round-1 `lua.rs` stopped at 2 borrow-checker errors. We fixed both by hand (`hand_fix.diff`, 9 changed lines):
  - return early by index after the length check in `ci_mut`;
  - read `GETARG_k` into a local before calling `SETARG_k`.

  No other change. The fixed file's sha256 is in `fixed_lua_rs.sha256`.
- **The test:** 22 hand-written scripts (`scripts/`) cover:
  - printing, arithmetic and integer/float rules;
  - `string.format` and patterns, `pairs` order;
  - closures, deep recursion, metatables, coroutines;
  - error messages with variable names, syntax and runtime errors;
  - goto and `<close>`, math, coercion, utf8, varargs, `load`, stack overflow.
- **The result:** each script went through the C driver (Lua 5.4.7, ASan + UBSan) and the Rust release build. **22/22 byte-identical** (`results.txt`).
- **Fixed during the run:** one of our scripts first printed a table address, which `INTERFACE.md` rules out. We fixed the script, not the port.

This is a smoke test written by us, with n = 22. It is not a held-out set or a generated corpus. Verified numbers need the Lua harness and `ai.loop` (`backend/ai/lua/HARNESS_SPEC.md`); there, Argon makes its own repairs.
