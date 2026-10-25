import type { Agent } from "@/lib/api";
import { pct } from "@/lib/derive";
import { StatusPill } from "./ui";

/** Repair budget is 5 rounds (see job-failed.json), so show at least 5 dots. */
const MIN_DOTS = 5;

function Dots({ agent }: { agent: Agent }) {
  const n = Math.max(MIN_DOTS, agent.iteration);
  const active = agent.status === "running" || agent.status === "verifying";
  return (
    <div className="flex gap-1.5" aria-label={`iteration ${agent.iteration}`}>
      {Array.from({ length: n }, (_, i) => {
        const it = i + 1;
        const done = it < agent.iteration || (it === agent.iteration && !active);
        const current = it === agent.iteration && active;
        return (
          <span
            key={it}
            className={`h-3 w-3 rounded-full ${
              current ? "animate-pulse bg-white" : done ? "bg-neutral-300" : "border border-neutral-700"
            }`}
          />
        );
      })}
    </div>
  );
}

export function Lanes({ agents }: { agents: Agent[] }) {
  if (!agents.length) return <p className="text-lg text-neutral-500">Waiting for the planner to assign modules...</p>;
  return (
    <div className="flex flex-col divide-y divide-neutral-800">
      {agents.map((a) => {
        const full = a.match_rate >= 1;
        return (
          <div key={a.id} className="grid grid-cols-[minmax(9rem,16rem)_8rem_auto_1fr_6rem] items-center gap-5 py-3">
            <div className="truncate text-2xl font-semibold">{a.module}</div>
            <div>
              <StatusPill status={a.status} />
            </div>
            <Dots agent={a} />
            <div className="min-w-0">
              <div className="h-4 overflow-hidden rounded-full bg-neutral-800">
                <div
                  className={`h-full rounded-full transition-[width] duration-700 ease-out ${
                    a.status === "failed" ? "bg-red-500" : full ? "bg-green-500" : "bg-green-600/70"
                  }`}
                  style={{ width: `${Math.max(0, Math.min(1, a.match_rate)) * 100}%` }}
                />
              </div>
              <div className="mt-1 truncate font-mono text-xs text-neutral-500">
                {a.interaction_id ?? "no interaction yet"} · iter {a.iteration}
              </div>
            </div>
            <div className={`text-right text-2xl font-bold tabular-nums ${full ? "text-green-400" : "text-white"}`}>
              {pct(a.match_rate)}
            </div>
          </div>
        );
      })}
    </div>
  );
}
