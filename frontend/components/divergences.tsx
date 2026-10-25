"use client";

import { useState } from "react";
import { diffParts, type DivergenceRow } from "@/lib/derive";

function Highlighted({ parts, tone }: { parts: string[]; tone: "c" | "rust" }) {
  const [pre, mid, post] = parts;
  return (
    <pre className="min-h-16 whitespace-pre-wrap break-all rounded-lg bg-black p-3 font-mono text-xl leading-snug text-neutral-100">
      {pre}
      {mid && (
        <mark className={`rounded px-0.5 text-white ${tone === "c" ? "bg-neutral-600" : "bg-red-600"}`}>{mid}</mark>
      )}
      {post}
    </pre>
  );
}

function Detail({ row }: { row: DivergenceRow }) {
  const [c, r] = diffParts(row.c_output, row.rust_output);
  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <div className="text-lg">
          <span className="text-neutral-400">split in </span>
          <span className="font-mono font-semibold text-white">{row.function}()</span>
        </div>
        <span className={`text-lg font-semibold ${row.resolved ? "text-green-400" : "text-red-400"}`}>
          {row.resolved ? "✓ repaired" : "✗ open divergence"}
        </span>
      </div>
      <div>
        <div className="mb-1 text-xs font-bold uppercase tracking-widest text-neutral-400">Input</div>
        <pre className="whitespace-pre-wrap break-all rounded-lg bg-black p-3 font-mono text-xl text-neutral-100">
          {row.input}
        </pre>
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <div className="mb-1 text-xs font-bold uppercase tracking-widest text-neutral-400">C output</div>
          <Highlighted parts={c} tone="c" />
        </div>
        <div>
          <div className="mb-1 text-xs font-bold uppercase tracking-widest text-neutral-400">Rust output</div>
          <Highlighted parts={r} tone="rust" />
        </div>
      </div>
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
      <table className="w-full self-start table-fixed border-separate border-spacing-y-1 text-left text-lg">
        <thead className="text-xs uppercase tracking-widest text-neutral-500">
          <tr>
            <th className="w-[45%] px-3 py-1 font-semibold">Input</th>
            <th className="px-3 py-1 font-semibold">Function</th>
            <th className="w-16 px-3 py-1 text-center font-semibold">Status</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((r) => {
            const isSel = r.key === current.key;
            const open = !r.resolved;
            return (
              <tr
                key={r.key}
                onClick={() => setSelected(r.key)}
                onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && setSelected(r.key)}
                tabIndex={0}
                aria-selected={isSel}
                className={`cursor-pointer outline-none transition-colors ${
                  open
                    ? "bg-red-600/25 text-red-100 hover:bg-red-600/40"
                    : "bg-neutral-800/50 hover:bg-neutral-800"
                } ${isSel ? (open ? "ring-2 ring-red-400" : "ring-2 ring-neutral-400") : ""}`}
              >
                <td className={`truncate rounded-l-lg px-3 py-2 font-mono ${open ? "border-l-4 border-red-500" : ""}`}>
                  {r.input}
                </td>
                <td className="truncate px-3 py-2 font-mono text-base">{r.function}</td>
                <td className="rounded-r-lg px-3 py-2 text-center text-2xl font-bold">
                  {open ? (
                    <span className="animate-pulse text-red-400" title="open divergence">
                      ✗
                    </span>
                  ) : (
                    <span className="text-green-400" title="resolved">
                      ✓
                    </span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <div className={`rounded-lg border p-4 ${current.resolved ? "border-neutral-700" : "border-red-500/70"}`}>
        <Detail row={current} />
      </div>
    </div>
  );
}
