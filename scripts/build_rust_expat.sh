#!/bin/bash
# Build an Argon-produced expat.rs into the theseus_expat driver binary.
# Usage: scripts/build_rust_expat.sh <path/to/expat.rs> [build_id]
# Prints one JSON line on stdout:
#   {"ok":true,"binary":"<abs path>","build_seconds":N}
#   {"ok":false,"errors":"...","build_seconds":N}   (exit 1)
# Bash 3.2 compatible. Needs cargo and python3.
set -u

ROOT=$(cd "$(dirname "$0")/.." && pwd -P)
TEMPLATE="$ROOT/backend/harness/expat/rust_crate"
BUILD_TIMEOUT=${BUILD_TIMEOUT:-300}
ERROR_LINES=150
START=$(date +%s)

# emit <ok:true|false> <binary-or-empty> <errors-or-empty>
emit() {
  python3 -I -c '
import json, sys
ok = sys.argv[1] == "true"
out = {"ok": ok}
if ok:
    out["binary"] = sys.argv[2]
else:
    out["errors"] = sys.argv[3]
out["build_seconds"] = int(sys.argv[4])
print(json.dumps(out, separators=(",", ":")))' "$1" "$2" "$3" "$(( $(date +%s) - START ))"
}

fail() {
  emit false "" "$1"
  exit 1
}

[ $# -ge 1 ] && [ $# -le 2 ] || fail "usage: scripts/build_rust_expat.sh <path/to/expat.rs> [build_id]"
SRC=$1
BUILD_ID=${2:-$(date +%Y%m%d-%H%M%S)-$$}

case "$BUILD_ID" in
  ''|*[!A-Za-z0-9._-]*|.|..) fail "invalid build_id '$BUILD_ID' (allowed: A-Z a-z 0-9 . _ -)" ;;
esac
[ -f "$SRC" ] || fail "expat.rs not found: $SRC"
command -v cargo >/dev/null 2>&1 || fail "cargo not found on PATH"

# Safe-Rust guarantee: reject the keyword anywhere, comments included.
if grep -n -w 'unsafe' "$SRC" >/dev/null 2>&1; then
  HITS=$(grep -n -w 'unsafe' "$SRC" | head -n 10)
  fail "rejected: expat.rs contains the keyword 'unsafe' (safe Rust only; remove it everywhere, comments included). Lines:
$HITS"
fi

# Sandbox guarantee: the module must translate libexpat, not shell out to it.
# forbid(unsafe_code) does not stop safe std::process::Command running the C
# binary, std::fs reading expected outputs, or include!/#[path]/mod pulling in
# other files. Scan expat.rs only (the fixed main.rs uses std::process::exit).
# Comments are blanked first (they cannot run anything, and the stub's own doc
# says "no `extern crate`"); string literals are kept, so text hidden in a
# string still counts. Multi-line, so grouped imports such as
# `use std::{\n fs,\n};` are caught.
BANNED=$(python3 -I - "$SRC" <<'PY'
import re, sys
raw = open(sys.argv[1], encoding="utf-8", errors="replace").read()

def strip_comments(s):
    out, i, n = [], 0, len(s)
    while i < n:
        c = s[i]
        if s.startswith("//", i):
            j = s.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i)); i = j
        elif s.startswith("/*", i):  # Rust block comments nest
            depth, j = 1, i + 2
            while j < n and depth:
                if s.startswith("/*", j): depth += 1; j += 2
                elif s.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            out.append(re.sub(r"[^\n]", " ", s[i:j])); i = j
        elif re.match(r'b?r#*"', s[i:i + 300]):  # raw string
            m = re.match(r'b?r(#*)"', s[i:])
            end = s.find('"' + m.group(1), i + m.end())
            j = n if end < 0 else end + 1 + len(m.group(1))
            out.append(s[i:j]); i = j
        elif c == '"':
            j = i + 1
            while j < n and s[j] != '"':
                j += 2 if s[j] == "\\" else 1
            out.append(s[i:j + 1]); i = j + 1
        elif c == "'":  # char literal, else a lifetime
            m = re.match(r"'(\\.[^']*|[^\\'])'", s[i:])
            j = i + (m.end() if m else 1)
            out.append(s[i:j]); i = j
        else:
            out.append(c); i += 1
    return "".join(out)

src = strip_comments(raw)
MODS = r"(?:process|fs|net|env|os)"
rules = [
    (r"\b(?:std|core|alloc)\s*::\s*" + MODS + r"\b", "std::process/fs/net/env/os"),
    (r"\b" + MODS + r"\s*::", "process/fs/net/env/os path (aliased import)"),
    (r"\buse\s+(?:::)?std\s*::\s*\{[^}]*\b" + MODS + r"\b", "grouped import of process/fs/net/env/os"),
    (r"\bstd\s+as\b", "renamed std crate"),
    (r"\bCommand\b", "Command"),
    (r"\binclude(?:_str|_bytes)?\s*!", "include!/include_str!/include_bytes!"),
    (r"\bextern\s+crate\b", "extern crate"),
    (r"#\s*!?\s*\[\s*path\b", "#[path] attribute"),
    (r"\bmod\s+[A-Za-z_][A-Za-z0-9_]*\s*;", "out-of-line `mod name;` declaration"),
]
hits = []
for pat, why in rules:
    for m in re.finditer(pat, src):
        line = src.count("\n", 0, m.start()) + 1
        text = raw.splitlines()[line - 1].strip()[:120]
        hits.append((line, "%d: [%s] %s" % (line, why, text)))
for _, h in sorted(set(hits))[:10]:
    print(h)
PY
) || fail "rejected: could not scan expat.rs for banned APIs"
if [ -n "$BANNED" ]; then
  fail "rejected: expat.rs uses a banned API (no std::process, std::fs, std::net, std::env, std::os, Command, include!, extern crate, #[path] or out-of-line mod; translate the C, do not call it or read files). Lines:
$BANNED"
fi

DEST="$ROOT/runs/build-expat/$BUILD_ID"
rm -rf "$DEST"
mkdir -p "$DEST/src" || fail "could not create $DEST"
cp "$TEMPLATE/Cargo.toml" "$DEST/" || fail "could not copy crate template"
[ -f "$TEMPLATE/Cargo.lock" ] && cp "$TEMPLATE/Cargo.lock" "$DEST/"
cp "$TEMPLATE/src/main.rs" "$DEST/src/main.rs" || fail "could not copy main.rs"
cp "$SRC" "$DEST/src/expat.rs" || fail "could not copy $SRC"

# Run "$@" with a wall-clock limit. Uses timeout/gtimeout if present,
# otherwise a perl fork+alarm that kills the whole process group. Exit 124 on timeout.
run_with_timeout() {
  local secs=$1; shift
  if command -v timeout >/dev/null 2>&1; then
    timeout "$secs" "$@"
  elif command -v gtimeout >/dev/null 2>&1; then
    gtimeout "$secs" "$@"
  else
    perl -e '
      my $secs = shift;
      my $pid = fork();
      die "fork failed\n" unless defined $pid;
      if ($pid == 0) { setpgrp(0, 0); exec @ARGV or exit 127; }
      local $SIG{ALRM} = sub { kill "KILL", -$pid; waitpid($pid, 0); exit 124; };
      alarm $secs;
      waitpid($pid, 0);
      exit($? & 127 ? 128 + ($? & 127) : $? >> 8);
    ' "$secs" "$@"
  fi
}

LOG="$DEST/cargo-stderr.log"
(
  cd "$DEST" || exit 1
  CARGO_TERM_COLOR=never RUSTFLAGS="${RUSTFLAGS:-} -A warnings" \
    run_with_timeout "$BUILD_TIMEOUT" cargo build --release --offline --quiet
) >/dev/null 2>"$LOG"
STATUS=$?

if [ "$STATUS" -eq 124 ]; then
  fail "cargo build timed out after ${BUILD_TIMEOUT}s"
fi
BIN="$DEST/target/release/theseus_expat"
if [ "$STATUS" -ne 0 ] || [ ! -x "$BIN" ]; then
  ERRS=$(head -n "$ERROR_LINES" "$LOG")
  [ -n "$ERRS" ] || ERRS="cargo build failed with exit $STATUS and no stderr"
  fail "$ERRS"
fi

emit true "$BIN" ""
