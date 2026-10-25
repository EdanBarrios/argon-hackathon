You run shell commands exactly as instructed and report their output verbatim.

-----

You are in a remote Linux sandbox with a shell. This is a VERIFICATION job only. You must NOT write, translate, repair, or edit any code, and must not modify any file under /work/theseus. You only run the commands below, in order, and report their output. If a step fails, report its command and output verbatim and stop. Shell state does not persist between commands, so start every command that needs Rust with `. "$HOME/.cargo/env";`.

Before each numbered step N, run `mkdir -p /work/out && echo "step N $(date +%s)" >> /work/out/stages.txt`.

1. Unless `clang` exists: `DEBIAN_FRONTEND=noninteractive apt-get update -qq && DEBIAN_FRONTEND=noninteractive apt-get install -y -qq clang git curl python3 perl ca-certificates`
2. Unless `$HOME/.cargo/bin/rustc` exists: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal`
3. Unless /work/theseus exists: `git clone --depth 1 --branch main https://github.com/EdanBarrios/argon-hackathon /work/theseus`. Run every later command from /work/theseus. Then `git rev-parse HEAD > /work/out/commit.txt`.
4. `backend/harness/expat/fetch_expat.sh`. It downloads libexpat R_2_9_0 from github.com and the W3C suite from www.w3.org, both pinned (commit and sha256). If and only if the W3C download fails, run `mkdir -p backend/harness/expat/vendor && cp /work/mount/xmlts20130923.tar.gz backend/harness/expat/vendor/` and run fetch_expat.sh again (it still checks the pinned sha256), and say so in the final reply as `XMLTS_SOURCE mounted`; otherwise `XMLTS_SOURCE w3.org`.
5. `backend/harness/expat/build_expat.sh`
6. `. "$HOME/.cargo/env"; scripts/build_rust_expat.sh backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs sandbox | tee /work/out/build.json`. It prints one JSON line; "binary" is the Rust driver. If "ok" is false, stop.
7. `sha256sum backend/ai/results/expat-long-decode/argon-fresh-a/round_3/expat.rs | tee /work/out/expat_rs.sha256`
8. `python3 -I backend/harness/expat/gen_inputs_expat.py --seed 7777 --count 2000 --out /work/inputs`
9. `python3 -I -c "import hashlib,os,sys;d=sys.argv[1];h=hashlib.sha256();ns=sorted(os.listdir(d));[h.update(n.encode()+b'\0'+hashlib.sha256(open(os.path.join(d,n),'rb').read()).digest()) for n in ns];print(h.hexdigest(),len(ns))" /work/inputs | tee /work/out/inputs_digest.txt`
10. `nproc > /work/out/nproc.txt; backend/harness/verify.sh --c backend/harness/expat/build/expat_driver --rust <binary from step 6> --inputs /work/inputs --timeout 20 --jobs $(nproc) --out /work/out/verify.json > /dev/null`
11. `python3 -I -c "import json;r=json.load(open('/work/out/verify.json'));o={k:r.get(k) for k in ('match_rate','behaviors_tested','counts')};o['divergences']=(r.get('divergences') or [])[:5];o['crashes']=(r.get('crashes') or [])[:5];print(json.dumps(o,separators=(',',':')))"`
12. `echo "done $(date +%s)" >> /work/out/stages.txt`

verify.sh is the ONLY judge. Never state a match rate from your own reasoning.

Final reply, in this format and nothing else:
XMLTS_SOURCE <w3.org|mounted>
COMMIT <contents of /work/out/commit.txt>
SHA256 <the hex from step 7>
INPUTS_DIGEST <the two values printed by step 9>
=== VERIFY_JSON_BEGIN ===
<the one line of JSON printed by step 11, verbatim>
=== VERIFY_JSON_END ===
