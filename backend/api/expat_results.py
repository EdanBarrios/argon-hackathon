"""libexpat: /try presets and GET /results/libexpat.

Every number in results() is read from committed result files (or counted from
the vendored C source); nothing is typed by hand.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

from . import harness

RICH_DOC = """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE catalog [
  <!ENTITY org "Theseus &amp; Argon">
  <!ATTLIST book lang CDATA "en">
]>
<catalog>
  <!-- a comment -->
  <?render mode="fast"?>
  <book id="b1" year="2026">
    <title>Ship of &org;</title>
    <code><![CDATA[if (a < b && c > d) { free(p); }]]></code>
  </book>
</catalog>
"""


def _billion_laughs(levels: int = 9, fanout: int = 10) -> str:
    """Same shape as gen_inputs_expat.py's billion_laughs (edge_bl_breach)."""
    parts = ['<?xml version="1.0"?>\n<!DOCTYPE lolz [\n <!ENTITY lol "lol">\n']
    for i in range(1, levels + 1):
        prev = "lol" if i == 1 else f"lol{i - 1}"
        parts.append(f' <!ENTITY lol{i} "{("&" + prev + ";") * fanout}">\n')
    parts.append(f"]>\n<lolz>&lol{levels};</lolz>\n")
    return "".join(parts)


PRESETS: dict[str, list[dict[str, str]]] = {
    "cjson": [],
    "expat": [
        {
            "id": "rich",
            "title": "Rich document",
            "description": "XML declaration, internal DTD with an entity and an ATTLIST default, "
                           "attributes, a comment, a PI, CDATA and an entity reference.",
            "input": RICH_DOC,
        },
        {
            "id": "billion_laughs",
            "title": "Billion laughs (CVE-2013-0340)",
            "description": "9 levels of 10x entity nesting (10^9 'lol's). Both sides must stop with "
                           "ERROR 43, the amplification limit breach, at the same line and column.",
            "input": _billion_laughs(),
        },
    ],
}


def _json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def _rel(path: Path) -> str:
    return str(path.relative_to(harness.ROOT))


def _c_lines() -> dict[str, Any]:
    lib = harness.EXPAT_VENDOR_LIB
    files = sorted([*lib.glob("*.c"), *lib.glob("*.h")]) if lib.is_dir() else []
    if files:
        n = sum(len(p.read_bytes().splitlines()) for p in files)
        return {"c_lines": n, "c_files": len(files),
                "c_lines_source": f"wc -l of {_rel(lib)}/*.c *.h (vendored R_2_9_0)"}
    readme = harness.EXPAT_HARNESS / "README.md"
    return {"c_lines": 18494, "c_files": 36,
            "c_lines_source": f"{_rel(readme)} (documented; vendor/ not fetched here)"}


def _diff_counts(diff: Path) -> dict[str, int]:
    lines = diff.read_text().splitlines()
    return {
        "added": sum(1 for line in lines if line.startswith(">")),
        "removed": sum(1 for line in lines if line.startswith("<")),
        "hunks": sum(1 for line in lines if line[:1].isdigit()),
    }


def _match(verify: dict[str, Any]) -> dict[str, Any]:
    return {"matched": verify["counts"]["match"], "tested": verify["behaviors_tested"],
            "match_rate": verify["match_rate"]}


def _seed(text: str, pattern: str) -> int | None:
    m = re.search(pattern, text)
    return int(m.group(1)) if m else None


def results() -> dict[str, Any]:
    base = harness.EXPAT_RESULTS
    argon = base / "argon-fresh-a"
    first = _json(base / "gemini-4-argon" / "summary.json")
    fixed = _json(argon / "summary.json")
    flash = _json(base / "gemini-3.8-flash" / "summary.json")
    holdout = _json(argon / "holdout_verify.json")
    loop = _json(argon / f"round_{fixed['best_round']}" / "verify.json")
    host = _json(harness.EXPAT_REVERIFY / "verify-seed4242.json")
    translate = first["rounds"][0]
    repairs = [r for r in fixed["rounds"] if r.get("kind") == "repair"]
    final = fixed["rounds"][-1]
    diff_path = argon / "round1_to_round3.diff"
    flash_round = flash["rounds"][0]
    return {
        "library": "expat",
        "c_version": harness.EXPAT_C_VERSION,
        **_c_lines(),
        "rust_lines": final["rust_lines"],
        "rust_lines_translate": translate["rust_lines"],
        "model": first["model"],
        "translate_seconds": translate["seconds"],
        "translate_output_tokens": translate["usage"]["total_output_tokens"],
        "compile_fix_rounds": len(repairs),
        "repair_seconds": round(sum(r["seconds"] for r in repairs), 1),
        "lines_changed_after_translate": _diff_counts(diff_path),
        "loop": {**_match(loop), "seed": _seed(fixed["inputs"], r"-s(\d+)$")},
        "holdout": {**_match(holdout), "seed": fixed["holdout"]["seed"]},
        "host_reverify": {**_match(host), "seed": _seed(
            (harness.EXPAT_REVERIFY / "README.md").read_text(), r"--seed (\d+)")},
        "flash_result": {
            "model": flash["model"],
            "status": "failed",
            "file": None,
            "reason": flash_round.get("error"),
        },
        "sources": {
            "translate": _rel(base / "gemini-4-argon" / "summary.json"),
            "repair_and_holdout": _rel(argon / "summary.json"),
            "loop_verify": _rel(argon / f"round_{fixed['best_round']}" / "verify.json"),
            "holdout_verify": _rel(argon / "holdout_verify.json"),
            "diff": _rel(diff_path),
            "host_reverify": _rel(harness.EXPAT_REVERIFY / "verify-seed4242.json"),
            "host_reverify_readme": _rel(harness.EXPAT_REVERIFY / "README.md"),
            "flash": _rel(base / "gemini-3.8-flash" / "summary.json"),
            "rust_source": _rel(harness.EXPAT_RUST_SRC),
        },
    }
