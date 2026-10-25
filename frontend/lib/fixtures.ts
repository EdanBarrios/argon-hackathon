// Pure fixture helpers, safe to import from server and client code.
import type { Job, JobEvent } from "./api";

export const FIXTURE_KINDS = ["running", "completed", "failed"] as const;
export type FixtureKind = (typeof FIXTURE_KINDS)[number];
export const isFixtureKind = (v: unknown): v is FixtureKind =>
  typeof v === "string" && (FIXTURE_KINDS as readonly string[]).includes(v);

export interface Snapshot {
  job: Job;
  events: JobEvent[];
}

/**
 * Builds a consistent snapshot from raw fixture files.
 * - running: job-running.json shows printer at iteration 1 (0.64), which matches
 *   events up to seq 7, before the print_number repair. Cutting there leaves the
 *   divergence open, so the red row is visible.
 * - completed: all events.
 * - failed: job-failed.json is a different job id, so no events.
 */
export function buildFixture(kind: FixtureKind, job: Job, events: JobEvent[]): Snapshot {
  if (kind === "running") return { job, events: events.filter((e) => e.seq <= 7) };
  if (kind === "completed") return { job, events };
  return { job, events: [] };
}
