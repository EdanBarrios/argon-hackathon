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
  divergence: "text-div",
  crash: "text-crash",
  approved: "text-match",
};

export function EventLog({ events }: { events: JobEvent[] }) {
  return (
    <details className="rounded-xl border border-line bg-panel p-5">
      <summary className="cursor-pointer text-base font-bold text-dim hover:text-paper">
        Event log ({events.length})
      </summary>
      {events.length === 0 ? (
        <p className="mt-3 text-dim">No events.</p>
      ) : (
        <ol className="mt-3 max-h-80 overflow-auto font-mono text-sm leading-relaxed">
          {[...events].reverse().map((e) => (
            <li key={e.seq} className="grid grid-cols-[3rem_6rem_9rem_1fr] gap-3 border-b border-line/60 py-1">
              <span className="text-dim">#{e.seq}</span>
              <span className="text-dim">{clock(e.at)}</span>
              <span className="truncate text-dim">{e.agent_id ?? "job"}</span>
              <span className={TONE[e.type] ?? "text-paper"}>{summary(e)}</span>
            </li>
          ))}
        </ol>
      )}
    </details>
  );
}
