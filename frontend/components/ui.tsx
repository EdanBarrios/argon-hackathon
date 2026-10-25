"use client";

import { useEffect, useRef, useState, type ReactNode } from "react";
import type { Status } from "@/lib/api";

// Color is semantic only: green = match/done, red = divergence/failure, amber = crash.
// C is blue, Rust is orange. Everything else stays neutral.
const PILL: Record<Status, string> = {
  queued: "border-line text-neutral-300",
  running: "border-match/60 text-match",
  verifying: "border-match/60 text-match",
  completed: "border-match/60 bg-match/10 text-match",
  failed: "border-div/70 bg-div/10 text-div",
  cancelled: "border-line text-neutral-400",
};

export function StatusPill({ status, size = "md" }: { status: Status; size?: "md" | "lg" }) {
  const active = status === "running" || status === "verifying";
  return (
    <span
      className={`inline-flex items-center gap-2 rounded border font-mono font-semibold uppercase tracking-widest ${PILL[status]} ${
        size === "lg" ? "px-3 py-1 text-base" : "px-2 py-0.5 text-xs"
      }`}
    >
      {active && <span className="live-dot h-2 w-2 rounded-full bg-match" />}
      {active ? "live" : status}
      {active && <span className="text-neutral-400">· {status}</span>}
    </span>
  );
}

export function SampleBadge({ notice }: { notice: string | null }) {
  return (
    <span
      title={notice ?? undefined}
      className="rounded border border-dashed border-neutral-500 px-2 py-0.5 font-mono text-xs uppercase tracking-wider text-neutral-300"
    >
      recorded run{notice ? ` (${notice})` : ""}
    </span>
  );
}

export function Card({
  title,
  right,
  children,
  className = "",
}: {
  title: string;
  right?: ReactNode;
  children: ReactNode;
  className?: string;
}) {
  return (
    <section className={`panel rise p-5 ${className}`}>
      <div className="mb-4 flex items-center justify-between gap-4">
        <h2 className="font-mono text-sm font-bold uppercase tracking-[0.25em] text-dim">
          <span className="text-match">▸</span> {title}
        </h2>
        {right}
      </div>
      {children}
    </section>
  );
}

/** Animates from the previous value to `value`. Starts at 0, so numbers count up on load. */
export function useCountUp(value: number, ms = 900) {
  const [shown, setShown] = useState(0);
  const from = useRef(0);
  useEffect(() => {
    const start = performance.now();
    const a = from.current;
    let raf = 0;
    const tick = (t: number) => {
      const k = Math.min(1, (t - start) / ms);
      const eased = 1 - Math.pow(1 - k, 3);
      const v = a + (value - a) * eased;
      setShown(v);
      from.current = v;
      if (k < 1) raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [value, ms]);
  return shown;
}

export function CountUp({ value, format = (x) => Math.round(x).toLocaleString("en-US") }: { value: number; format?: (x: number) => string }) {
  return <>{format(useCountUp(value))}</>;
}
