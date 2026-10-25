// Server-side copy of the fixtures, bundled at build time, so ?fixture= views
// render fully in the initial HTML (no client fetch needed).
import type { Job, JobEvent } from "./api";
import { buildFixture, type FixtureKind, type Snapshot } from "./fixtures";
import running from "@/public/fixtures/job-running.json";
import completed from "@/public/fixtures/job-completed.json";
import failed from "@/public/fixtures/job-failed.json";
import events from "@/public/fixtures/job-events.json";

const JOBS: Record<FixtureKind, unknown> = { running, completed, failed };

export function staticFixture(kind: FixtureKind): Snapshot {
  return buildFixture(kind, JOBS[kind] as Job, events as unknown as JobEvent[]);
}
