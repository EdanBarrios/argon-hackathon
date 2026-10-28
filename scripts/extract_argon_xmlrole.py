#!/usr/bin/env python3
"""Copy Argon's prolog/role state machine out of its full expat.rs port.

Usage: python3 -I scripts/extract_argon_xmlrole.py <argon expat.rs> <out argon_role.rs>

Every emitted line except the comment markers and the one re-opened
`impl Encoding {` / `}` pair is a verbatim line of Argon's file. Nothing is
rewritten. Blocks copied: the XML_TOK_* and XML_ROLE_* constants, the
Encoding and InitEncIndex enums, three Encoding methods (min_bytes_per_char,
char_matches, name_matches_ascii), and PrologHandler / PrologState with its
whole impl.
"""
import hashlib
import re
import sys

path, dest = sys.argv[1], sys.argv[2]
raw = open(path, "rb").read()
src = raw.decode("utf-8").split("\n")
out = []


def find(pat):
    for i, s in enumerate(src, 1):
        if re.match(pat, s):
            return i
    raise SystemExit("pattern not found: " + pat)


def block_end(i):
    """Line of the closing brace matching the item that starts on line i."""
    ind = len(src[i - 1]) - len(src[i - 1].lstrip())
    j = i
    while True:
        j += 1
        s = src[j - 1]
        if s.startswith(" " * ind + "}") and len(s) - len(s.lstrip()) == ind:
            return j


def emit(a, b):
    out.append("// --- Argon expat.rs lines %d-%d (verbatim)" % (a, b))
    out.extend(src[a - 1:b])
    out.append("")


out.append("// AUTO-EXTRACTED from Argon's port, expat.rs sha256")
out.append("// %s" % hashlib.sha256(raw).hexdigest())
out.append("// by scripts/extract_argon_xmlrole.py. Do not edit by hand.")
out.append("")
out.append("// --- Argon expat.rs XML_TOK_* / XML_ROLE_* constants (verbatim)")
for s in src:
    if re.match(r"const (XML_TOK_|XML_ROLE_)\w+: i32", s):
        out.append(s)
out.append("")
for pat in (r"enum Encoding \{", r"enum InitEncIndex \{"):
    i = find(pat)
    emit(i - 1, block_end(i))  # include the #[derive] line above
out.append("impl Encoding { // extractor: re-opened impl holding 3 of Argon's methods")
for name in ("min_bytes_per_char", "char_matches", "name_matches_ascii"):
    i = find(r"    fn %s\(" % name)
    a = i - 1 if src[i - 2].strip() == "#[inline]" else i
    emit(a, block_end(i))
out.append("} // extractor: end of re-opened impl")
out.append("")
p = find(r"enum PrologHandler \{")
emit(p - 1, block_end(find(r"impl PrologState \{")))
open(dest, "w").write("\n".join(out) + "\n")
