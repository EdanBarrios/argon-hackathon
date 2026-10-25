import type { Job, Source } from "@/lib/api";
import type { CrashView, DivergenceRow, WallState } from "@/lib/derive";
import { ApprovalPanel } from "./approval";

interface Check {
  ok: boolean;
  text: string;
}

// What the PR's evidence report will claim, built only from what the verifier reported.
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
    <div className="flex flex-col gap-4">
      {checks.length ? (
        <ul className="flex flex-col gap-2">
          {checks.map((c, i) => (
            <li key={i} className="rise flex items-start gap-3 text-lg" style={{ animationDelay: `${i * 120}ms` }}>
              <span className={`font-mono font-bold ${c.ok ? "text-match" : "text-div"}`}>{c.ok ? "✓" : "✗"}</span>
              <span className="text-paper">{c.text}</span>
            </li>
          ))}
        </ul>
      ) : (
        <p className="text-lg text-dim">Evidence appears as the verifier reports.</p>
      )}
      <ApprovalPanel job={job} source={source} onJob={onJob} />
      <p className="text-sm text-dim">
        A human approves before anything is pushed. The model never grades its own work.
      </p>
    </div>
  );
}
