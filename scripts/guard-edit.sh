#!/bin/bash
# PreToolUse guard for Edit|Write|MultiEdit|NotebookEdit.
# Enforces the CLAUDE.md ownership table using the role in .role.
# Cooperative guardrail, not a security boundary. Bash 3.2 compatible.
# Exit 0 = allow, exit 2 = block (stderr goes back to Claude).
# Any other non-zero exit would NOT block, so every failure path exits 2.

block() {
  echo "BLOCKED: $1" >&2
  exit 2
}

ROOT=$(cd "$(dirname "$0")/.." 2>/dev/null && pwd -P) || block "guard could not locate the repo root."

INPUT=$(cat)

# Extract tool_input.file_path (or notebook_path) and cwd.
if command -v jq >/dev/null 2>&1; then
  TARGET=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' 2>/dev/null) \
    || block "guard could not parse hook input."
  HOOK_CWD=$(printf '%s' "$INPUT" | jq -r '.cwd // empty' 2>/dev/null)
else
  TARGET=$(printf '%s' "$INPUT" | python3 -I -c '
import json, sys
t = json.load(sys.stdin).get("tool_input") or {}
print(t.get("file_path") or t.get("notebook_path") or "")' 2>/dev/null) \
    || block "guard could not parse hook input."
  HOOK_CWD=$(printf '%s' "$INPUT" | python3 -I -c '
import json, sys
print(json.load(sys.stdin).get("cwd") or "")' 2>/dev/null)
fi

# Not a file edit: nothing to check.
[ -z "$TARGET" ] && exit 0

# Make absolute.
case "$TARGET" in
  /*) ABS="$TARGET" ;;
  *) ABS="${HOOK_CWD:-$PWD}/$TARGET" ;;
esac

# Lexically normalize . and .. segments.
NORM=""
OLD_IFS=$IFS
IFS=/
for seg in $ABS; do
  case "$seg" in
    ""|.) ;;
    ..) NORM="${NORM%/*}" ;;
    *) NORM="$NORM/$seg" ;;
  esac
done
IFS=$OLD_IFS
[ -z "$NORM" ] && NORM=/

# Resolve symlinks on the deepest existing ancestor (the file may not exist yet).
DIR="$NORM"
REST=""
while [ ! -d "$DIR" ]; do
  REST="/${DIR##*/}$REST"
  DIR="${DIR%/*}"
  [ -z "$DIR" ] && DIR=/
done
REAL_DIR=$(cd "$DIR" 2>/dev/null && pwd -P) || block "guard could not resolve $TARGET."
[ "$REAL_DIR" = "/" ] && REAL_DIR=""
RESOLVED="$REAL_DIR$REST"

# Outside the repo: allowed.
case "$RESOLVED" in
  "$ROOT") REL="" ;;
  "$ROOT"/*) REL="${RESOLVED#"$ROOT"/}" ;;
  *) exit 0 ;;
esac

# Inside the repo: role required.
ROLE=""
[ -f "$ROOT/.role" ] && ROLE=$(tr -d '[:space:]' < "$ROOT/.role")
case "$ROLE" in
  ai|frontend|systems) ;;
  *) block "$REL: .role is missing or invalid, so this session is read-only. Run: echo <ai|frontend|systems> > .role" ;;
esac

# Owner of a repo-relative path, per CLAUDE.md.
case "$REL" in
  docs/DECISIONS.md) exit 0 ;;
  backend/ai|backend/ai/*) OWNER=ai ;;
  frontend|frontend/*) OWNER=frontend ;;
  *) OWNER=systems ;;  # backend/api, backend/harness, scripts, and all shared files
esac

[ "$ROLE" = "$OWNER" ] && exit 0
block "$REL is owned by $OWNER. Ask them for a handoff; do not work around this."
