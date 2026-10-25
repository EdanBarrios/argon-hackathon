# oldcve: real ASan crash for the Theseus demo

A reproducible AddressSanitizer crash in an **old, vulnerable cJSON** parse
path, for the "C crashes, safe Rust returns a clean error" demo moment. The
current target (cJSON v1.7.19) does not crash on this input; an older release
does.

## The CVE

**CVE-2023-53154** - heap-based buffer over-read (CWE-125, out-of-bounds read)
in cJSON's object/string parse path.

- Last vulnerable release: **v1.7.17** (commit `87d8f0961a01bf09bef98ff89bae9fdec42181ee`)
- Fixed in: **v1.7.18**, by commit
  [`3ef4e4e730e5efd381be612df41e1ff3f5bb3c32`](https://github.com/DaveGamble/cJSON/commit/3ef4e4e730e5efd381be612df41e1ff3f5bb3c32)
  (adds a `cannot_access_at_index(input_buffer, 1)` check in `parse_object`
  before advancing past a trailing comma).
- Sources:
  - NVD: https://nvd.nist.gov/vuln/detail/CVE-2023-53154
  - MITRE: https://cveawg.mitre.org/api/cve/CVE-2023-53154
  - Upstream issue (tagged exploit): https://github.com/DaveGamble/cJSON/issues/800
  - Fix PR: https://github.com/DaveGamble/cJSON/pull/852
  - Version diff: https://github.com/DaveGamble/cJSON/compare/v1.7.17...v1.7.18

### What the bug is, in plain words

cJSON's `parse_object` reads a comma inside an object, then unconditionally
advances past it and starts parsing the next member name without first checking
that any byte actually follows the comma. If the input is a JSON object that
ends right after a comma and the buffer has no readable byte at that position
(you parsed it with `cJSON_ParseWithLength`, giving the exact length and no NUL
terminator), the parser reads one byte past the end of the heap buffer.
v1.7.18 adds the missing bounds check.

## The crashing input

File: `inputs/cve_2023_53154.json` - 7 bytes, no trailing newline, no NUL.

```
hex:   7b 22 31 22 3a 31 2c
ascii: {  "  1  "  :  1  ,
```

This is the exact PoC from the advisory: `{"1":1,`.

## ASan report (old driver, cJSON v1.7.17), first lines

```
==NNNNN==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x6020000000d7 ...
READ of size 1 at 0x6020000000d7 thread T0
    #0 ... in parse_string cJSON.c:786
    #1 ... in parse_object cJSON.c:1665
    #2 ... in parse_value cJSON.c:1365
    #3 ... in cJSON_ParseWithLengthOpts cJSON.c:1125
    #4 ... in main cjson_driver_old.c:89
0x6020000000d7 is located 0 bytes after 7-byte region [0x6020000000d0,0x6020000000d7)
allocated by thread T0 here:
    #0 ... in malloc
    #1 ... in main cjson_driver_old.c:77
SUMMARY: AddressSanitizer: heap-buffer-overflow cJSON.c:786 in parse_string
```

`SUMMARY: AddressSanitizer: heap-buffer-overflow cJSON.c:786 in parse_string`

The read is exactly 0 bytes past the 7-byte allocation, which is the input
buffer sized to the exact length of the PoC.

## v1.7.19 behavior on the same input (clean)

The existing v1.7.19 driver (`backend/harness/build/cjson_driver`):

```
PARSE_ERROR at 7
exit 0
```

To prove it is the *library* fix and not a driver difference, this same old
driver source compiled against v1.7.19 also returns `PARSE_ERROR` cleanly with
no ASan report. v1.7.18 is where the bounds check was added.

## Driver difference (documented)

The existing v1.7.19 driver appends a NUL and calls
`cJSON_ParseWithLengthOpts(buf, len + 1, &end, /*require_null_terminated=*/1)`.
That extra readable NUL byte gives the parser a sentinel and hides this bug
even on a vulnerable library.

CVE-2023-53154 is specifically about `cJSON_ParseWithLength` on a buffer that
is **not** NUL-terminated at the stated length. So `cjson_driver_old.c` makes
exactly one change from the baseline driver: it copies stdin into a heap buffer
of **exactly** `len` bytes (ASan redzone immediately after) and calls
`cJSON_ParseWithLengthOpts(buf, len, &end, /*require_null_terminated=*/0)`,
with no appended NUL. Same stdin/stdout protocol otherwise. This is the
faithful trigger path for the CVE.

## Reproduce

```bash
# 1. Build the old (vulnerable) driver; clones + pins cJSON v1.7.17.
bash backend/harness/oldcve/build_old.sh

# 2. See the old version crash.
backend/harness/oldcve/build/cjson_driver_old < backend/harness/oldcve/inputs/cve_2023_53154.json

# 3. See v1.7.19 handle it cleanly (build it first if needed:
#    bash backend/harness/fetch_cjson.sh && bash backend/harness/build_cjson.sh)
backend/harness/build/cjson_driver < backend/harness/oldcve/inputs/cve_2023_53154.json

# 4. Differential verifier: classifies the input as safety_improvement.
#    NOTE: use --timeout 20. macOS ASan symbolization (atos) takes ~5s, which
#    trips the default 5s timeout and misclassifies the crash as a timeout.
backend/harness/verify.sh \
  --c backend/harness/oldcve/build/cjson_driver_old \
  --rust backend/harness/build/cjson_driver \
  --inputs backend/harness/oldcve/inputs/ \
  --timeout 20
```

Verifier result: `counts.safety_improvement = 1`, `rust_behavior = "exit 0,
stdout: PARSE_ERROR at 7"`.
