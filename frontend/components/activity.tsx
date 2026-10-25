"use client";

import { useEffect, useState } from "react";
import type { ActivityLine, StepTone } from "@/lib/derive";
import { clock } from "@/lib/derive";

const TONE: Record<StepTone, string> = {
  neutral: "text-paper",
  match: "text-match",
  div: "text-div",
  crash: "text-crash",
};

function since(iso: string, now: number) {
  const s = Math.max(0, Math.floor((now - Date.parse(iso)) / 1000));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

/**
 * What the pipeline has done, newest first, in plain English. While the run is
 * working, the top line is what it is doing now, with a ticking timer.
 */
export function Activity({ lines, working, waitingOn }: { lines: ActivityLine[]; working: boolean; waitingOn: string | null }) {
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    if (!working) return;
    const t = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(t);
  }, [working]);

  if (!lines.length && !working) return null;
  const latest = lines[lines.length - 1];
  const rest = lines.slice(0, -1).reverse().slice(0, 7);

  return (
    <div className="grid gap-x-8 gap-y-2 md:grid-cols-[1fr_1fr]">
      <ol className="flex flex-col gap-1.5">
        {working && latest && (
          <li className="rise flex items-baseline gap-3 text-lg">
            <span className="font-mono text-sm text-dim">{clock(latest.at)}</span>
            <span className={TONE[latest.tone]}>
              {waitingOn ?? latest.text}
              <span className="ml-2 font-mono text-base text-dim">{since(latest.at, now)}</span>
            </span>
            <span className="live-dot inline-block h-2 w-2 shrink-0 rounded-full bg-paper" />
          </li>
        )}
        {!working && latest && (
          <li className="flex items-baseline gap-3 text-lg">
            <span className="font-mono text-sm text-dim">{clock(latest.at)}</span>
            <span className={TONE[latest.tone]}>{latest.text}</span>
          </li>
        )}
      </ol>
      <ol className="flex flex-col gap-1 text-base">
        {rest.map((l) => (
          <li key={l.seq} className="flex items-baseline gap-3 text-dim">
            <span className="font-mono text-xs">{clock(l.at)}</span>
            <span className={l.tone === "neutral" ? "" : `${TONE[l.tone]} opacity-80`}>{l.text}</span>
          </li>
        ))}
      </ol>
    </div>
  );
}
