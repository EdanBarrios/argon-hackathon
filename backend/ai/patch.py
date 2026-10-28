"""Apply a model reply made of SEARCH/REPLACE blocks to a Rust source file.

Repair used to ask for the whole file back, so a 1 to 3 line fix cost a full
regeneration (6,554 lines for the libexpat port). A reply is now a few blocks:

    <<<<<<< SEARCH
    exact lines from the current file
    =======
    replacement lines
    >>>>>>> REPLACE

apply_patch is all or nothing. Every SEARCH must match exactly once in the text as
edited by the blocks before it; anything else raises PatchError and the caller falls
back to a full-file repair. Matching is exact, except that trailing whitespace on a
line is ignored (models often drop it).
"""

from __future__ import annotations

import re
from dataclasses import dataclass


class PatchError(ValueError):
    """The reply had no usable blocks, or a block did not match exactly once."""


@dataclass
class PatchStats:
    blocks: int
    lines_removed: int
    lines_added: int


_BLOCK = re.compile(
    r"^<{7} SEARCH[ \t]*\r?\n(.*?)^={7}[ \t]*\r?\n(.*?)^>{7} REPLACE[ \t]*$",
    re.DOTALL | re.MULTILINE,
)


def parse_blocks(reply: str | None) -> list[tuple[str, str]]:
    """(search, replace) pairs in reply order. Raises PatchError if there are none."""
    if not reply:
        raise PatchError("reply was empty: no SEARCH/REPLACE blocks")
    blocks = [(m.group(1), m.group(2)) for m in _BLOCK.finditer(reply)]
    if not blocks:
        if "<<<<<<< SEARCH" in reply:
            raise PatchError("reply has a SEARCH marker but no complete block (truncated output?)")
        raise PatchError("reply contains no SEARCH/REPLACE blocks")
    return blocks


def _lines(text: str) -> list[str]:
    return text.splitlines()


def _find(lines: list[str], search: list[str]) -> list[int]:
    """Start indexes where search matches lines, ignoring trailing whitespace."""
    want = [s.rstrip() for s in search]
    n = len(want)
    return [i for i in range(len(lines) - n + 1)
            if all(lines[i + j].rstrip() == want[j] for j in range(n))]


def apply_patch(source: str, reply: str | None) -> tuple[str, PatchStats]:
    """source with every block of reply applied, in order. Raises PatchError, never partial."""
    lines = _lines(source)
    removed = added = 0
    blocks = parse_blocks(reply)
    for i, (search, replace) in enumerate(blocks, 1):
        s, r = _lines(search), _lines(replace)
        if not any(x.strip() for x in s):
            raise PatchError(f"block {i}: SEARCH is empty or blank, so it cannot be placed")
        hits = _find(lines, s)
        if not hits:
            raise PatchError(f"block {i}: SEARCH text not found: {s[0].strip()[:80]!r}")
        if len(hits) > 1:
            raise PatchError(f"block {i}: SEARCH text matches {len(hits)} places, add context: {s[0].strip()[:80]!r}")
        at = hits[0]
        lines[at:at + len(s)] = r
        removed += len(s)
        added += len(r)
    return "\n".join(lines).rstrip("\n") + "\n", PatchStats(len(blocks), removed, added)
