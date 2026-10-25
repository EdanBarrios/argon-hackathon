"""Text-similarity metrics between Rust corpora (provenance check).

Treats every input file as data: reads text only, never imports or executes it.
Usage: python3 -I similarity.py OUT_JSON CLIB_DIR NAME=PATH [NAME=PATH ...]
PATH is a .rs file or a directory (all *.rs under src/, excluding tests).
CLIB_DIR holds libexpat's .c/.h, used to check whether shared identifiers are
simply inherited from the C source both ports translate.
"""
import hashlib
import itertools
import json
import os
import re
import sys

MIN_LINE = 30
NGRAM = 8


def rs_files(path):
    if os.path.isfile(path):
        return [path]
    out = []
    for root, _dirs, files in os.walk(path):
        rel = os.path.relpath(root, path)
        if rel.split(os.sep)[0] in ("tests", "benches", "examples"):
            continue
        for f in files:
            if f.endswith(".rs") and f != "tests.rs":
                out.append(os.path.join(root, f))
    return sorted(out)


def strip_comments(src):
    # Remove /* */ and // comments; keep string literals (crude but symmetric).
    src = re.sub(r"/\*.*?\*/", " ", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def strip_cfg_test(src):
    # Drop inline `#[cfg(test)] mod tests { ... }` blocks by brace matching.
    out, i = [], 0
    for m in re.finditer(r"#\[cfg\(test\)\]\s*mod\s+\w+\s*\{", src):
        if m.start() < i:
            continue
        out.append(src[i:m.start()])
        depth, j = 1, m.end()
        while j < len(src) and depth:
            depth += {"{": 1, "}": -1}.get(src[j], 0)
            j += 1
        i = j
    out.append(src[i:])
    return "".join(out)


def load(path):
    texts = []
    for f in rs_files(path):
        texts.append(strip_cfg_test(strip_comments(open(f, encoding="utf-8", errors="replace").read())))
    return "\n".join(texts)


TRIVIAL = re.compile(r"^(use |mod |pub mod |pub use |extern crate|#\[|[{}()\[\];,]+$)")


def norm_lines(src):
    lines = set()
    for ln in src.splitlines():
        n = " ".join(ln.split())
        if len(n) >= MIN_LINE and not TRIVIAL.match(n):
            lines.add(n)
    return lines


TOK = re.compile(r'"(?:\\.|[^"\\])*"|b"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'|b\'(?:\\.|[^\'\\])\''
                 r"|[A-Za-z_][A-Za-z0-9_]*|0x[0-9A-Fa-f_]+|\d[\d_]*|::|->|=>|==|!=|<=|>=|&&|\|\||\S")


def tokens(src):
    return TOK.findall(src)


TABLE = re.compile(r"=\s*\[[^\[\]]{300,}?\]\s*;", re.S)


def code_tokens(src):
    """Tokens with large array-literal data tables (e.g. nametab.h bitmaps) removed."""
    return TOK.findall(TABLE.sub("= [];", src))


DECL = re.compile(r"\b(fn|struct|enum|trait|type|const|static)\s+([A-Za-z_][A-Za-z0-9_]*)")


KEYWORDS = {"fn", "unsafe", "extern", "mut", "impl", "for", "where", "dyn", "self", "Self"}


def idents(src):
    return {m.group(2) for m in DECL.finditer(src)} - KEYWORDS


def canon(name):
    return name.replace("_", "").lower()


def ngrams(toks, n):
    return {hashlib.blake2b("\x1f".join(toks[i:i + n]).encode(), digest_size=8).digest()
            for i in range(len(toks) - n + 1)}


def longest_common_run(ta, tb):
    """Longest contiguous identical token run, by binary search on hashed n-gram sets."""
    def shared(n):
        sb = {}
        for i in range(len(tb) - n + 1):
            sb.setdefault(hash(tuple(tb[i:i + n])), i)
        for i in range(len(ta) - n + 1):
            h = hash(tuple(ta[i:i + n]))
            if h in sb and tuple(tb[sb[h]:sb[h] + n]) == tuple(ta[i:i + n]):
                return i
        return None
    lo, hi, best = 0, min(len(ta), len(tb), 4000), (0, None)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        at = shared(mid)
        if at is not None:
            lo, best = mid, (mid, at)
        else:
            hi = mid - 1
    n, at = best
    return n, (" ".join(ta[at:at + n]) if at is not None else "")


def c_identifiers(cdir):
    names = set()
    for f in os.listdir(cdir):
        if f.endswith((".c", ".h")):
            names |= set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", open(os.path.join(cdir, f), errors="replace").read()))
    return {canon(n) for n in names}


def main():
    out_path, cdir = sys.argv[1], sys.argv[2]
    corpora = {}
    for arg in sys.argv[3:]:
        name, path = arg.split("=", 1)
        src = load(path)
        toks = tokens(src)
        ctoks = code_tokens(src)
        corpora[name] = {"lines": norm_lines(src), "idents": idents(src), "toks": toks,
                         "ctoks": ctoks, "cgrams": ngrams(ctoks, NGRAM),
                         "grams": ngrams(toks, NGRAM), "files": len(rs_files(path)),
                         "path": path}
    cids = c_identifiers(cdir)
    res = {"params": {"min_line_chars": MIN_LINE, "ngram": NGRAM}, "corpora": {}, "pairs": []}
    for k, v in corpora.items():
        res["corpora"][k] = {"files": v["files"], "tokens": len(v["toks"]),
                             "nontrivial_unique_lines": len(v["lines"]),
                             "declared_identifiers": len(v["idents"]),
                             f"unique_{NGRAM}grams": len(v["grams"])}
    for a, b in itertools.combinations(corpora, 2):
        A, B = corpora[a], corpora[b]
        sl = sorted(A["lines"] & B["lines"])
        ia, ib = {canon(x) for x in A["idents"]}, {canon(x) for x in B["idents"]}
        si = ia & ib
        gi = len(A["grams"] & B["grams"])
        run, run_text = longest_common_run(A["toks"], B["toks"])
        crun, crun_text = longest_common_run(A["ctoks"], B["ctoks"])
        cgi = len(A["cgrams"] & B["cgrams"])
        res["pairs"].append({
            "a": a, "b": b,
            "shared_nontrivial_lines": len(sl),
            "shared_lines_pct_of_a": round(100 * len(sl) / max(1, len(A["lines"])), 2),
            "shared_lines_pct_of_b": round(100 * len(sl) / max(1, len(B["lines"])), 2),
            "shared_lines": sl,
            "shared_identifiers": len(si),
            "identifier_jaccard": round(len(si) / max(1, len(ia | ib)), 4),
            "identifier_overlap_pct_of_a": round(100 * len(si) / max(1, len(ia)), 2),
            "shared_identifiers_also_in_libexpat_c": len(si & cids),
            "shared_identifiers_list": sorted(si),
            f"token_{NGRAM}gram_jaccard": round(gi / max(1, len(A["grams"] | B["grams"])), 5),
            f"token_{NGRAM}gram_containment_of_a": round(gi / max(1, len(A["grams"])), 5),
            "longest_common_token_run": run,
            "longest_common_token_run_text": run_text[:600],
            f"code_only_token_{NGRAM}gram_jaccard": round(cgi / max(1, len(A["cgrams"] | B["cgrams"])), 5),
            f"code_only_token_{NGRAM}gram_containment_of_a": round(cgi / max(1, len(A["cgrams"])), 5),
            "code_only_longest_common_token_run": crun,
            "code_only_longest_common_token_run_text": crun_text[:600],
            "shared_identifiers_not_in_libexpat_c": sorted(si - cids),
        })
    json.dump(res, open(out_path, "w"), indent=1)
    print(json.dumps(res["corpora"], indent=1))
    for p in res["pairs"]:
        print({k: v for k, v in p.items() if k not in ("shared_lines", "shared_identifiers_list")})


main()
