import type { Job, Source } from "@/lib/api";
import type { CrashView, DivergenceRow, WallState } from "@/lib/derive";
import { ApprovalPanel } from "./approval";
import { Mark } from "./plates";

interface Check {
  ok: boolean;
  text: string;
}

// The last row of the spine: what the PR's evidence report will claim, built only
// from what the verifier reported, and the one approve control.
export function Evidence({
  job,
  source,
  onJob,
  wall,
  holdout,
  rows,
  crashes,
}: {
  job: Job;
  source: Source;
  onJob: (j: Job) => void;
  wall: WallState | null;
  holdout: { match: number; total: number } | null;
  rows: DivergenceRow[];
  crashes: CrashView[];
}) {
  const latest = wall?.rounds[wall.rounds.length - 1];
  const checks: Check[] = [];
  if (latest) checks.push({ ok: latest.match === latest.total, text: `${latest.match}/${latest.total} inputs byte-identical to C` });
  if (holdout) checks.push({ ok: holdout.match === holdout.total, text: `${holdout.match}/${holdout.total} held-out inputs the model never saw` });
  if (rows.length) {
    const fixed = rows.filter((r) => r.resolved).length;
    checks.push({ ok: fixed === rows.length, text: `${fixed}/${rows.length} divergences found and repaired` });
  }
  for (const c of crashes) checks.push({ ok: true, text: `${c.cve ?? "Crash input"}: C ${c.kind}, Rust ${c.rust_behavior}` });

  return (
    <div className="grid items-start gap-6 border-t border-line px-1 py-6 md:grid-cols-[minmax(0,1fr)_auto]">
      <div>
        <h2 className="mb-3 text-xl font-bold">The evidence report this PR will carry</h2>
        {checks.length ? (
          <ul className="flex flex-col gap-2">
            {checks.map((c, i) => (
              <li key={i} className="flex items-center gap-3 text-lg">
                <Mark state={c.ok ? "registered" : "apart"} size={12} />
                <span>{c.text}</span>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-lg text-dim">Evidence appears as the verifier reports.</p>
        )}
      </div>
      <div className="flex max-w-sm flex-col gap-3 md:items-end md:text-right">
        <ApprovalPanel job={job} source={source} onJob={onJob} />
        <p className="text-sm text-dim">A human approves before anything is pushed. The model never grades its own work.</p>
      </div>
    </div>
  );
}
