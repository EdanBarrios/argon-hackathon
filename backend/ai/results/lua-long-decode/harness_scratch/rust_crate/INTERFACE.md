# lua.rs interface

`lua.rs` is one safe-Rust module, the port of Lua 5.4.7 (core plus the base, coroutine,
table, string, math and utf8 libraries). It must export exactly:

```rust
pub fn run(script: &[u8]) -> Vec<u8>
```

`run` behaves like `drivers/lua_driver.c` given `script` on stdin, and returns the exact
bytes that driver writes to stdout:

1. Create a state; open only `_G` (base), `coroutine`, `table`, `string`, `math`, `utf8`.
   Remove `dofile`, `loadfile` and `require` from `_G` (set to nil). `load` stays,
   text chunks only.
2. The string hash seed is 0 (C: `-Dluai_makeseed(L)=0`).
3. Seed math.random as `math.randomseed(42)` does, after opening math.
4. Load `script` as a text chunk named "=input" (`luaL_loadbufferx(L, s, n, "=input", "t")`),
   so messages read "input:<line>: ...".
5. Call it with `lua_pcall(L, 0, 0, 0)`. `print` writes to stdout as lbaselib.c does
   (arguments through `luaL_tolstring`, separated by "\t", then "\n").
6. If load or the call fails: everything printed so far, then
   `ERROR <msg>\n`, where `<msg>` is the error value if it is a string or a number
   (converted as lua_tostring does), otherwise `(error object is a <typename> value)`.
7. Nothing else is ever written. Scripts never print addresses (`tostring` of tables or
   functions), never use `collectgarbage("count")`, `__gc` or weak tables, and always end.
