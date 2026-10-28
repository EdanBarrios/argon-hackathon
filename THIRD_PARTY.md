# Third-party inputs

Theseus does not commit any third-party source. These are fetched at build time into gitignored directories and are used as the C programs being migrated and as test corpora.

| Project | Version | License | Used for |
|---|---|---|---|
| [cJSON](https://github.com/DaveGamble/cJSON) | v1.7.19 (pinned commit in `backend/harness/fetch_cjson.sh`), v1.7.17 for the CVE-2023-53154 check | MIT | C library migrated to Rust; differential oracle |
| [libexpat](https://github.com/libexpat/libexpat) | R_2_9_0 (R_2_6_4 for the CVE replay) | MIT | C library migrated to Rust; differential oracle |
| [Lua](https://www.lua.org/) | 5.4.7 | MIT | C interpreter translated to Rust (translation only, not yet verified) |
| W3C XML Conformance Test Suite | as fetched by `backend/harness/expat/` | W3C Software and Document License | Test inputs for libexpat |

Rust code in `backend/ai/results/` was written by Gemini models through the Theseus pipeline. It is derived from the MIT-licensed C above and is kept as a record of results, not as a supported library.
