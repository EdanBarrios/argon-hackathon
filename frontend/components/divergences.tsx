"use client";

import { useState } from "react";
import { diffParts, type DivergenceRow } from "@/lib/derive";

function Highlighted({ parts, tone }: { parts: string[]; tone: "c" | "rust" }) {
  const [pre, mid, post] = parts;
  return (
    <pre className="min-h-14 whitespace-pre-wrap break-all rounded-md bg-black p-3 font-mono text-xl leading-snug text-neutral-100">
      {pre}
      {mid && (
        <mark className={`rounded px-0.5 text-white ${tone === "c" ? "bg-clang/40" : "bg-div shadow-[0_0_12px_rgb(255_77_94/0.6)]"}`}>
          {mid}
        </mark>
      )}
      {post}
    </pre>
  );
}

function Detail({ row }: { row: DivergenceRow }) {
  const [c, r] = diffParts(row.c_output, row.rust_output);
  return (
    <div key={row.key} className="rise flex flex-col gap-4">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <div className="text-lg">
          <span className="text-neutral-400">{row.function_guessed ? "likely in " : "split in "}</span>
          <span className="font-mono font-semibold text-white">{row.function}()</span>
        </div>
        <span className={`font-mono text-base font-semibold ${row.resolved ? "text-match" : "text-div"}`}>
          {row.resolved ? "[✓] repaired" : "[✗] open divergence"}
        </span>
      </div>
      <div>
        <div className="mb-1 font-mono text-xs uppercase tracking-[0.2em] text-dim">Input</div>
        <pre className="whitespace-pre-wrap break-all rounded-md bg-black p-3 font-mono text-xl text-white">{row.input}</pre>
      </div>
      <div className="grid grid-cols-2 gap-3">
        <div>
          <div className="mb-1 font-mono text-xs uppercase tracking-[0.2em] text-clang">C · reference</div>
          <Highlighted parts={c} tone="c" />
        </div>
        <div>
          <div className="mb-1 font-mono text-xs uppercase tracking-[0.2em] text-rust">Rust · translation</div>
          <Highlighted parts={r} tone="rust" />
        </div>
      </div>
      {row.repair && (
        <div className="rounded-md border border-match/50 bg-match/[0.06] p-4">
          <div className="mb-2 flex flex-wrap items-baseline justify-between gap-2 font-mono text-xs uppercase tracking-[0.2em] text-match">
            <span>
              {row.repair.model ?? "Argon"} repair{row.repair.iteration !== null ? ` · round ${row.repair.iteration}` : ""}
            </span>
            {row.repair.seconds !== null && <span>{Math.round(row.repair.seconds)} s</span>}
          </div>
          {row.repair.patterns.length > 0 && (
            <div className="mb-2">
              <div className="mb-1 text-sm text-neutral-400">Verifier feedback sent to the model:</div>
              <ul className="flex flex-col gap-0.5 font-mono text-sm text-neutral-200">
                {row.repair.patterns.map((p) => (
                  <li key={p}>
                    <span className="text-div">›</span> {p}
                  </li>
                ))}
              </ul>
            </div>
          )}
          <p className="text-lg leading-snug text-neutral-100">{row.repair.summary}</p>
        </div>
      )}
    </div>
  );
}

export function Divergences({ rows }: { rows: DivergenceRow[] }) {
  const [selected, setSelected] = useState<string | null>(null);
  if (!rows.length) return <p className="text-lg text-neutral-500">No divergences yet. Every tested behavior matches so far.</p>;
  // Default to the first open divergence so the red row is what the detail shows.
  const current = rows.find((r) => r.key === selected) ?? rows.find((r) => !r.resolved) ?? rows[0];

  return (
    <div className="grid grid-cols-[minmax(0,5fr)_minmax(0,7fr)] gap-5">
      <table className="w-full self-start table-fixed border-separate border-spacing-y-1.5 text-left text-lg">
        <thead className="font-mono text-xs uppercase tracking-[0.2em] text-dim">
          <tr>
            <th className="w-[50%] px-3 py-1 font-normal">Input</th>
            <th className="px-3 py-1 font-normal">Function</th>
            <th className="w-14 px-3 py-1 text-center font-normal">
              <span className="sr-only">Status</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {rows.map((r, i) => {
            const isSel = r.key === current.key;
            const open = !r.resolved;
            return (
              <tr
                key={r.key}
                onClick={() => setSelected(r.key)}
                onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && setSelected(r.key)}
                tabIndex={0}
                aria-selected={isSel}
                style={{ animationDelay: `${i * 90}ms` }}
                className={`rise cursor-pointer outline-none transition-colors duration-500 ${
                  open ? "bg-div/20 text-red-100 hover:bg-div/30" : "bg-white/[0.03] hover:bg-white/[0.07]"
                } ${isSel ? (open ? "ring-1 ring-div" : "ring-1 ring-match/60") : ""}`}
              >
                <td className={`truncate rounded-l-md border-l-2 px-3 py-2 font-mono ${open ? "border-div" : "border-match/60"}`}>
                  {r.input}
                </td>
                <td className="truncate px-3 py-2 font-mono text-base text-neutral-300">{r.function}</td>
                <td className="rounded-r-md px-3 py-2 text-center font-mono text-xl font-bold">
                  {open ? (
                    <span className="text-div glow-div" title="open divergence">
                      ✗
                    </span>
                  ) : (
                    <span className="text-match" title="resolved">
                      ✓
                    </span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <div className={`rounded-md border p-4 transition-colors duration-500 ${current.resolved ? "border-line" : "border-div/70"}`}>
        <Detail row={current} />
      </div>
    </div>
  );
}
