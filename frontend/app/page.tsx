"use client";

import Link from "next/link";
import { useRef, useState } from "react";
import gsap from "gsap";
import { useGSAP } from "@gsap/react";
import { ScrollTrigger } from "gsap/ScrollTrigger";
import { C_PARSE_STRING, RUST_PARSE_STRING, type CodeLine } from "@/data/excerpts";
import { crashViews, divergenceRows, heroStats, holdoutResult, wallState } from "@/lib/derive";
import { staticFixture } from "@/lib/fixtures-static";
import { Field, type FieldState } from "@/components/story/field";
import { SmoothScroll } from "@/components/story/smooth-scroll";
import { StartRun } from "@/components/story/start-run";
import { TryPanel } from "@/components/try-panel";

gsap.registerPlugin(useGSAP, ScrollTrigger);

// One real run from today, read through the same view-model helpers the run view uses.
const { job, events } = staticFixture("completed");
const stats = heroStats(job, events);
const wall = wallState(job, events);
const round1 = wall?.rounds[0];
const roundLast = wall?.rounds[wall.rounds.length - 1];
const holdout = holdoutResult(events);
const divergence = divergenceRows(job, events)[0];
const repair = divergence?.repair ?? null;
const crash = crashViews(job, events)[0];
const translate = events.find((e) => e.type === "translate");
const rustLines = typeof translate?.data.rust_lines === "number" ? translate.data.rust_lines : null;
const draftModel = typeof translate?.data.source_model === "string" ? translate.data.source_model : null;
const startedAt = job.created_at.slice(11, 16);
const total = round1?.total ?? stats.behaviors ?? 0;
const firstMatch = round1?.match ?? 0;
const divergent = round1?.div ?? 0;
const finalMatch = roundLast?.match ?? 0;
const preserved = stats.final && stats.current !== null && stats.current >= 1;

const RECORDED = `/jobs/${encodeURIComponent(job.id)}?fixture=completed`;
const n = (x: number) => x.toLocaleString("en-US");

function Code({ lines }: { lines: CodeLine[] }) {
  return (
    <pre className="code overflow-hidden">
      {lines.map((l, i) => (
        <div key={i} className={l.hl ? "hl" : undefined}>
          <span className="ln">{l.n ?? ""}</span>
          <span className={l.n === null ? "italic text-dim" : undefined}>{l.text}</span>
        </div>
      ))}
    </pre>
  );
}

const pad = "px-6 md:px-12 lg:px-20";

export default function Home() {
  const root = useRef<HTMLDivElement>(null);
  // One mutable object the scroll timeline drives and the canvas reads each frame.
  const [field] = useState<FieldState>(() => ({ reg: 0, repair: 0 }));
  const counter = useRef<HTMLSpanElement>(null);

  useGSAP(
    () => {
      const setCount = () => {
        if (counter.current) counter.current.textContent = n(Math.round(firstMatch * field.reg + divergent * field.repair));
      };
      const mm = gsap.matchMedia();
      mm.add({ motion: "(prefers-reduced-motion: no-preference)", still: "(prefers-reduced-motion: reduce)" }, (ctx) => {
        if (ctx.conditions?.still) {
          field.reg = 1;
          field.repair = 1;
          setCount();
          gsap.set("#ov-a, #ov-b", { display: "none" });
          gsap.set("#ov-c", { position: "static", opacity: 1 });
          return;
        }
        setCount();

        // Hero: the two plates come into register.
        gsap.fromTo(".reg", { "--rx": "14px", "--ry": "-7px" }, { "--rx": "0px", "--ry": "0px", duration: 2.2, ease: "expo.out", delay: 0.3 });
        gsap.from("#hero [data-rise]", { y: 28, opacity: 0, duration: 1.2, ease: "power3.out", stagger: 0.08, delay: 0.1 });

        // Scroll-linked reveals.
        gsap.utils.toArray<HTMLElement>("[data-reveal]").forEach((el) => {
          gsap.fromTo(
            el,
            { clipPath: "inset(0 0 100% 0)", y: 24 },
            { clipPath: "inset(0 0 -4% 0)", y: 0, ease: "none", scrollTrigger: { trigger: el, start: "top 90%", end: "top 55%", scrub: true } },
          );
        });
        gsap.utils.toArray<HTMLElement>("[data-parallax]").forEach((el) => {
          gsap.fromTo(el, { y: 40 }, { y: -40, ease: "none", scrollTrigger: { trigger: el, start: "top bottom", end: "bottom top", scrub: true } });
        });

        // The rewrite: the Rust writes itself beside the C, then the one line that differs lights up.
        gsap
          .timeline({ scrollTrigger: { trigger: "#translate", start: "top top", end: "+=170%", pin: true, scrub: 0.5 } })
          .fromTo("#rust-col", { clipPath: "inset(0 0 100% 0)" }, { clipPath: "inset(0 0 0% 0)", ease: "none", duration: 1 })
          .fromTo("#translate .hl", { backgroundColor: "rgba(255,90,60,0)", boxShadow: "inset 3px 0 0 rgba(255,90,60,0)" }, { backgroundColor: "rgba(255,90,60,0.18)", boxShadow: "inset 3px 0 0 rgba(255,90,60,1)", duration: 0.25 }, 1.05)
          .to({}, { duration: 0.3 });

        // The check: plates register for the inputs that agree; the divergent ones hold out until the repair.
        gsap
          .timeline({ scrollTrigger: { trigger: "#verify", start: "top top", end: "+=420%", pin: true, scrub: 0.6 } })
          .to(field, { reg: 1, duration: 1.2, ease: "none", onUpdate: setCount })
          .to({}, { duration: 0.25 })
          .to("#ov-a", { opacity: 0, y: -24, duration: 0.3 })
          .fromTo("#ov-b", { opacity: 0, y: 32 }, { opacity: 1, y: 0, duration: 0.3 }, "<0.15")
          .to({}, { duration: 0.7 })
          .to("#ov-b", { opacity: 0, y: -24, duration: 0.3 })
          .fromTo("#ov-c", { opacity: 0, y: 32 }, { opacity: 1, y: 0, duration: 0.3 }, "<0.15")
          .to(field, { repair: 1, duration: 0.8, ease: "power2.inOut", onUpdate: setCount }, "<0.2")
          .fromTo("#ov-c-after", { opacity: 0 }, { opacity: 1, duration: 0.3 }, ">-0.2")
          .to({}, { duration: 0.4 });
      });
    },
    { scope: root },
  );

  return (
    <div ref={root}>
      <SmoothScroll />

      <nav className={`fixed inset-x-0 top-0 z-50 flex items-center justify-between bg-ink/90 py-4 ${pad}`}>
        <Link href="/" className="text-lg font-extrabold tracking-[-0.02em]">
          Theseus
        </Link>
        <div className="flex items-center gap-6 text-base">
          <Link href={RECORDED} className="hidden text-dim transition hover:text-paper sm:inline">
            Recorded run
          </Link>
          <a href="#run" className="whitespace-nowrap rounded-md bg-paper px-4 py-2 font-bold text-ink transition hover:bg-white">
            Start a live run
          </a>
        </div>
      </nav>

      {/* Hero */}
      <section id="hero" className={`flex min-h-screen flex-col justify-end pb-14 pt-32 ${pad}`}>
        <p data-rise className="mb-8 text-base text-dim">
          Built today at the Gemini 4 Argon SF Hackathon. cJSON 1.7.19, C → memory-safe Rust.
        </p>
        <h1 className="display text-[clamp(44px,7vw,112px)]">
          <span data-rise className="block">
            Theseus doesn&rsquo;t ask the model
          </span>
          <span data-rise className="block">
            whether the port works.
          </span>
          <span data-rise className="reg mt-3 inline-block" data-text="It checks.">
            <span>It checks.</span>
          </span>
        </h1>
        <div className="mt-12 grid gap-10 lg:grid-cols-[1fr_auto] lg:items-end">
          <div className="flex flex-col gap-5">
            <p data-rise className="lede text-paper/90">
              Gemini 4 Argon migrates C parsers to memory-safe Rust. A deterministic verifier runs the original and the
              replacement on the same inputs, exposes the differences, and makes the model repair them. A person approves,
              and the evidence ships with the pull request.
            </p>
            <p data-rise className="max-w-[40ch] border-l-2 border-rust pl-4 text-lg leading-snug">
              libexpat, about 18,500 lines of C, to 6,554 lines of safe Rust; matched on every input we tried.
            </p>
          </div>
          <div data-rise className="flex flex-wrap items-center gap-4">
            <a href="#run" className="rounded-md bg-paper px-6 py-3.5 text-lg font-bold text-ink transition hover:bg-white">
              Start a live run
            </a>
            <Link href={RECORDED} className="text-lg underline decoration-line underline-offset-8 transition hover:decoration-paper">
              See the recorded run
            </Link>
          </div>
        </div>
        <p data-rise className="mt-16 border-t border-line pt-5 text-base text-dim">
          Everything below is one run from today, <span className="font-mono text-paper/80">{job.id}</span>, started at {startedAt}. Scroll.
        </p>
      </section>

      {/* The rewrite */}
      <section id="translate" className={`flex h-screen flex-col justify-center overflow-hidden pb-8 pt-24 ${pad}`}>
        <div className="mb-6 grid gap-6 lg:grid-cols-[1fr_1fr] lg:items-end">
          <h2 className="display text-[clamp(34px,4.4vw,68px)]">First, the rewrite.</h2>
          <p className="max-w-[46ch] text-lg leading-snug text-paper/85">
            The model reads the parser and writes one safe-Rust file against a fixed driver: <span className="font-mono text-base">#![forbid(unsafe_code)]</span>,
            no dependencies, no way to call the C. This draft: {rustLines ? `${n(rustLines)} lines` : "one file"}, by{" "}
            {draftModel ?? "the model"}. The verifier does not know who wrote it.
          </p>
        </div>
        <div className="grid gap-8 lg:grid-cols-2">
          <div>
            <div className="mb-3 flex items-baseline justify-between border-b border-line pb-2 text-sm">
              <span className="font-bold text-clang">C &middot; cJSON.c v1.7.19</span>
              <span className="font-mono text-dim">parse_string, line {C_PARSE_STRING[0].n}</span>
            </div>
            <Code lines={C_PARSE_STRING} />
          </div>
          <div id="rust-col">
            <div className="mb-3 flex items-baseline justify-between border-b border-line pb-2 text-sm">
              <span className="font-bold text-rust">Rust &middot; cjson.rs, the draft</span>
              <span className="font-mono text-dim">parse_string, line {RUST_PARSE_STRING[0].n}</span>
            </div>
            <Code lines={RUST_PARSE_STRING} />
          </div>
        </div>
      </section>

      {/* The check */}
      <section id="verify" className="h-screen overflow-hidden">
        <div className={`grid h-full gap-8 pt-24 pb-10 lg:grid-cols-[5fr_7fr] ${pad}`}>
          <div className="relative flex flex-col">
            <div className="text-base text-dim">Then, the check.</div>
            <div className="display mt-2 text-[clamp(64px,8vw,132px)] tabular-nums">
              <span ref={counter}>0</span>
              <span className="text-dim"> / {n(total)}</span>
            </div>
            <div className="relative mt-6 flex-1">
              <div id="ov-a" className="absolute inset-x-0 top-0">
                <h2 className="display text-[clamp(28px,3.4vw,52px)]">Same bytes into the C and into the Rust. Compare every byte out.</h2>
                <p className="mt-5 max-w-[42ch] text-lg leading-snug text-paper/85">
                  {n(total)} inputs, each one a mark printed from two plates. Where the outputs agree, the plates register.{" "}
                  <span className="text-div">{n(divergent)} inputs disagree.</span>
                </p>
              </div>
              <div id="ov-b" className="absolute inset-x-0 top-0 opacity-0">
                <h2 className="display text-[clamp(28px,3.4vw,52px)]">One of the {n(divergent)}.</h2>
                {divergence && (
                  <dl className="mt-6 grid grid-cols-[auto_1fr] gap-x-6 gap-y-3 font-mono text-lg">
                    <dt className="text-dim">input</dt>
                    <dd className="text-paper">{divergence.input}</dd>
                    <dt className="text-clang">C</dt>
                    <dd>{divergence.c_output.replace(/\t/g, "  ")}</dd>
                    <dt className="text-rust">Rust</dt>
                    <dd className="text-div">{divergence.rust_output}</dd>
                  </dl>
                )}
                <p className="mt-6 max-w-[42ch] text-lg leading-snug text-paper/85">
                  C accepts a lone backslash after a bad <span className="font-mono">\u</span> escape and prints an empty string. The draft is
                  stricter. Cleaner, maybe, but behavior-preserving means preserving the weird parts too.
                  {divergence && (
                    <>
                      {" "}
                      {divergence.function_guessed ? "Likely in" : "In"} <span className="font-mono">{divergence.function}</span>.
                    </>
                  )}
                </p>
              </div>
              <div id="ov-c" className="absolute inset-x-0 top-0 opacity-0">
                <h2 className="display text-[clamp(28px,3.4vw,52px)]">Argon gets the differences, not the answers.</h2>
                {repair && repair.patterns.length > 0 && (
                  <ul className="mt-5 flex flex-col gap-1 font-mono text-base text-paper/85">
                    {repair.patterns.map((p) => (
                      <li key={p}>
                        <span className="text-div">&rsaquo;</span> {p}
                      </li>
                    ))}
                  </ul>
                )}
                <p id="ov-c-after" className="mt-6 max-w-[42ch] text-lg leading-snug">
                  {repair?.model ?? "gemini-4-argon"} returned a corrected file
                  {repair?.seconds !== null && repair?.seconds !== undefined ? ` in ${Math.round(repair.seconds)} seconds` : ""}. Now{" "}
                  <span className="text-match">{n(finalMatch)} of {n(total)}</span> agree
                  {holdout ? (
                    <>
                      , and <span className="text-match">{holdout.match} of {holdout.total}</span> held-out inputs it never saw.
                    </>
                  ) : (
                    "."
                  )}
                </p>
              </div>
            </div>
          </div>
          <div className="min-h-[46vh]">
            <Field state={field} total={total} divergent={divergent} />
          </div>
        </div>
      </section>

      {/* Memory safety */}
      <section id="safety" className={`py-32 ${pad}`}>
        <div className="grid gap-12 lg:grid-cols-[5fr_7fr]">
          <div>
            <h2 data-reveal className="display text-[clamp(34px,4.4vw,68px)]">
              And the bug that started all this.
            </h2>
            <p data-reveal className="mt-6 max-w-[42ch] text-lg leading-snug text-paper/85">
              Seven bytes. On the vulnerable release, cJSON 1.7.17, they read past the end of a heap buffer, and AddressSanitizer
              catches it: {crash?.cve ?? "a known CVE"}. The Rust rejects this historical crashing input with a clean parse error.
              cJSON fixed it upstream in 1.7.18; the point is that safe Rust cannot express that read at all.
            </p>
            {crash && (
              <dl className="mt-8 grid grid-cols-[auto_1fr] gap-x-6 gap-y-3 font-mono text-lg">
                <dt className="text-dim">input</dt>
                <dd>{crash.input}</dd>
                <dt className="text-rust">Rust</dt>
                <dd className="text-match">{crash.rust_behavior}</dd>
              </dl>
            )}
          </div>
          <div data-parallax>
            <div className="mb-3 flex items-baseline justify-between border-b border-line pb-2 text-sm">
              <span className="font-bold text-clang">C &middot; {crash?.c_version ?? "cJSON 1.7.17"}, AddressSanitizer</span>
              <span className="font-mono text-crash">{crash?.kind}</span>
            </div>
            <pre className="code max-h-[520px] overflow-hidden whitespace-pre-wrap break-all text-[0.9rem] text-paper/80">
              {(crash?.sanitizer_report ?? "").split("\n").slice(0, 14).join("\n")}
            </pre>
          </div>
        </div>
      </section>

      {/* Approval and the live run */}
      <section id="run" className={`border-t border-line py-32 ${pad}`}>
        <h2 data-reveal className="display text-[clamp(34px,4.4vw,68px)]">
          Nothing ships without a person.
        </h2>
        <p data-reveal className="mt-6 max-w-[52ch] text-lg leading-snug text-paper/85">
          Approve, and a managed agent opens a pull request: the crate, the verifier&rsquo;s evidence report, the command to reproduce it, and
          what it does not cover. Start one now, or hand the verifier any input you like.
        </p>
        <div className="mt-14">
          <StartRun />
        </div>
        <div className="mt-20 border-t border-line pt-10">
          <h3 className="mb-6 text-2xl font-extrabold">Try any input</h3>
          <TryPanel />
        </div>
      </section>

      <footer className={`border-t border-line py-16 ${pad}`}>
        <p className="display text-[clamp(34px,5vw,80px)]">
          {stats.behaviors !== null ? n(stats.behaviors) : "n/a"} tested behaviors {preserved ? "preserved." : "tested."}
        </p>
        <p className="mt-8 max-w-[70ch] text-base text-dim">
          Built today: the verifier, the repair pipeline, the sandbox runs, this site. cJSON is third-party code (MIT), used as the
          input. Type: Schibsted Grotesk and Fragment Mono (OFL), self-hosted. Motion: GSAP and Lenis.
        </p>
      </footer>
    </div>
  );
}
