"use client";

import { useState } from "react";
import { API_URL, tryInput, type TryResult, type TrySide, type TryVerdict } from "@/lib/api";
import { diffParts } from "@/lib/derive";

// Every preset runs through the real binaries via POST /try. Nothing here is precomputed.
const PRESETS: { label: string; input: string; note?: string }[] = [
  { label: "[1e999]", input: "[1e999]" },
  { label: "[9007199254740993]", input: "[9007199254740993]" },
  { label: '"😀"', input: '"😀"' },
  { label: '{"a": 1,}', input: '{"a": 1,}' },
  { label: '{"1":1,', input: '{"1":1,', note: "CVE-2023-53154, auto-compares the old version" },
];

const VERDICT: Record<TryVerdict, { text: string; cls: string }> = {
  match: { text: "Identical behavior", cls: "border-green-500 bg-green-500/15 text-green-300" },
  divergence: { text: "Divergence", cls: "border-red-500 bg-red-500/15 text-red-300" },
  safety_improvement: { text: "Safety improvement", cls: "border-amber-500 bg-amber-500/15 text-amber-300" },
  rust_crash: { text: "Rust crashed", cls: "border-red-500 bg-red-500/15 text-red-300" },
  both_crash: { text: "Both crashed", cls: "border-red-500 bg-red-500/15 text-red-300" },
  timeout: { text: "Timeout", cls: "border-neutral-500 text-neutral-300" },
};

function Out({ parts, tone, empty }: { parts: string[]; tone: "same" | "diff"; empty: boolean }) {
  const [pre, mid, post] = parts;
  return (
    <pre className="min-h-14 whitespace-pre-wrap break-all rounded-lg bg-black p-3 font-mono text-lg leading-snug text-neutral-100">
      {empty ? <span className="text-neutral-600">(no stdout)</span> : null}
      {pre}
      {mid && <mark className={`rounded px-0.5 text-white ${tone === "diff" ? "bg-red-600" : "bg-neutral-600"}`}>{mid}</mark>}
      {post}
    </pre>
  );
}

function Side({ side, parts, tone }: { side: TrySide; parts: string[]; tone: "same" | "diff" }) {
  return (
    <div className="flex min-w-0 flex-col gap-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className="text-sm font-bold uppercase tracking-widest text-neutral-300">{side.label}</span>
        <span className="truncate font-mono text-xs text-neutral-500" title={side.binary}>
          {side.version} · {side.seconds.toFixed(2)}s
        </span>
      </div>
      <Out parts={parts} tone={tone} empty={!side.stdout} />
      {side.stdout_hex && (
        <pre className="whitespace-pre-wrap break-all rounded bg-black/60 p-2 font-mono text-xs text-neutral-400">hex: {side.stdout_hex}</pre>
      )}
      <div className={`font-mono text-sm ${side.crashed ? "text-amber-400" : "text-neutral-400"}`}>
        {side.exit}
        {side.crashed ? " (crashed)" : ""}
      </div>
      {side.sanitizer_summary && (
        <div className="rounded-lg border border-amber-500/60 bg-amber-500/10 p-2 font-mono text-sm text-amber-300">
          {side.sanitizer_summary}
        </div>
      )}
      {side.crashed && side.stderr_head && (
        <details>
          <summary className="cursor-pointer text-xs font-semibold uppercase tracking-widest text-neutral-400 hover:text-white">
            Sanitizer report
          </summary>
          <pre className="mt-1 max-h-56 overflow-auto rounded-lg bg-black p-2 font-mono text-xs leading-relaxed text-neutral-200">
            {side.stderr_head}
          </pre>
        </details>
      )}
    </div>
  );
}

function Verdict({ r }: { r: TryResult }) {
  const old = r.old_c && r.old_verdict === "safety_improvement";
  const main = old ? VERDICT.safety_improvement : VERDICT[r.verdict];
  return (
    <div className="flex flex-wrap items-center gap-3">
      <span className={`rounded-full border px-5 py-2 text-2xl font-black uppercase tracking-wide ${main.cls}`}>{main.text}</span>
      {old && (
        <span className="text-lg text-neutral-200">
          {r.cve ? `${r.cve}: ` : ""}old C ({r.old_c?.version}) hits AddressSanitizer; safe Rust returns a clean error.
        </span>
      )}
      {r.old_c && (
        <span className={`rounded-full border px-3 py-0.5 text-sm font-semibold uppercase ${VERDICT[r.verdict].cls}`}>
          vs {r.c.version}: {VERDICT[r.verdict].text}
        </span>
      )}
    </div>
  );
}

export function TryPanel({ compact = false }: { compact?: boolean }) {
  const [input, setInput] = useState(PRESETS[0].input);
  const [compareOld, setCompareOld] = useState(false);
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<TryResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const run = async (text = input) => {
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      setResult(await tryInput({ library: "cjson", input: text, compare_old: compareOld }));
    } catch (e) {
      setError((e as Error).message);
    } finally {
      setBusy(false);
    }
  };

  const bytes = new TextEncoder().encode(input).length;
  const [cParts, rParts] = result ? diffParts(result.c.stdout, result.rust.stdout) : [["", "", ""], ["", "", ""]];
  const differs = result ? result.c.stdout !== result.rust.stdout : false;

  return (
    <div className="flex flex-col gap-4">
      {!compact && (
        <p className="text-neutral-400">
          Type any JSON (or garbage). It runs right now through the original C (cJSON v1.7.19, ASan) and the Rust
          translation, byte for byte, via the same verifier as the full runs.
        </p>
      )}
      <div className="flex flex-wrap gap-2">
        {PRESETS.map((p) => (
          <button
            key={p.label}
            type="button"
            title={p.note}
            disabled={busy}
            onClick={() => {
              setInput(p.input);
              void run(p.input);
            }}
            className={`rounded-full border px-3 py-1 font-mono text-base hover:bg-neutral-800 disabled:opacity-50 ${
              p.note ? "border-amber-500/70 text-amber-200" : "border-neutral-600 text-neutral-200"
            }`}
          >
            {p.label}
            {p.note && <span className="ml-2 font-sans text-xs uppercase tracking-wider text-amber-400">CVE</span>}
          </button>
        ))}
      </div>
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
        spellCheck={false}
        rows={compact ? 2 : 3}
        aria-label="Input bytes"
        className="w-full rounded-lg border border-neutral-700 bg-black p-3 font-mono text-xl text-white outline-none focus:border-neutral-300"
      />
      <div className="flex flex-wrap items-center gap-4">
        <button
          type="button"
          onClick={() => void run()}
          disabled={busy || bytes > 65536}
          className="rounded-lg bg-white px-6 py-2 text-xl font-bold text-black hover:bg-neutral-200 disabled:bg-neutral-700 disabled:text-neutral-400"
        >
          {busy ? "Running..." : "Run"}
        </button>
        <label className="flex items-center gap-2 text-neutral-300">
          <input type="checkbox" checked={compareOld} onChange={(e) => setCompareOld(e.target.checked)} />
          also run old cJSON v1.7.17
        </label>
        <span className="font-mono text-sm text-neutral-500">
          {bytes} bytes{bytes > 65536 ? " (over the 64 KB limit)" : ""} · POST {API_URL}/try
        </span>
      </div>
      {busy && (
        <p className="animate-pulse text-neutral-400">
          Running both binaries... (an ASan crash report takes about 8 s to symbolize on macOS)
        </p>
      )}
      {error && <p className="rounded-lg border border-red-500 bg-red-500/10 p-3 text-lg text-red-300">{error}</p>}
      {result && (
        <div className="flex flex-col gap-4">
          <Verdict r={result} />
          {result.old_c_missing && <p className="text-sm text-neutral-500">{result.old_c_missing}</p>}
          <div className={`grid gap-4 ${result.old_c ? "md:grid-cols-3" : "md:grid-cols-2"}`}>
            {result.old_c && <Side side={result.old_c} parts={[result.old_c.stdout, "", ""]} tone="same" />}
            <Side side={result.c} parts={cParts} tone="same" />
            <Side side={result.rust} parts={rParts} tone={differs ? "diff" : "same"} />
          </div>
        </div>
      )}
    </div>
  );
}
