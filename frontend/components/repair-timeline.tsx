"use client";

import { useEffect, useState } from "react";
import type { RepairRound } from "@/lib/derive";

function elapsed(iso: string, now: number) {
  const s = Math.max(0, Math.floor((now - Date.parse(iso)) / 1000));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

const n = (x: number) => x.toLocaleString("en-US");

/**
 * Each repair round as it happens: what the model was sent (the verifier's
 * patterns, or the compiler's errors), how long it took, and what came back.
 * The model never sees pass/fail from itself; this is the loop made visible.
 */
export function RepairTimeline({ rounds }: { rounds: RepairRound[] }) {
  const working = rounds.some((r) => !r.done);
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    if (!working) return;
    const t = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(t);
  }, [working]);
  if (!rounds.length) return null;

  return (
    <ol className="flex flex-col">
      {rounds.map((r) => (
        <li key={r.iteration} className="rise grid gap-4 border-t border-line py-5 md:grid-cols-[11rem_1fr_1fr]">
          <div>
            <div className="text-lg font-bold">Repair round {r.iteration}</div>
            <div className="font-mono text-sm text-dim">{r.model}</div>
            <div className={`mt-1 font-mono text-base ${r.done ? "text-dim" : "text-paper"}`}>
              {r.done ? (r.seconds !== null ? `${Math.round(r.seconds)} s` : "done") : `thinking · ${elapsed(r.startedAt, now)}`}
              {!r.done && <span className="live-dot ml-2 inline-block h-2 w-2 rounded-full bg-paper" />}
            </div>
          </div>

          <div>
            <div className="mb-1 text-sm text-dim">
              {r.kind === "compile" ? "Sent: the compiler's errors" : `Sent: ${r.patterns.length} divergence patterns, not answers`}
            </div>
            {r.kind === "divergences" ? (
              <ul className="flex flex-col gap-0.5 font-mono text-sm">
                {r.patterns.map((p) => (
                  <li key={p}>
                    <span className="text-div">›</span> {p}
                  </li>
                ))}
              </ul>
            ) : (
              <p className="font-mono text-sm text-dim">The previous round&rsquo;s file, with rustc&rsquo;s output.</p>
            )}
          </div>

          <div>
            <div className="mb-1 text-sm text-dim">Came back</div>
            {!r.done ? (
              <p className="text-paper/80">Waiting for a corrected file…</p>
            ) : r.buildError ? (
              <>
                <p className="font-bold text-div">
                  Did not compile{r.buildErrorCount ? ` (${r.buildErrorCount} errors)` : ""}. Errors go back to the model.
                </p>
                <pre className="mt-1 whitespace-pre-wrap break-words font-mono text-xs leading-snug text-dim">{r.buildError}</pre>
              </>
            ) : r.verified ? (
              <p className={`font-bold ${r.verified.match === r.verified.total ? "text-match" : "text-div"}`}>
                Built. Verifier: {n(r.verified.match)}/{n(r.verified.total)} byte-identical.
              </p>
            ) : (
              <p className="text-paper/80">A corrected file. Building and verifying…</p>
            )}
          </div>
        </li>
      ))}
    </ol>
  );
}
