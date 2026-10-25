"""Add download provenance to the similarity JSON. Usage: python3 -I scripts_meta.py IN OUT"""
import json
import sys

d = json.load(open(sys.argv[1]))
d["sources"] = {
    "argon": {"path": "backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs",
              "sha256": "418570361191d0019294b981d80e3185c7a732e0d4c02a627107f60b5af72c0d"},
    "argon_round1": {"path": "backend/ai/results/expat-long-decode/argon-fresh-a/round_1/expat.rs",
                     "sha256_prefix": "760bc3e40442bfac",
                     "role": "copy anchor: same model output before 3 lines of compile fixes"},
    "expat_crate": {"crate": "expat", "version": "0.1.0-beta.1", "only_published_version": True,
                    "published": "2026-10-18T11:35:35Z", "publisher": "chemicalpeace",
                    "repository": "https://github.com/chemicalpeace/expat-parity",
                    "vcs_sha1": "bfe5373b4c86a7fd1c43f7630ba405ecacf44475",
                    "crate_sha256": "c0dc55ad7938332ca7356b51c68f033cb53479b0a1a3928c088d4c6a08f2d572",
                    "checksum_matches_registry": True},
    "xml_rs": {"crate": "xml-rs", "version": "0.8.29",
               "crate_sha256": "e450f9b2ed1dff33c94c12589a87338689467b9c4f5d8a5710bd09a847d2c8a7"},
    "quick_xml": {"crate": "quick-xml", "version": "0.42.0",
                  "crate_sha256": "41b1177fdf999d2321d3fb46ff47159d9c1fb9ad66a4879f8c50a0b504615e9b"},
}
d["method_notes"] = [
    "Rust sources only (src/, excluding tests/, tests.rs and #[cfg(test)] modules); comments stripped.",
    "Lines: whitespace-collapsed, >= 30 chars, excluding use/mod/attribute/brace-only lines; unique-line sets compared.",
    "Identifiers: names declared with fn/struct/enum/trait/type/const/static, compared after lowercasing and removing underscores (so do_prolog == doProlog).",
    "shared_identifiers_also_in_libexpat_c: shared names that also appear (same canonical form) as identifiers in libexpat 2.9.0 lib/*.c,*.h.",
    "Token 8-grams: Jaccard of unique 8-token shingle sets; containment_of_a = shared / a's 8-grams.",
    "code_only_*: same metrics after removing array literals longer than 300 chars (data tables such as nametab.h bitmaps).",
    "longest_common_token_run is capped at 4000 tokens.",
    "Downloaded crates were extracted into their own empty directories and only read as text; nothing was built or run.",
]
json.dump(d, open(sys.argv[2], "w"), indent=1)
