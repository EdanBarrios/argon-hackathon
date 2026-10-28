"""Coverage-guided differential fuzzer: libexpat 2.9.0 C vs the Rust port.

Each worker: load the seed corpus, record C edge coverage (trace-pc-guard
bitmap from expat_driver_edge), then loop: pick a corpus input, mutate it,
run C (edge build) and Rust on it, classify with the verifier's own
classify() from backend/harness/verify.py, and keep the input if it hit a C
edge no earlier input in this worker hit. Workers share new inputs through
their queue directories.

Usage: python3 -I diff_fuzz.py --c EDGE_BIN --rust RUST_BIN --seeds DIR --out DIR
         --worker K --seconds N [--timeout 2]
"""
import argparse
import hashlib
import json
import os
import random
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "..", "backend", "harness"))
import verify  # noqa: E402  (run_one, classify: same comparison as verify.sh)

TOKENS = [
    b"<a>", b"</a>", b"<a/>", b"<b x='1'>", b"</b>", b' x="1"', b" y='&amp;'", b' z="&#x41;"',
    b"&amp;", b"&lt;", b"&gt;", b"&quot;", b"&apos;", b"&#65;", b"&#x10FFFF;", b"&#0;", b"&#xD800;",
    b"&e;", b"&f;", b"%p;", b"<![CDATA[", b"]]>", b"<![CDATA[x]]>", b"<!--", b"-->", b"<!-- c -->",
    b"<?pi data?>", b"<?xml version='1.0'?>", b'<?xml version="1.0" encoding="ISO-8859-1"?>',
    b'<?xml version="1.0" encoding="US-ASCII"?>', b'<?xml version="1.0" encoding="UTF-16"?>',
    b'<?xml version="1.0" standalone="yes"?>', b"<!DOCTYPE a [", b"]>", b"<!DOCTYPE a>",
    b'<!DOCTYPE a SYSTEM "x.dtd">', b'<!DOCTYPE a PUBLIC "-//x//y" "z">',
    b'<!ENTITY e "v">', b'<!ENTITY f "<a>&e;</a>">', b'<!ENTITY % p "q">', b'<!ENTITY u SYSTEM "u">',
    b'<!ENTITY n SYSTEM "n" NDATA g>', b"<!NOTATION g SYSTEM 'g'>",
    b"<!ELEMENT a (#PCDATA|b)*>", b"<!ELEMENT a ANY>", b"<!ELEMENT a EMPTY>", b"<!ELEMENT a (b,c?)+>",
    b"<!ATTLIST a x CDATA 'd'>", b"<!ATTLIST a y ID #REQUIRED>", b"<!ATTLIST a z (p|q) 'p'>",
    b"<!ATTLIST a w NOTATION (g) #IMPLIED>", b"<!ATTLIST a v CDATA #FIXED 'f'>",
    b"<![IGNORE[", b"<![INCLUDE[", b"\r\n", b"\r", b"\n", b"\t", b" ", b"\xef\xbb\xbf",
    b"\xc3\xa9", b"\xe2\x82\xac", b"\xf0\x9f\x98\x80", b"\xff", b"\xc0\x80", b"\xed\xa0\x80", b"\x00",
    b"xmlns='u'", b"xmlns:p='u'", b"<p:a>", b"'", b'"', b"<", b">", b"&", b"=", b"]", b"[", b"%",
]


def mutate(data, pool, rng):
    d = bytearray(data)
    for _ in range(rng.choice((1, 1, 2, 3, 4, 8))):
        op = rng.randrange(11)
        n = len(d)
        pos = rng.randrange(n + 1) if n else 0
        if op == 0 and n:
            d[rng.randrange(n)] ^= 1 << rng.randrange(8)
        elif op == 1 and n:
            d[rng.randrange(n)] = rng.randrange(256)
        elif op == 2:
            d[pos:pos] = bytes([rng.randrange(256)])
        elif op == 3 and n:
            ln = rng.randint(1, min(32, n))
            s = rng.randrange(n - ln + 1)
            del d[s:s + ln]
        elif op == 4 and n:
            ln = rng.randint(1, min(64, n))
            s = rng.randrange(n - ln + 1)
            d[pos:pos] = d[s:s + ln]
        elif op in (5, 6, 7):
            d[pos:pos] = rng.choice(TOKENS)
        elif op == 8 and n:
            ln = rng.randint(1, min(16, n))
            s = rng.randrange(n - ln + 1)
            d[s:s + ln] = rng.choice(TOKENS)
        elif op == 9 and pool:
            other = rng.choice(pool)
            if other:
                a = rng.randrange(len(other))
                b = rng.randint(a, min(len(other), a + 256))
                d[pos:pos] = other[a:b]
        elif op == 10 and rng.random() < 0.3:
            try:  # transcode to UTF-16 with a BOM to reach little2_/big2_ tokenizers
                txt = bytes(d).decode("utf-8")
                enc = rng.choice(("utf-16-le", "utf-16-be"))
                bom = b"\xff\xfe" if enc == "utf-16-le" else b"\xfe\xff"
                d = bytearray((bom if rng.random() < 0.8 else b"") + txt.encode(enc))
            except UnicodeDecodeError:
                pass
    if len(d) > 65536:
        del d[65536:]
    return bytes(d)


def run_c(cbin, data, timeout, env, edge_path):
    env = dict(env, EDGE_OUT=edge_path)
    try:
        os.unlink(edge_path)
    except OSError:
        pass
    res = verify.run_one([cbin], data, timeout, env)
    try:
        with open(edge_path, "rb") as f:
            bm = f.read()
    except OSError:
        bm = b""
    return res, bm


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--c", required=True)
    ap.add_argument("--rust", required=True)
    ap.add_argument("--seeds", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--worker", type=int, default=0)
    ap.add_argument("--seconds", type=float, default=1200)
    ap.add_argument("--timeout", type=float, default=2.0)
    a = ap.parse_args()
    rng = random.Random(1000 + a.worker)
    env = verify.child_env() if hasattr(verify, "child_env") else dict(os.environ, LC_ALL="C")
    qdir = os.path.join(a.out, "queue", "w%d" % a.worker)
    fdir = os.path.join(a.out, "findings")
    os.makedirs(qdir, exist_ok=True)
    os.makedirs(fdir, exist_ok=True)
    edge_path = os.path.join(a.out, "edge_w%d.bin" % a.worker)
    log = open(os.path.join(a.out, "log_w%d.jsonl" % a.worker), "a")

    seeds = []
    for name in sorted(os.listdir(a.seeds)):
        with open(os.path.join(a.seeds, name), "rb") as f:
            seeds.append(f.read())
    total = None

    def merge(bm):
        nonlocal total
        if total is None:
            total = bytearray(len(bm))
        new = 0
        for i, v in enumerate(bm):
            if v and not total[i]:
                total[i] = 1
                new += 1
        return new

    for s in seeds:
        _, bm = run_c(a.c, s, a.timeout * 5, env, edge_path)
        if bm:
            merge(bm)
    seed_edges = sum(total)
    pool = list(seeds)
    seen_q = set()
    counts = {}
    execs = new_inputs = 0
    start = time.monotonic()
    log.write(json.dumps({"event": "start", "worker": a.worker, "seed_edges": seed_edges,
                          "guards": len(total)}) + "\n")
    log.flush()
    while time.monotonic() - start < a.seconds:
        if execs % 300 == 0:  # pull other workers' new inputs into the pool
            qroot = os.path.join(a.out, "queue")
            for w in os.listdir(qroot):
                for name in os.listdir(os.path.join(qroot, w)):
                    key = (w, name)
                    if key in seen_q:
                        continue
                    seen_q.add(key)
                    if w != "w%d" % a.worker:
                        with open(os.path.join(qroot, w, name), "rb") as f:
                            pool.append(f.read())
        # favour recent finds a little
        base = pool[-rng.randint(1, min(len(pool), 200))] if rng.random() < 0.3 else rng.choice(pool)
        data = mutate(base, pool, rng)
        c, bm = run_c(a.c, data, a.timeout, env, edge_path)
        r = verify.run_one([a.rust], data, a.timeout, env)
        cls = verify.classify(c, r)
        counts[cls] = counts.get(cls, 0) + 1
        execs += 1
        h = hashlib.sha1(data).hexdigest()[:16]
        new = merge(bm) if bm else 0
        if new:
            new_inputs += 1
            with open(os.path.join(qdir, "id_%s" % h), "wb") as f:
                f.write(data)
            pool.append(data)
        if cls != "match":
            with open(os.path.join(fdir, "%s_%s.xml" % (cls, h)), "wb") as f:
                f.write(data)
            with open(os.path.join(fdir, "%s_%s.json" % (cls, h)), "w") as f:
                json.dump({"class": cls,
                           "c": {k: (v.decode("latin-1")[:4000] if isinstance(v, bytes) else v)
                                 for k, v in c.items()},
                           "rust": {k: (v.decode("latin-1")[:4000] if isinstance(v, bytes) else v)
                                    for k, v in r.items()}}, f, indent=1)
        if execs % 500 == 0 or new or cls != "match":
            log.write(json.dumps({"t": round(time.monotonic() - start, 1), "execs": execs,
                                  "edges": sum(total), "new_inputs": new_inputs,
                                  "counts": counts, "last": cls, "new": new}) + "\n")
            log.flush()
    log.write(json.dumps({"event": "end", "worker": a.worker, "execs": execs,
                          "seed_edges": seed_edges, "edges": sum(total),
                          "new_inputs": new_inputs, "counts": counts}) + "\n")
    log.close()


if __name__ == "__main__":
    main()
