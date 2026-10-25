# Lua 5.4.7: harness spec (systems) for the `lua` target

The AI side is ready: `ai.loop --target lua` exists (`backend/ai/targets.py` `LUA`, prompts in `backend/ai/prompts.py`). The loop runs once the harness files below exist. The paths are the ones `LUA` points to. Change a path in one place and tell A, so the `Target` can follow.

## Size

These were measured, not estimated:

| | libexpat (`expat/lib`) | Lua 5.4.7, files sent |
|---|---|---|
| C lines | 17,763 | 25,575 (52 files) |
| Bytes | 606,859 | 755,305 |
| Tokens (`count_tokens`, gemini-4-argon) | ~196K | **271,891** |

What gets sent is `src/` minus `lua.c luac.c ldump.c lundump.[ch] loadlib.c liolib.c loslib.c ldblib.c` (`prompts._LUA_SKIP`). The Rust output is expected to be 2–3x libexpat's 74K output tokens.

## Vendor

`backend/harness/lua/fetch_lua.sh` puts the official tarball into `backend/harness/lua/vendor/lua-5.4.7/`, which is gitignored like libexpat:
- URL: `https://www.lua.org/ftp/lua-5.4.7.tar.gz`
- sha256: `9fbf5e28ef86c69858f6d3d34eccc32e911c1a28b4120ff3e84aaa70cfbf1e30`
- `vendor_check` is `src/lvm.c`.

## C driver: `backend/harness/lua/drivers/lua_driver.c`, built to `backend/harness/lua/build/lua_driver`

`build_lua.sh` builds it with ASan, like libexpat's build. The driver must do the following, and the Rust `main.rs` mirrors it:

1. **Load the script.** Read all of stdin, then `luaL_loadbufferx(L, buf, len, "=input", "t")`, so the chunk is named `input` and only text chunks are accepted.
2. **Open the libraries in scope.** Only `_G`, `coroutine`, `table`, `string`, `math` and `utf8`, through `luaL_requiref`. Set `dofile`, `loadfile` and `require` to nil if `_G` has them; `load` stays, text chunks only.
3. **Fix every source of nondeterminism:**
   - Hash seed: compile with `-Dluai_makeseed(L)=0` (`lstate.c` guards it with `#if !defined(luai_makeseed)`). Without this, `pairs` order changes from run to run.
   - Random seed: after opening `math`, run the equivalent of `math.randomseed(42)`.
   - Addresses: `tostring` of a table or function prints `%p` through `lua_pointer2str`. That macro is **not** guarded in `luaconf.h` (line 618), so `-D` can't override it. Simplest: the generator never prints an address. The prompt tells the model to read the format from the driver; a comment in the driver saying addresses are never printed is enough.
4. **Run it.** `lua_pcall(L, 0, 0, 0)`. Output is whatever the script `print`s. `print` writes through `lua_writestring`, with stdout fully buffered and flushed at exit.
5. **Report errors.** If load or pcall fails, flush the output so far, then write `ERROR <msg>\n`:
   - `<msg>` is `lua_tostring` of the error value, or `(error object is a <type> value)` when it is not a string.
   - Exit 0, so a Lua error is output, not a crash.
6. **No hooks and no memory limit.** Infinite loops are left to verify.sh `--timeout`, so the generator must not emit them.

## Rust crate: `backend/harness/lua/rust_crate/`

- **`INTERFACE.md`:** `pub fn run(script: &[u8]) -> Vec<u8>` returns exactly the driver's stdout bytes, with the `ERROR` line included.
- **`src/main.rs`:** reads stdin, calls `lua::run` **on a thread with a 256 MB stack** (the parser and metamethod calls may recurse up to `LUAI_MAXCCALLS` = 200 levels), writes the bytes, and exits 0. It has `#![forbid(unsafe_code)]`, like the expat crate.
- **`src/lua.rs`:** a template stub with the `run` signature.
- **Build script:** `scripts/build_rust_lua.sh <lua.rs> [build_id]`, the same JSON contract as `build_rust_expat.sh`, including the rejection of the word `unsafe`.

## Inputs: `backend/harness/lua/gen_inputs_lua.py --seed S --count N --out DIR`

Same CLI as `gen_inputs_expat.py`. One `.lua` file per input. Every script must be deterministic, finish quickly, and stay within the libraries in scope. Suggested mix:

- **Fixed edge cases** (the same in every seed, like libexpat's 747): integer and float printing (`%.14g`, `-0.0`, `inf`, `nan`, `math.maxinteger + 1`), `//` and `%` with negatives, string↔number coercion, hex floats, every `string.format` conversion, the pattern classes plus `%b` and `%f`, `gsub` with tables and functions, `pairs` order over mixed keys, `#` on tables with holes, `table.sort` with a comparator (including an invalid one), `table.concat`, `table.move`, `utf8.*`, goto, `<close>`, coroutines (wrap, yield across pcall, dead coroutines), metatables (`__index`, `__newindex`, `__call`, `__eq`, `__lt`, `__le`, `__concat`, `__len`, `__close`, `__tostring`, `__name`), error messages with variable names (global, local, field, upvalue, method), `error()` with non-string values and levels, and syntax errors (`near` tokens, `<eof>`).
- **Generated programs:** random expression trees over ints, floats and strings with `print`; random table build-and-iterate programs; string-library calls on random byte strings; deep nesting (C stack overflow at 200 levels, and deep Lua recursion to "stack overflow").
- **Lua's own test suite** (`lua-5.4.7-tests`), as a corpus: only the files that avoid `io`, `os`, `debug` and `package`, with addresses and timing removed.

## Before the first model call

`scripts/preflight.sh lua` should apply the same gate as libexpat:
- canary;
- C vs C at 1.0;
- the reject-everything stub (`run` returns `ERROR x`) at or below 0.05.

Then:

```
cd backend && THESEUS_ROOT=<main checkout> uv run python -m ai.loop --target lua --model gemini-4-argon \
  --rounds 4 --fresh-repair --seed 1 --count 300 --holdout-count 2000 --jobs 4 --timeout 20 \
  --out ../runs/ai/lua-long-decode
```

Translate runs at thinking high and repair at medium, the same as libexpat. polish_rounds is 3. Expect about 30–50 minutes for round 1.
