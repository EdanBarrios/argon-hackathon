# libexpat: is Argon's port a copy of the published `expat` crate?

Answers the critique "Argon may have memorized an existing Rust port of expat". Measured by the systems side (C) on 2026-10-25 with text-similarity metrics. Nothing downloaded was built or run.

## What we compared

| Corpus | Source | Size (Rust, comments and tests removed) |
|---|---|---|
| **argon** | Argon's port, `backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs` (sha256 `41857036...`) | 1 file, 36,969 tokens, 1,182 non-trivial lines, 332 declared names |
| **expat crate** | crates.io `expat` 0.1.0-beta.1, the only published version (published 2026-10-18 by `chemicalpeace`, repo `chemicalpeace/expat-parity`, "Memory-safe Rust port of the libexpat XML parser"). Crate sha256 matches the registry checksum. | 33 files, 51,494 tokens, 2,945 lines, 640 names |
| **xml-rs** | crates.io `xml-rs` 0.8.29, an unrelated Rust XML parser (baseline) | 29 files, 32,821 tokens |
| **quick-xml** | crates.io `quick-xml` 0.42.0, an unrelated Rust XML parser (baseline) | 36 files, 83,441 tokens |
| **argon round 1** | Argon's own round-1 output, which differs from round 3 by 3 lines of compile fixes (copy anchor: what a copy looks like) | 1 file, 36,950 tokens |

## Results

"of first" means as a share of the first corpus in the pair. Code-only metrics drop array literals over 300 characters (data tables).

| Pair | Shared lines (of first) | Shared names (of first) | Name Jaccard | Shared names also in libexpat C | 8-gram Jaccard | 8-gram containment (of first) | Code-only 8-gram Jaccard / containment | Longest common token run (code-only) |
|---|---|---|---|---|---|---|---|---|
| argon vs expat crate | 42 (3.6%) | 97 (29.3%) | 0.116 | 91 of 97 | **0.027** | **6.9%** | 0.015 / 3.8% | 652 (74) |
| argon vs xml-rs | 0 | 4 (1.2%) | 0.007 | 4 of 4 | 0.0007 | 0.16% | 0.0007 / 0.16% | 11 (11) |
| argon vs quick-xml | 0 | 6 (1.8%) | 0.007 | 6 of 6 | 0.0011 | 0.38% | 0.0011 / 0.40% | 17 (17) |
| expat crate vs xml-rs | 2 (0.07%) | 25 (4.2%) | 0.031 | 18 of 25 | 0.0033 | 0.56% | 0.0033 / 0.58% | 31 (31) |
| expat crate vs quick-xml | 1 (0.03%) | 29 (4.8%) | 0.027 | 23 of 29 | 0.0027 | 0.68% | 0.0027 / 0.70% | 21 (21) |
| xml-rs vs quick-xml | 4 (0.3%) | 34 (14.4%) | 0.049 | 16 of 34 | 0.009 | 2.8% | 0.009 / 2.8% | 145 (145) |
| argon vs argon round 1 (copy anchor) | 1,181 (99.9%) | 331 (99.7%) | 1.0 | | **0.997** | **99.8%** | 0.997 / 99.8% | 4,000+ (capped) |

## Reading it

- Argon's port is far more similar to the expat crate than to unrelated XML parsers (8-gram Jaccard 0.027 vs about 0.001). That is expected: both translate the same C file by file, so they inherit libexpat's names, constants and bit arithmetic.
- The overlap that exists traces to the C source. 91 of the 97 shared names are libexpat's own identifiers in snake case (`do_prolog`, `store_atts`, `scan_lt`, `normalize_public_id`, ...); 5 of the other 6 are C names with a prefix dropped (`trim_to_complete_utf8_characters` from `_INTERNAL_trim_to_complete_utf8_characters`, `store_string` from `poolStoreString`, `token_role` from `XmlTokenRole`, `diff_tolerated`, `current_amplification`), and the last is the Rust idiom `entity_mut`. The longest shared run, 652 tokens, is the `NAMING_BITMAP` table from libexpat's `nametab.h`. With data tables removed, the longest shared run is 74 tokens: the UTF-16 surrogate to UTF-8 bit math from `xmltok.c` line 624. The 42 shared lines are short C-derived statements like `self.dtd.keep_processing = self.dtd.standalone;` and `out.push((plane >> 2) | 0xF0);` (all listed in `provenance-summary.json`).
- It is nowhere near a copy. 93% of Argon's 8-token sequences (96% with tables removed) and 96% of its non-trivial lines do not appear anywhere in the crate. The copy anchor shows what a copy scores: 0.997.
- Structure differs too: the crate is 33 modules with a `Handler` trait and chunked input; Argon's port is one file implementing our driver interface (`backend/harness/expat/rust_crate/INTERFACE.md`).
- Timeline: libexpat 2.9.0 was released 2026-10-05 and this crate, a port of 2.9.0, was first published 2026-10-18. We do not know Argon's training cutoff, so we do not argue from dates.

## What this does and does not show

- It shows Argon's output is not a copy or a lightly edited copy of the published crate, at the level of lines, names or 8-token sequences.
- It does not prove Argon never saw the crate or its GitHub repo in training. Low textual overlap is compatible with exposure; it rules out reproduction, not influence.
- Missing calibration: we have no second independent port of the same C to show the "same source, no copying" level of overlap. All of Argon's runs produced the same file (identical hashes across `argon-fresh-a`, `argon-fresh-b` and `gemini-4-argon` round 1), so they cannot serve as one. The only same-source pair is argon vs the crate itself.
- The metrics are text-level (normalized lines, declared names, token shingles). They would miss a copy that was systematically renamed and reformatted; the shared-name and structure differences make that unlikely but this method does not exclude it.
- Only the published crate was compared, not the GitHub repo history.

Honest one-liner: Argon's port shares 7% of its 8-token sequences and 3.6% of its lines with the only published Rust expat port, almost all of it traceable to the C both translate, against 99.8% for an actual copy; that rules out copying, not training exposure.

## Files

- `provenance-summary.json`: all pairs with every shared line and shared name, longest common runs, crate versions, sha256s and method notes.
- `scripts/similarity.py` (metrics, reads files as text only) and `scripts/add_meta.py`. Run with `python3 -I`, from outside the download directories.
