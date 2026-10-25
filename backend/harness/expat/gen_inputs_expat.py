"""Generate differential-test inputs for the libexpat harness.

Stdlib only, deterministic for a given --seed. Writes one file per behavior:
  seed_*   documents from the W3C XML conformance suite (xmlts20130923):
           all of James Clark's xmltest not-wf/sa and valid/sa, plus an
           evenly spaced sample of the sun, ibm, oasis, eduni and japanese
           suites (well-formed and malformed)
  edge_*   hand-written edge cases (entities, char refs, CDATA, comments,
           PIs, attributes, nesting, BOMs, encodings, invalid bytes, DTDs,
           billion laughs)
  mut_*    mutations of the above until --count files exist

Usage: python3 -I gen_inputs_expat.py [--seed 0] [--count 2000] [--out DIR]
                                      [--xmlts DIR] [--max-seeds 600]
"""

import argparse
import hashlib
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PREFIXES = ("seed_", "edge_", "mut_")
SEED_MAX_BYTES = 65536
# Edge cases that are slow under ASan (or huge) and stay out of the mutation pool.
NO_MUTATE = ("edge_bl_breach", "edge_deep_", "edge_wide_", "edge_long_")

TOKENS = [
    b"<", b">", b"</", b"/>", b"&", b";", b"&amp;", b"&lt;", b"&gt;", b"&quot;",
    b"&apos;", b"&#", b"&#x", b"&#60;", b"&#x3C;", b"&#0;", b"&#xD800;", b"&#x10FFFF;",
    b"&#x110000;", b"&undefined;", b"&e;", b"%pe;", b"<!--", b"-->", b"--", b"<![CDATA[",
    b"]]>", b"<?", b"?>", b"<?xml ", b'"', b"'", b"=", b" ", b"\t", b"\r", b"\n", b"\r\n",
    b"\x00", b"\xef\xbb\xbf", b"\xff", b"\xc3", b"\xc3\xa9", b"\xed\xa0\x80",
    b"\xf4\x90\x80\x80", b"<!DOCTYPE", b"<!ENTITY", b"<!ATTLIST", b"<!ELEMENT", b"]>",
    b"[", b"SYSTEM", b"PUBLIC", b"#PCDATA", b"#REQUIRED", b"#IMPLIED", b"#FIXED",
    b"xmlns", b"xml:lang", b":", b"a", b"x=\"1\"",
]
NAMES = [b"a", b"b", b"doc", b"x:y", b"_", b"\xc3\xa9l\xc3\xa9ment", b"a.b-c", b"1bad", b"-bad"]


def billion_laughs(levels, fanout=10):
    """Classic nested-entity amplification document."""
    parts = [b'<?xml version="1.0"?>\n<!DOCTYPE lolz [\n <!ENTITY lol "lol">\n']
    for i in range(1, levels + 1):
        prev = b"lol" if i == 1 else b"lol%d" % (i - 1)
        parts.append(b' <!ENTITY lol%d "%s">\n' % (i, (b"&" + prev + b";") * fanout))
    parts.append(b"]>\n<lolz>&lol%d;</lolz>\n" % levels)
    return b"".join(parts)


def edge_cases():
    """Hand-written edge cases. Returns a list of (name, bytes)."""
    c = {}
    # basics
    c["empty"] = b""
    c["ws_only"] = b" \n\t "
    c["min"] = b"<a/>"
    c["min_text"] = b"<a>hello</a>"
    c["xmldecl_full"] = b'<?xml version="1.0" encoding="UTF-8" standalone="yes"?><a/>'
    c["xmldecl_no"] = b'<?xml version="1.0" standalone="no"?><a/>'
    c["xmldecl_v11"] = b'<?xml version="1.1"?><a/>'
    c["xmldecl_late"] = b' <?xml version="1.0"?><a/>'
    c["xmldecl_bad_version"] = b'<?xml version="2.0"?><a/>'
    c["xmldecl_no_version"] = b'<?xml encoding="UTF-8"?><a/>'
    c["xmldecl_upper"] = b'<?XML version="1.0"?><a/>'
    # entities and character references
    c["predef_entities"] = b"<a>&lt;&gt;&amp;&quot;&apos;</a>"
    c["charref_dec"] = b"<a>&#65;&#233;&#8364;&#128512;</a>"
    c["charref_hex"] = b"<a>&#x41;&#xE9;&#x20AC;&#x1F600;</a>"
    c["charref_zero"] = b"<a>&#0;</a>"
    c["charref_surrogate"] = b"<a>&#xD800;</a>"
    c["charref_too_big"] = b"<a>&#x110000;</a>"
    c["charref_fffe"] = b"<a>&#xFFFE;</a>"
    c["charref_ctrl"] = b"<a>&#1;</a>"
    c["charref_tab_cr"] = b"<a>&#9;&#13;&#10;</a>"
    c["charref_empty"] = b"<a>&#;</a>"
    c["charref_bad_digit"] = b"<a>&#12a;</a>"
    c["undefined_entity"] = b"<a>&nope;</a>"
    c["bare_amp"] = b"<a>a & b</a>"
    c["amp_no_semi"] = b"<a>&amp</a>"
    c["entity_in_attr"] = b'<!DOCTYPE a [<!ENTITY e "v">]><a x="&e;&lt;"/>'
    # internal DTD entity expansion
    c["dtd_entity"] = b'<!DOCTYPE a [<!ENTITY e "hello">]><a>&e; &e;</a>'
    c["dtd_entity_markup"] = b'<!DOCTYPE a [<!ENTITY e "<b>x</b>">]><a>&e;</a>'
    c["dtd_entity_nested"] = b'<!DOCTYPE a [<!ENTITY e1 "1"><!ENTITY e2 "&e1;&e1;">]><a>&e2;</a>'
    c["dtd_entity_recursive"] = b'<!DOCTYPE a [<!ENTITY e "&e;">]><a>&e;</a>'
    c["dtd_entity_mutual"] = b'<!DOCTYPE a [<!ENTITY x "&y;"><!ENTITY y "&x;">]><a>&x;</a>'
    c["dtd_entity_unbalanced"] = b'<!DOCTYPE a [<!ENTITY e "<b>">]><a>&e;</b></a>'
    c["dtd_entity_charref"] = b'<!DOCTYPE a [<!ENTITY e "&#60;b/&#62;">]><a>&e;</a>'
    c["dtd_entity_lt_attr"] = b'<!DOCTYPE a [<!ENTITY e "&#60;">]><a x="&e;"/>'
    c["dtd_entity_redefine"] = b'<!DOCTYPE a [<!ENTITY e "1"><!ENTITY e "2">]><a>&e;</a>'
    c["dtd_external_entity"] = b'<!DOCTYPE a [<!ENTITY e SYSTEM "file:///etc/passwd">]><a>&e;</a>'
    c["dtd_external_entity_attr"] = b'<!DOCTYPE a [<!ENTITY e SYSTEM "x.xml">]><a x="&e;"/>'
    c["dtd_unparsed_entity"] = (b'<!DOCTYPE a [<!NOTATION n SYSTEM "n"><!ENTITY e SYSTEM "x" NDATA n>]>'
                                b"<a>&e;</a>")
    c["dtd_public_entity"] = b'<!DOCTYPE a [<!ENTITY e PUBLIC "-//X//Y" "y.ent">]><a/>'
    c["dtd_pe"] = b'<!DOCTYPE a [<!ENTITY % p "<!ENTITY e \'v\'>"> %p;]><a>&e;</a>'
    c["dtd_pe_decl_only"] = b'<!DOCTYPE a [<!ENTITY % p "x">]><a/>'
    c["dtd_external_subset"] = b'<!DOCTYPE a SYSTEM "a.dtd"><a>&e;</a>'
    c["dtd_external_subset_public"] = b'<!DOCTYPE a PUBLIC "-//A//DTD//EN" "a.dtd"><a/>'
    c["dtd_standalone_undefined"] = (b'<?xml version="1.0" standalone="yes"?>'
                                     b'<!DOCTYPE a SYSTEM "a.dtd"><a>&e;</a>')
    c["dtd_attlist_default"] = b'<!DOCTYPE a [<!ATTLIST a x CDATA "dflt" y CDATA #IMPLIED>]><a/>'
    c["dtd_attlist_fixed"] = b'<!DOCTYPE a [<!ATTLIST a x CDATA #FIXED "f">]><a z="1"/>'
    c["dtd_attlist_id"] = b'<!DOCTYPE a [<!ATTLIST a i ID #IMPLIED>]><a i="  x  y "/>'
    c["dtd_attlist_nmtokens"] = b'<!DOCTYPE a [<!ATTLIST a t NMTOKENS #IMPLIED>]><a t="  p   q  "/>'
    c["dtd_element_decl"] = b"<!DOCTYPE a [<!ELEMENT a (b*|c)+><!ELEMENT b EMPTY><!ELEMENT c ANY>]><a><b/></a>"
    c["dtd_comment_pi"] = b"<!DOCTYPE a [<!-- c --><?p d?>]><a/>"
    c["dtd_unclosed"] = b'<!DOCTYPE a [<!ENTITY e "v">'
    c["dtd_after_root"] = b"<a/><!DOCTYPE a>"
    c["dtd_twice"] = b"<!DOCTYPE a><!DOCTYPE a><a/>"
    c["bl_small"] = billion_laughs(4)
    c["bl_medium"] = billion_laughs(5)
    c["bl_breach"] = billion_laughs(9)  # trips the amplification limit
    c["bl_attr"] = billion_laughs(4).replace(b"<lolz>&lol4;</lolz>", b'<lolz a="&lol4;"/>')
    c["quadratic_blowup"] = (b'<!DOCTYPE a [<!ENTITY x "' + b"x" * 2000 + b'">]><a>'
                             + b"&x;" * 2000 + b"</a>")
    # CDATA, comments, PIs
    c["cdata"] = b"<a><![CDATA[<b>&amp;]]]]></a>"
    c["cdata_empty"] = b"<a><![CDATA[]]></a>"
    c["cdata_unclosed"] = b"<a><![CDATA[x</a>"
    c["cdata_outside"] = b"<![CDATA[x]]><a/>"
    c["cdata_end_in_text"] = b"<a>x]]>y</a>"
    c["comment"] = b"<!-- top --><a><!-- in --></a><!-- after -->"
    c["comment_dashes"] = b"<a><!-- a -- b --></a>"
    c["comment_end_dash"] = b"<a><!-- a ---></a>"
    c["comment_unclosed"] = b"<a><!-- x </a>"
    c["pi"] = b"<?p data here?><a><?q?><?r  spaced  ?></a>"
    c["pi_xml_reserved"] = b"<a><?xml version='1.0'?></a>"
    c["pi_xmlfoo"] = b"<a><?xml-stylesheet href='x'?></a>"
    c["pi_no_target"] = b"<a><? x?></a>"
    c["pi_unclosed"] = b"<a><?p x</a>"
    # attributes
    c["attrs_order"] = b'<a z="1" y="2" x="3" b=\'4\'/>'
    c["attr_duplicate"] = b'<a x="1" x="2"/>'
    c["attr_unquoted"] = b"<a x=1/>"
    c["attr_amp"] = b'<a x="a & b"/>'
    c["attr_lt"] = b'<a x="a < b"/>'
    c["attr_gt"] = b'<a x="a > b"/>'
    c["attr_ws_normalize"] = b'<a x=" a\tb\nc\r\nd "/>'
    c["attr_charref_ws"] = b'<a x="&#9;&#10;&#13;&#32;"/>'
    c["attr_no_value"] = b"<a x/>"
    c["attr_no_space"] = b'<a x="1"y="2"/>'
    c["attr_quote_mix"] = b'<a x="it\'s" y=\'say "hi"\'/>'
    c["attr_many"] = b"<a " + b" ".join(b'k%d="%d"' % (i, i) for i in range(300)) + b"/>"
    c["xmlns_plain"] = b'<a xmlns="urn:x" xmlns:p="urn:p"><p:b p:c="1"/></a>'
    c["colon_names"] = b"<a:b:c d:e:f='1'/>"
    # structure
    c["unclosed_tag"] = b"<a><b>"
    c["unclosed_start"] = b"<a"
    c["mismatched"] = b"<a><b></a></b>"
    c["two_roots"] = b"<a/><b/>"
    c["text_after_root"] = b"<a/>junk"
    c["text_before_root"] = b"junk<a/>"
    c["no_root"] = b"<!-- only -->"
    c["end_only"] = b"</a>"
    c["lt_in_text"] = b"<a>1 < 2</a>"
    c["gt_in_text"] = b"<a>2 > 1</a>"
    c["ws_after_root"] = b"<a/>\n\n  \t"
    c["ws_in_end_tag"] = b"<a></a  >"
    c["bad_name_start"] = b"<1a/>"
    c["empty_name"] = b"<>x</>"
    for depth in (100, 5000, 100000):
        c["deep_%d" % depth] = b"<a>" * depth + b"</a>" * depth
    c["deep_unclosed_5000"] = b"<a>" * 5000
    c["wide_10000"] = b"<r>" + b"<c/>" * 10000 + b"</r>"
    c["long_text"] = b"<a>" + b"x" * 100000 + b"</a>"
    c["long_name"] = b"<" + b"n" * 5000 + b"/>"
    # line ends and positions
    c["crlf"] = b"<a>\r\nx\r\ny\r</a>"
    c["lines_error"] = b"<a>\n<b>\n\n  <c></b>\n</a>"
    c["cr_error"] = b"<a>\r\r\r<b></a>"
    c["multibyte_col_error"] = b"<a>\xc3\xa9\xc3\xa9\xc3\xa9</b>"
    # BOMs and encodings
    doc = '<?xml version="1.0" encoding="%s"?><a x="é">café €</a>'
    c["bom_utf8"] = b"\xef\xbb\xbf<a>x</a>"
    c["bom_utf8_decl"] = b'\xef\xbb\xbf<?xml version="1.0" encoding="UTF-8"?><a/>'
    c["bom_utf8_twice"] = b"\xef\xbb\xbf\xef\xbb\xbf<a/>"
    c["bom_utf8_decl_mismatch"] = b'\xef\xbb\xbf<?xml version="1.0" encoding="ISO-8859-1"?><a/>'
    c["utf16le_bom"] = b"\xff\xfe" + (doc % "UTF-16").encode("utf-16-le")
    c["utf16be_bom"] = b"\xfe\xff" + (doc % "UTF-16").encode("utf-16-be")
    c["utf16le_no_decl"] = b"\xff\xfe" + "<a>é\U0001F600</a>".encode("utf-16-le")
    c["utf16le_no_bom"] = "<a>x</a>".encode("utf-16-le")
    c["utf16be_no_bom"] = "<a>x</a>".encode("utf-16-be")
    c["utf16le_odd_byte"] = b"\xff\xfe" + "<a>x</a>".encode("utf-16-le") + b"<"
    c["utf16le_lone_surrogate"] = b"\xff\xfe" + "<a>".encode("utf-16-le") + b"\x00\xd8" + "</a>".encode("utf-16-le")
    c["utf16_decl_utf8_body"] = (doc % "UTF-16").encode("utf-8")
    c["latin1"] = (doc % "ISO-8859-1").encode("latin-1", errors="replace")
    c["latin1_lower"] = (doc % "iso-8859-1").encode("latin-1", errors="replace")
    c["latin1_high"] = b'<?xml version="1.0" encoding="ISO-8859-1"?><a>' + bytes(range(0xa0, 0x100)) + b"</a>"
    c["ascii_ok"] = b'<?xml version="1.0" encoding="US-ASCII"?><a>plain</a>'
    c["ascii_high"] = b'<?xml version="1.0" encoding="US-ASCII"?><a>\xe9</a>'
    c["utf8_declared"] = (doc % "utf-8").encode("utf-8")
    c["unknown_encoding"] = b'<?xml version="1.0" encoding="EBCDIC-XYZ"?><a/>'
    c["windows1252"] = b'<?xml version="1.0" encoding="windows-1252"?><a>\x80</a>'
    c["encoding_empty"] = b'<?xml version="1.0" encoding=""?><a/>'
    c["encoding_bad_chars"] = b'<?xml version="1.0" encoding="UTF 8"?><a/>'
    # invalid bytes
    c["invalid_utf8_ff"] = b"<a>\xff</a>"
    c["invalid_utf8_trunc"] = b"<a>\xc3</a>"
    c["invalid_utf8_trunc_eof"] = b"<a>x</a>\xe2\x82"
    c["invalid_utf8_overlong"] = b"<a>\xc0\xaf</a>"
    c["invalid_utf8_surrogate"] = b"<a>\xed\xa0\x80</a>"
    c["invalid_utf8_too_big"] = b"<a>\xf4\x90\x80\x80</a>"
    c["invalid_utf8_in_name"] = b"<a\xff/>"
    c["invalid_utf8_in_attr"] = b'<a x="\xfe"/>'
    c["utf8_4byte"] = b"<a>\xf0\x9f\x98\x80</a>"
    c["utf8_fffe"] = b"<a>\xef\xbf\xbe</a>"
    c["nul_in_text"] = b"<a>x\x00y</a>"
    c["nul_after_root"] = b"<a/>\x00"
    c["nul_only"] = b"\x00"
    c["ctrl_char"] = b"<a>\x01</a>"
    c["del_char"] = b"<a>\x7f</a>"
    c["c1_char"] = b"<a>\xc2\x85</a>"
    return sorted(c.items())


def load_xmlts(xmlts_dir, max_seeds):
    """Deterministic selection of conformance-suite documents."""
    root = os.path.join(xmlts_dir, "xmlconf")
    if not os.path.isdir(root):
        sys.exit("missing %s; run fetch_expat.sh first" % root)
    primary, rest = [], []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        rel_dir = os.path.relpath(dirpath, root)
        if "/out" in "/" + rel_dir:  # canonical outputs, not test documents
            continue
        for name in sorted(filenames):
            if not name.endswith(".xml"):
                continue
            path = os.path.join(dirpath, name)
            if os.path.getsize(path) > SEED_MAX_BYTES:
                continue
            rel = os.path.relpath(path, root)
            if rel.startswith(("xmltest/not-wf/sa/", "xmltest/valid/sa/")):
                primary.append(rel)
            elif rel_dir != "." and not name.startswith(("xmlconf", "xmltest")):
                rest.append(rel)
    primary.sort()
    rest.sort()
    want = max(0, max_seeds - len(primary))
    step = max(1, len(rest) // want) if want else len(rest) + 1
    chosen = primary + rest[::step][:want]
    seeds = []
    for rel in chosen:
        with open(os.path.join(root, rel), "rb") as fh:
            seeds.append((rel.replace("/", "_").replace(".", "_"), fh.read()))
    return seeds


def mutate(rng, data, pool):
    """Apply one random structured or byte-level mutation."""
    op = rng.randrange(12)
    n = len(data)
    pos = rng.randint(0, n)
    if op == 0 and n:                        # flip one bit
        i = rng.randrange(n)
        return data[:i] + bytes([data[i] ^ (1 << rng.randrange(8))]) + data[i + 1:]
    if op == 1:                              # insert an XML-ish token
        return data[:pos] + rng.choice(TOKENS) + data[pos:]
    if op == 2 and n:                        # delete a span
        end = min(n, pos + rng.randint(1, 8))
        return data[:pos] + data[end:]
    if op == 3 and n:                        # duplicate a span
        a = rng.randrange(n)
        b = min(n, a + rng.randint(1, 24))
        return data[:pos] + data[a:b] + data[pos:]
    if op == 4:                              # splice two inputs
        other = rng.choice(pool)
        cut = rng.randint(0, len(other))
        return data[:pos] + other[cut:]
    if op == 5 and n:                        # truncate
        return data[:rng.randrange(n)]
    if op == 6:                              # wrap in an element
        name = rng.choice(NAMES)
        return b"<" + name + b">" + data + b"</" + name + b">"
    if op == 7 and n:                        # random byte overwrite
        i = rng.randrange(n)
        return data[:i] + bytes([rng.randrange(256)]) + data[i + 1:]
    if op == 8:                              # rename one tag (often a mismatch)
        starts = [i for i in range(n - 1) if data[i:i + 1] == b"<" and data[i + 1:i + 2].isalpha()]
        if starts:
            i = rng.choice(starts) + 1
            j = i
            while j < n and (data[j:j + 1].isalnum() or data[j:j + 1] in b"_-.:"):
                j += 1
            return data[:i] + rng.choice(NAMES) + data[j:]
    if op == 9:                              # insert an attribute into a start tag
        starts = [i for i in range(n - 1) if data[i:i + 1] == b"<" and data[i + 1:i + 2].isalpha()]
        if starts:
            i = rng.choice(starts) + 1
            while i < n and data[i:i + 1] not in b" />":
                i += 1
            attr = rng.choice([b' q="1"', b' q="&amp;"', b" q='<'", b' q="&#x41;"', b" q", b' q="a\tb"'])
            return data[:i] + attr + data[i:]
    if op == 10:                             # nest a value a few levels deep
        depth = rng.randint(1, 30)
        return b"<n>" * depth + data + b"</n>" * depth
    # prepend a random prolog piece
    prolog = rng.choice([b'<?xml version="1.0"?>', b"\xef\xbb\xbf", b"<!-- p -->",
                         b'<!DOCTYPE n [<!ENTITY e "v">]>', b"<?pi x?>", b" ",
                         b'<?xml version="1.0" encoding="ISO-8859-1"?>'])
    return prolog + data


def main(argv=None):
    ap = argparse.ArgumentParser(description="Generate libexpat differential inputs")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--count", type=int, default=2000, help="total files to write")
    ap.add_argument("--out", default=os.path.join(HERE, "inputs"))
    ap.add_argument("--xmlts", default=os.path.join(HERE, "vendor", "xmlts"))
    ap.add_argument("--max-seeds", type=int, default=600,
                    help="conformance-suite documents to include")
    ap.add_argument("--max-input-bytes", type=int, default=262144,
                    help="drop mutations larger than this")
    args = ap.parse_args(argv)

    rng = random.Random(args.seed)
    os.makedirs(args.out, exist_ok=True)
    for name in os.listdir(args.out):  # clear only files we generated earlier
        if name.startswith(PREFIXES):
            os.remove(os.path.join(args.out, name))

    seen = set()
    written = []
    pool = []

    def emit(name, data):
        digest = hashlib.sha256(data).digest()
        if digest in seen:
            return False
        seen.add(digest)
        with open(os.path.join(args.out, name), "wb") as fh:
            fh.write(data)
        written.append(data)
        return True

    for name, data in load_xmlts(args.xmlts, args.max_seeds):
        if emit("seed_%s" % name, data):
            pool.append(data)
    n_seeds = len(written)
    for name, data in edge_cases():
        full = "edge_%s" % name
        if emit(full, data) and not full.startswith(NO_MUTATE):
            pool.append(data)

    n_base = len(written)
    i, attempts = 0, 0
    while len(written) < args.count and attempts < args.count * 50:
        attempts += 1
        data = rng.choice(pool)
        for _ in range(rng.randint(1, 3)):
            data = mutate(rng, data, pool)
        if len(data) > args.max_input_bytes:
            continue
        if emit("mut_%05d" % i, data):
            i += 1
    print("wrote %d inputs to %s (%d xmlts seeds, %d edge cases, %d mutations, seed %d)"
          % (len(written), args.out, n_seeds, n_base - n_seeds, i, args.seed))
    return 0


if __name__ == "__main__":
    sys.exit(main())
