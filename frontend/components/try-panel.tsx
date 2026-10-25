"use client";

import { useEffect, useState } from "react";
import { API_URL, tryInput, tryPresets, type TryPreset, type TryResult, type TrySide, type TryVerdict } from "@/lib/api";
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
  match: { text: "Identical behavior", cls: "border-match bg-match/15 text-match" },
  divergence: { text: "Divergence", cls: "border-div bg-div/15 text-div" },
  safety_improvement: { text: "Safety improvement", cls: "border-crash bg-crash/15 text-crash" },
  rust_crash: { text: "Rust crashed", cls: "border-div bg-div/15 text-div" },
  both_crash: { text: "Both crashed", cls: "border-div bg-div/15 text-div" },
  timeout: { text: "Timeout", cls: "border-dim text-paper/80" },
};

function Out({ parts, tone, empty }: { parts: string[]; tone: "same" | "diff"; empty: boolean }) {
  const [pre, mid, post] = parts;
  return (
    <pre className="max-h-[28rem] min-h-14 overflow-auto whitespace-pre-wrap break-all rounded-lg bg-ink p-3 font-mono text-lg leading-snug text-paper">
      {empty ? <span className="text-dim">(no stdout)</span> : null}
      {pre}
      {mid && <mark className={`rounded px-0.5 text-paper ${tone === "diff" ? "bg-div" : "bg-line"}`}>{mid}</mark>}
      {post}
    </pre>
  );
}

function Side({ side, parts, tone }: { side: TrySide; parts: string[]; tone: "same" | "diff" }) {
  return (
    <div className="flex min-w-0 flex-col gap-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className="text-base font-bold">{side.label}</span>
        <span className="truncate font-mono text-xs text-dim" title={side.binary}>
          {side.version} · {side.seconds.toFixed(2)}s
        </span>
      </div>
      <Out parts={parts} tone={tone} empty={!side.stdout} />
      {side.stdout_hex && (
        <pre className="whitespace-pre-wrap break-all rounded bg-ink/60 p-2 font-mono text-xs text-dim">hex: {side.stdout_hex}</pre>
      )}
      <div className={`font-mono text-sm ${side.crashed ? "text-crash" : "text-dim"}`}>
        {side.exit}
        {side.crashed ? " (crashed)" : ""}
      </div>
      {side.sanitizer_summary && (
        <div className="rounded-lg border border-crash/60 bg-crash/10 p-2 font-mono text-sm text-crash">
          {side.sanitizer_summary}
        </div>
      )}
      {side.crashed && side.stderr_head && (
        <details>
          <summary className="cursor-pointer text-sm text-dim hover:text-paper">
            Sanitizer report
          </summary>
          <pre className="mt-1 max-h-56 overflow-auto rounded-lg bg-ink p-2 font-mono text-xs leading-relaxed text-paper">
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
      <span className={`rounded-full border px-5 py-2 text-2xl font-extrabold ${main.cls}`}>{main.text}</span>
      {old && (
        <span className="text-lg text-paper">
          {r.cve ? `${r.cve}: ` : ""}old C ({r.old_c?.version}) hits AddressSanitizer; safe Rust returns a clean error.
        </span>
      )}
      {r.first_difference && !r.old_c && (
        <span className="text-lg text-paper">Outputs first differ on line {r.first_difference.line}.</span>
      )}
      {r.old_c && (
        <span className={`rounded-full border px-3 py-0.5 text-sm font-semibold ${VERDICT[r.verdict].cls}`}>
          vs {r.c.version}: {VERDICT[r.verdict].text}
        </span>
      )}
    </div>
  );
}

type Lang = "xml" | "json";

const LANG: Record<Lang, { label: string; library: "expat" | "cjson"; about: string }> = {
  xml: { label: "XML", library: "expat", about: "libexpat 2.9.0 (C, ASan+UBSan) against Argon's safe-Rust port; one parser event per line" },
  json: { label: "JSON", library: "cjson", about: "cJSON v1.7.19 (C, ASan) against the Rust translation" },
};

export function TryPanel({ compact = false }: { compact?: boolean }) {
  const [lang, setLang] = useState<Lang>("xml");
  const [xmlPresets, setXmlPresets] = useState<TryPreset[] | null>(null);
  const [presetError, setPresetError] = useState<string | null>(null);
  const [input, setInput] = useState("");
  const [compareOld, setCompareOld] = useState(false);
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<TryResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  // XML presets come from the backend (GET /try/presets?library=expat); "rich" is the default input.
  useEffect(() => {
    let live = true;
    tryPresets("expat")
      .then((ps) => {
        if (!live) return;
        setXmlPresets(ps);
        const rich = ps.find((p) => p.id === "rich") ?? ps[0];
        if (rich) setInput((cur) => (cur === "" ? rich.input : cur));
      })
      .catch((e) => live && setPresetError((e as Error).message));
    return () => {
      live = false;
    };
  }, []);

  const switchTo = (next: Lang) => {
    if (next === lang) return;
    setLang(next);
    setResult(null);
    setError(null);
    if (next === "json") setInput(PRESETS[0].input);
    else setInput((xmlPresets?.find((p) => p.id === "rich") ?? xmlPresets?.[0])?.input ?? "");
  };

  const run = async (text = input) => {
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      setResult(
        await tryInput(
          lang === "json" ? { library: "cjson", input: text, compare_old: compareOld } : { library: "expat", input: text },
        ),
      );
    } catch (e) {
      setError((e as Error).message);
    } finally {
      setBusy(false);
    }
  };

  const bytes = new TextEncoder().encode(input).length;
  const [cParts, rParts] = result ? diffParts(result.c.stdout, result.rust.stdout) : [["", "", ""], ["", "", ""]];
  const differs = result ? result.c.stdout !== result.rust.stdout : false;
  const chips: { key: string; label: string; input: string; note?: string; cve?: boolean }[] =
    lang === "json"
      ? PRESETS.map((p) => ({ key: p.label, label: p.label, input: p.input, note: p.note, cve: !!p.note }))
      : (xmlPresets ?? []).map((p) => ({ key: p.id, label: p.title, input: p.input, note: p.description, cve: /CVE/.test(p.title) }));

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-wrap items-center gap-4">
        <div role="group" aria-label="Input language" className="inline-flex rounded-md border border-line p-0.5">
          {(Object.keys(LANG) as Lang[]).map((l) => (
            <button
              key={l}
              type="button"
              aria-pressed={lang === l}
              onClick={() => switchTo(l)}
              disabled={busy}
              className={`rounded px-4 py-1.5 text-base font-bold transition ${lang === l ? "bg-paper text-ink" : "text-dim hover:text-paper"}`}
            >
              {LANG[l].label}
            </button>
          ))}
        </div>
        <p className="text-dim">{LANG[lang].about}.</p>
      </div>
      {!compact && (
        <p className="text-dim">
          Type any {LANG[lang].label} (or garbage). It runs right now through the original C and the Rust translation, byte for byte,
          via the same verifier as the full runs.
        </p>
      )}
      <div className="flex flex-wrap gap-2">
        {lang === "xml" && !xmlPresets && !presetError && <span className="text-dim">Loading XML presets…</span>}
        {lang === "xml" && presetError && <span className="text-div">Could not load XML presets: {presetError}</span>}
        {chips.map((p) => (
          <button
            key={p.key}
            type="button"
            title={p.note}
            disabled={busy}
            onClick={() => {
              setInput(p.input);
              void run(p.input);
            }}
            className={`rounded-full border px-3 py-1 text-base hover:bg-line disabled:opacity-50 ${lang === "json" ? "font-mono" : ""} ${
              p.cve ? "border-crash/70 text-crash" : "border-line text-paper"
            }`}
          >
            {p.label}
            {lang === "json" && p.cve && <span className="ml-2 text-xs font-bold text-crash">CVE</span>}
          </button>
        ))}
      </div>
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
        spellCheck={false}
        rows={lang === "xml" ? (compact ? 5 : 7) : compact ? 2 : 3}
        aria-label="Input bytes"
        className="w-full rounded-lg border border-line bg-ink p-3 font-mono text-lg text-paper outline-none focus:border-dim"
      />
      <div className="flex flex-wrap items-center gap-4">
        <button
          type="button"
          onClick={() => void run()}
          disabled={busy || bytes > 65536 || input === ""}
          className="rounded-lg bg-paper px-6 py-2 text-xl font-bold text-ink hover:bg-white disabled:bg-line disabled:text-dim"
        >
          {busy ? "Running..." : "Run"}
        </button>
        {lang === "json" && (
          <label className="flex items-center gap-2 text-paper/80">
            <input type="checkbox" checked={compareOld} onChange={(e) => setCompareOld(e.target.checked)} />
            also run old cJSON v1.7.17
          </label>
        )}
        <span className="font-mono text-sm text-dim">
          {bytes} bytes{bytes > 65536 ? " (over the 64 KB limit)" : ""} · POST {API_URL}/try · library {LANG[lang].library}
        </span>
      </div>
      {busy && <p className="animate-pulse text-dim">Running both binaries…</p>}
      {error && <p className="rounded-lg border border-div bg-div/10 p-3 text-lg text-div">{error}</p>}
      {result && (
        <div className="flex flex-col gap-4">
          <Verdict r={result} />
          {result.old_c_missing && <p className="text-sm text-dim">{result.old_c_missing}</p>}
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
