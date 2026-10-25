/* Lua 5.4.7 differential driver (the C side). See rust_crate/INTERFACE.md.
 * Reads one script from stdin, runs it with base, coroutine, table, string, math, utf8,
 * prints what the script prints; on failure prints "ERROR <msg>". Always exits 0.
 * Built with -Dluai_makeseed(L)=0 so string hashes and pairs order are fixed. */
#include <stdio.h>
#include <stdlib.h>
#include "lua.h"
#include "lauxlib.h"
#include "lualib.h"

static const luaL_Reg libs[] = {
  {LUA_GNAME, luaopen_base},      {LUA_COLIBNAME, luaopen_coroutine},
  {LUA_TABLIBNAME, luaopen_table}, {LUA_STRLIBNAME, luaopen_string},
  {LUA_MATHLIBNAME, luaopen_math}, {LUA_UTF8LIBNAME, luaopen_utf8},
  {NULL, NULL}};

static char *read_stdin(size_t *len) {
  size_t cap = 1 << 16, n = 0;
  char *buf = malloc(cap);
  size_t got;
  if (!buf) return NULL;
  while ((got = fread(buf + n, 1, cap - n, stdin)) > 0) {
    n += got;
    if (n == cap) {
      char *nb = realloc(buf, cap *= 2);
      if (!nb) { free(buf); return NULL; }
      buf = nb;
    }
  }
  *len = n;
  return buf;
}

int main(void) {
  size_t len = 0;
  char *src = read_stdin(&len);
  if (!src) { fputs("driver: failed to read stdin\n", stderr); return 2; }
  lua_State *L = luaL_newstate();
  if (!L) return 2;
  for (const luaL_Reg *lib = libs; lib->func; lib++) {
    luaL_requiref(L, lib->name, lib->func, 1);
    lua_pop(L, 1);
  }
  lua_pushnil(L); lua_setglobal(L, "dofile");
  lua_pushnil(L); lua_setglobal(L, "loadfile");
  lua_pushnil(L); lua_setglobal(L, "require");
  /* math.randomseed(42) */
  lua_getglobal(L, "math"); lua_getfield(L, -1, "randomseed");
  lua_pushinteger(L, 42); lua_call(L, 1, 0); lua_pop(L, 1);

  int st = luaL_loadbufferx(L, src, len, "=input", "t");
  if (st == LUA_OK) st = lua_pcall(L, 0, 0, 0);
  if (st != LUA_OK) {
    fflush(stdout);
    if (lua_type(L, -1) == LUA_TSTRING || lua_type(L, -1) == LUA_TNUMBER)
      printf("ERROR %s\n", lua_tostring(L, -1));
    else
      printf("ERROR (error object is a %s value)\n", luaL_typename(L, -1));
  }
  fflush(stdout);
  lua_close(L);
  free(src);
  return 0;
}
