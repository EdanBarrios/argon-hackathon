"""Coarse divergence signatures, so 50 copies of one bug count as one pattern.

One function per target output format. trim_feedback groups divergences by these.
"""

from __future__ import annotations


def _kind(line: str | None) -> str:
    line = line or ""
    if line.startswith("OK\t"):
        return "OK"
    if line.startswith("PARSE_ERROR at"):
        return "ERR_AT"
    if line == "PARSE_ERROR":
        return "ERR"
    if line == "TIMEOUT":
        return "TIMEOUT"
    return "OTHER"


def _char_class(ch: str) -> str:
    if ch == "":
        return "end"
    if ch.isdigit() or ch in "+-.eE":
        return "num"
    if ch == '"':
        return "quote"
    if ch == "\\":
        return "backslash"
    if ch in "{}[],:":
        return ch
    if ch.isalpha():
        return "alpha"
    return "punct"


def divergence_pattern(d: dict) -> str:
    """Coarse signature of a divergence, so 50 copies of one bug count as one pattern."""
    c, r = d.get("c_output") or "", d.get("rust_output") or ""
    kc, kr = _kind(c), _kind(r)
    if kc == "OK" and kr == "OK":
        i = next((j for j, (a, b) in enumerate(zip(c, r)) if a != b), min(len(c), len(r)))
        before = c[max(0, i - 1):i]
        in_string = c[:i].replace("\\\\", "").replace('\\"', "").count('"') % 2 == 1
        where = "string" if in_string else ("number" if before and _char_class(before) == "num" else "structure")
        return f"OK/OK print differs in {where}: C next={_char_class(c[i:i+1])} Rust next={_char_class(r[i:i+1])}"
    inp = d.get("input") or ""
    if kc == "ERR_AT" and kr == "ERR_AT":
        try:
            oc, orr = int(c.rsplit(" ", 1)[1]), int(r.rsplit(" ", 1)[1])
            side = "earlier" if orr < oc else "later"
        except ValueError:
            oc, side = 0, "?"
        # Byte class at C's error offset (approximate: the report input is text).
        return f"both reject, Rust error offset {side} than C, C stops at {_char_class(inp[oc:oc+1])}"
    # Accept/reject disagreements: split by what the document starts with.
    first = inp.lstrip(" \t\r\n")[:1]
    return f"C {kc} vs Rust {kr}, input starts with {_char_class(first)}"


def _expat_kind(out: str) -> str:
    if out.startswith("ERROR "):
        return "ERROR " + out.split(" ", 2)[1]
    return "TIMEOUT" if out == "TIMEOUT" else "events"


def expat_divergence_pattern(d: dict) -> str:
    """libexpat signature: error codes when either side fails, else the first differing event kind."""
    kc, kr = _expat_kind(d.get("c_output") or ""), _expat_kind(d.get("rust_output") or "")
    if kc.startswith("ERROR") and kr.startswith("ERROR"):
        if kc != kr:
            return f"both reject: C {kc}, Rust {kr}"
        return f"both reject with {kc}, line/col differ"
    if kc != kr:
        return f"C {kc} vs Rust {kr}"
    fd = d.get("first_diff") or {}
    first = lambda line: (line or "<end of output>").split(" ", 1)[0]
    if fd:
        return f"both accept, first differing line: C {first(fd.get('c'))} vs Rust {first(fd.get('rust'))}"
    return "both accept, events differ"


def _lua_last(out: str) -> str:
    """Kind of the driver's last line: the error class if the script failed, else output."""
    if out == "TIMEOUT":
        return "TIMEOUT"
    last = out.rstrip("\n").rsplit("\n", 1)[-1]
    if not last.startswith("ERROR "):
        return "output"
    msg = last[len("ERROR "):]
    if msg.startswith("input:"):
        msg = msg.split(": ", 1)[-1]
    # First few words name the error class ("attempt to index", "'=' expected near").
    return "ERROR " + " ".join(msg.split()[:3])


def lua_divergence_pattern(d: dict) -> str:
    """Lua signature: the error class when either side fails, else the first differing line's start."""
    kc, kr = _lua_last(d.get("c_output") or ""), _lua_last(d.get("rust_output") or "")
    if kc != kr:
        return f"C {kc} vs Rust {kr}"
    if kc == "TIMEOUT":
        return "both TIMEOUT"
    if kc.startswith("ERROR"):
        return f"both fail with {kc}, output before or message tail differs"
    fd = d.get("first_diff") or {}
    first = lambda line: (line or "<end of output>")[:12]
    if fd:
        return f"both succeed, first differing line: C {first(fd.get('c'))!r} vs Rust {first(fd.get('rust'))!r}"
    return "both succeed, output differs"
