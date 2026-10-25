# backend/harness

Owner: **C (systems)**. Differential verification: builds C (with ASan) and Rust, runs both on the same generated inputs, emits JSON (match rate, divergences, crashes). See docs/PLAN.md section 3.

## Self-test and verify.sh

Target: cJSON v1.7.19, core `cJSON.c` only, module 1 = parse + print.

```bash
scripts/selftest.sh            # fetch, build, generate 2000 inputs, verify C against C
scripts/selftest.sh --count 300 --seed 1   # quicker run
```

It exits nonzero unless `match_rate` is 1.0 with 0 divergences, and saves a report to `runs/selftest-<stamp>.json`. It then runs the `-O1` driver against a `-O0` build of the same code (`runs/selftest-<stamp>-O0.json`) and fails, listing the inputs, unless every input matches: a difference there means cJSON's behaviour depends on the optimiser, i.e. undefined behaviour the sanitizers did not catch. Run it before trusting any Rust number: if C against C is not a perfect match, the harness is broken.

Pieces (all under `backend/harness/`):

- `fetch_cjson.sh`: shallow-clones cJSON v1.7.19 into `vendor/cJSON` and checks the pinned commit. Idempotent.
- `build_cjson.sh [OUTPUT]`: builds `build/cjson_driver` (or `OUTPUT`) with `clang -g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer`. `OPT=-O0` changes the optimisation level; `CJSON_C=<path>` compiles a different `cJSON.c` (used by `mutants.py`).
- `drivers/cjson_driver.c`: reads stdin, prints one line, `OK\t<cJSON_PrintUnformatted output>` or `PARSE_ERROR at <offset>`. Always exits 0 unless it crashes. A Rust driver must follow the same protocol.
- `gen_inputs.py`: deterministic inputs (`--seed`, `--count`) into `inputs/cjson/`: cJSON's own test and fuzz seeds, hand-written edge cases, then mutations.
- `verify.sh` / `verify.py`: the differential verifier.

```bash
backend/harness/verify.sh --c backend/harness/build/cjson_driver \
  --rust "path/to/rust_driver" --inputs backend/harness/inputs/cjson \
  [--timeout 5] [--max-bytes 1048576] [--out runs/<name>.json] [--jobs N]
```

Each input goes to both commands on stdin with `LC_ALL=C`. Raw stdout bytes and exit codes are compared, never re-parsed as JSON. Classes: `match`, `divergence`, `safety_improvement` (C crashed or a sanitizer fired, Rust exited cleanly, not counted as a mismatch), `rust_crash` (signal, sanitizer or panic), `both_crash`, `timeout`. A side that printed a sanitizer report and only then hit `--timeout` (macOS symbolization can take seconds) counts as crashed, not timed out; its crash entry has `timed_out_after_report: true`.

`match_rate = match / (match + divergence + rust_crash + both_crash + timeout)`. `--max-bytes` skips input files larger than the limit (listed under `skipped_inputs`). Commands are split with shell quoting rules, not run through a shell.

Speed: each ASan process costs about 0.2 s to start on macOS, so 2000 inputs take about 3 to 4 minutes on 8 cores. Use `--count 300` for repair loops.

## Mutation sensitivity check

The self-test shows the verifier reports no false divergences. `mutants.py` shows the other direction: that the verifier plus the generated inputs catch a real bug in C.

```bash
python3 -I backend/harness/mutants.py --inputs backend/harness/inputs/cjson [--timeout 5] [--jobs N]
```

For each of a fixed list of single-site mutations in cJSON's parse/print path (flipped comparisons, off-by-one bounds, changed constants such as the nesting limit or `%1.15g`, a dropped escape case, a narrower whitespace test, and so on), it copies the vendored `cJSON.c` to a temp dir, applies the one exact-string substitution (it errors out, naming the mutation, if a site does not match exactly once), builds a driver with `build_cjson.sh`, and runs `verify.sh` with the unmutated driver against the mutant. A mutant is KILLED if any input does not match, SURVIVED otherwise. It prints one JSON report (`mutants`, `killed`, `survived`, `kill_rate`, per-mutant counts and example inputs), lists survivors on stderr and exits 1 if any survived. A survivor is not a harness bug: it names behaviour the generated inputs never exercise. About 45 s for 300 inputs on 8 cores.

## C coverage

```bash
scripts/coverage.sh [--inputs backend/harness/inputs/cjson]
```

Builds an instrumented driver (`-fprofile-instr-generate -fcoverage-mapping`, no sanitizers), runs every input through it, merges the profiles with `xcrun llvm-profdata` and prints one JSON line: line, branch, function and region coverage of `cJSON.c`, plus region and branch coverage per parse/print function. Whole-file numbers are low because `cJSON.c` is mostly API the driver never calls; the per-function numbers are the useful ones. Reported only, never a gate.
