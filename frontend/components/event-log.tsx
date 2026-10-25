import type { JobEvent } from "@/lib/api";
import { clock, pct } from "@/lib/derive";

function summary(e: JobEvent): string {
  const d = e.data;
  if (typeof d.summary === "string" && e.type !== "repair") return `${e.type}: ${d.summary}`;
  switch (e.type) {
    case "job_status":
      return `job ${d.status ?? ""}${d.plan ? ` plan: ${d.plan.join(", ")}` : ""}`;
    case "translate":
      return `translate ${d.module ?? ""} iter ${d.iteration ?? "?"}${d.functions ? ` (${d.functions.join(", ")})` : ""}`;
    case "verify":
      return `verify iter ${d.iteration ?? "?"}: ${pct(d.match_rate)} match, ${d.behaviors_tested ?? "?"} behaviors, ${d.divergences ?? 0} divergences, ${d.crashes ?? 0} crashes`;
    case "divergence":
      return `divergence in ${d.function ?? "?"}: ${d.input ?? ""}`;
    case "crash":
      return `crash (${d.side ?? "?"}): ${d.sanitizer ?? ""}; rust: ${d.rust_behavior ?? "?"}`;
    case "repair":
      return `repair ${d.function ?? ""} iter ${d.iteration ?? "?"}: ${d.summary ?? ""}`;
    case "approval_required":
      return `approval required: ${d.action ?? ""}`;
    case "approved":
      return `approved${d.approved_by ? ` by ${d.approved_by}` : ""}${d.pr_url ? `: ${d.pr_url}` : ""}`;
    default:
      return JSON.stringify(d);
  }
}

const TONE: Partial<Record<JobEvent["type"], string>> = {
  divergence: "text-red-400",
  crash: "text-amber-400",
  approved: "text-green-400",
};

export function EventLog({ events }: { events: JobEvent[] }) {
  return (
    <details className="rounded-xl border border-neutral-800 bg-neutral-900 p-5">
      <summary className="cursor-pointer text-sm font-bold uppercase tracking-[0.2em] text-neutral-400 hover:text-white">
        Event log ({events.length})
      </summary>
      {events.length === 0 ? (
        <p className="mt-3 text-neutral-500">No events.</p>
      ) : (
        <ol className="mt-3 max-h-80 overflow-auto font-mono text-sm leading-relaxed">
          {[...events].reverse().map((e) => (
            <li key={e.seq} className="grid grid-cols-[3rem_6rem_9rem_1fr] gap-3 border-b border-neutral-800/60 py-1">
              <span className="text-neutral-600">#{e.seq}</span>
              <span className="text-neutral-500">{clock(e.at)}</span>
              <span className="truncate text-neutral-500">{e.agent_id ?? "job"}</span>
              <span className={TONE[e.type] ?? "text-neutral-200"}>{summary(e)}</span>
            </li>
          ))}
        </ol>
      )}
    </details>
  );
}
