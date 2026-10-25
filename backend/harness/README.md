# backend/harness

Owner: **C (systems)**. Differential verification: builds C (with ASan) and Rust, runs both on the same generated inputs, emits JSON (match rate, divergences, crashes). See docs/PLAN.md section 3.

## Self-test and verify.sh

Target: cJSON v1.7.19, core `cJSON.c` only, module 1 = parse + print.

```bash
scripts/selftest.sh            # fetch, build, generate 2000 inputs, verify C against C
scripts/selftest.sh --count 300 --seed 1   # quicker run
```

It exits nonzero unless `match_rate` is 1.0 with 0 divergences, and saves a report to `runs/selftest-<stamp>.json`. Run it before trusting any Rust number: if C against C is not a perfect match, the harness is broken.

Pieces (all under `backend/harness/`):

- `fetch_cjson.sh`: shallow-clones cJSON v1.7.19 into `vendor/cJSON` and checks the pinned commit. Idempotent.
- `build_cjson.sh`: builds `build/cjson_driver` with `clang -g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer`.
- `drivers/cjson_driver.c`: reads stdin, prints one line, `OK\t<cJSON_PrintUnformatted output>` or `PARSE_ERROR at <offset>`. Always exits 0 unless it crashes. A Rust driver must follow the same protocol.
- `gen_inputs.py`: deterministic inputs (`--seed`, `--count`) into `inputs/cjson/`: cJSON's own test and fuzz seeds, hand-written edge cases, then mutations.
- `verify.sh` / `verify.py`: the differential verifier.

```bash
backend/harness/verify.sh --c backend/harness/build/cjson_driver \
  --rust "path/to/rust_driver" --inputs backend/harness/inputs/cjson \
  [--timeout 5] [--max-bytes 1048576] [--out runs/<name>.json] [--jobs N]
```

Each input goes to both commands on stdin with `LC_ALL=C`. Raw stdout bytes and exit codes are compared, never re-parsed as JSON. Classes: `match`, `divergence`, `safety_improvement` (C crashed or a sanitizer fired, Rust exited cleanly, not counted as a mismatch), `rust_crash` (signal, sanitizer or panic), `both_crash`, `timeout`.

`match_rate = match / (match + divergence + rust_crash + both_crash + timeout)`. `--max-bytes` skips input files larger than the limit (listed under `skipped_inputs`). Commands are split with shell quoting rules, not run through a shell.

Speed: each ASan process costs about 0.2 s to start on macOS, so 2000 inputs take about 3 to 4 minutes on 8 cores. Use `--count 300` for repair loops.
