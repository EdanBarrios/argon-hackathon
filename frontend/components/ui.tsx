import type { ReactNode } from "react";
import type { Status } from "@/lib/api";

// Color is semantic only: green = match/done, red = divergence/failure, amber = crash.
// Everything else stays neutral.
const PILL: Record<Status, string> = {
  queued: "border-neutral-600 text-neutral-300",
  running: "border-neutral-300 text-white",
  verifying: "border-neutral-300 text-white",
  completed: "border-green-500 bg-green-500/15 text-green-300",
  failed: "border-red-500 bg-red-500/15 text-red-300",
  cancelled: "border-neutral-600 text-neutral-400",
};

export function StatusPill({ status, size = "md" }: { status: Status; size?: "md" | "lg" }) {
  const active = status === "running" || status === "verifying";
  return (
    <span
      className={`inline-flex items-center gap-2 rounded-full border font-semibold uppercase tracking-wide ${PILL[status]} ${
        size === "lg" ? "px-4 py-1.5 text-lg" : "px-3 py-0.5 text-sm"
      }`}
    >
      {active && <span className="h-2 w-2 animate-pulse rounded-full bg-white" />}
      {status}
    </span>
  );
}

export function SampleBadge({ notice }: { notice: string | null }) {
  return (
    <span
      title={notice ?? undefined}
      className="rounded border border-dashed border-neutral-400 px-2 py-0.5 font-mono text-xs uppercase tracking-wider text-neutral-300"
    >
      sample data{notice ? ` (${notice})` : ""}
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
    <section className={`rounded-xl border border-neutral-800 bg-neutral-900 p-5 ${className}`}>
      <div className="mb-4 flex items-center justify-between gap-4">
        <h2 className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">{title}</h2>
        {right}
      </div>
      {children}
    </section>
  );
}
