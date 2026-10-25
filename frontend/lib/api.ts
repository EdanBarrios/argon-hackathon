// Data layer for the Theseus UI. Types mirror contracts/fixtures/*.json exactly.
// If the API is unreachable (or ?fixture= is set) we fall back to /fixtures/ so
// the demo keeps working when the backend dies.

import { useEffect, useRef, useState } from "react";
import { buildFixture, type FixtureKind, type Snapshot } from "./fixtures";

// ---------- Models (see contracts/README.md) ----------

export type Status = "queued" | "running" | "verifying" | "completed" | "failed" | "cancelled";
export type Runner = "local" | "remote";

export interface Agent {
  id: string;
  module: string;
  status: Status;
  iteration: number;
  match_rate: number;
  interaction_id: string | null;
}

export interface Divergence {
  input: string;
  c_output: string;
  rust_output: string;
  function: string;
  resolved: boolean;
}

export interface Crash {
  input: string;
  sanitizer_report: string;
  rust_behavior: string;
}

export interface Result {
  baseline_match_rate: number;
  final_match_rate: number;
  behaviors_tested: number;
  divergences: Divergence[];
  crashes: Crash[];
}

export interface Approval {
  required: boolean;
  approved_by: string | null;
  pr_url: string | null;
}

export interface JobError {
  code: string;
  message: string;
  agent_id: string | null;
  retryable: boolean;
}

export interface Job {
  id: string;
  library: string;
  status: Status;
  created_at: string;
  updated_at: string;
  runner: Runner;
  agents: Agent[];
  result: Result | null;
  approval: Approval;
  error: JobError | null;
}

export type EventType =
  | "job_status"
  | "translate"
  | "verify"
  | "divergence"
  | "crash"
  | "repair"
  | "approval_required"
  | "approved";

/** Known `data` fields per event type. All optional so unexpected payloads never crash the UI. */
export interface EventData {
  status?: Status;
  plan?: string[];
  module?: string;
  iteration?: number;
  functions?: string[];
  match_rate?: number;
  behaviors_tested?: number;
  divergences?: number;
  crashes?: number;
  input?: string;
  c_output?: string;
  rust_output?: string;
  function?: string;
  side?: "c" | "rust";
  sanitizer?: string;
  rust_behavior?: string;
  summary?: string;
  final_match_rate?: number;
  action?: string;
  evidence?: string;
  approved_by?: string;
  pr_url?: string;
  [key: string]: unknown;
}

export interface JobEvent {
  seq: number;
  at: string;
  job_id: string;
  agent_id: string | null;
  type: EventType;
  data: EventData;
}

export interface ModelCost {
  model: string;
  input_tokens: number;
  cached_tokens: number;
  output_tokens: number;
  thought_tokens: number;
  usd: number;
}

export interface JobCost {
  job_id: string;
  models: ModelCost[];
  total_usd: number;
}

export interface CreateJobBody {
  library: string;
  modules?: string[];
}

export const TERMINAL: Status[] = ["completed", "failed", "cancelled"];
export const isTerminal = (s: Status) => TERMINAL.includes(s);

// ---------- HTTP ----------

export const API_URL = (process.env.NEXT_PUBLIC_API_URL || "http://localhost:8000").replace(/\/$/, "");

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message);
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    ...init,
    headers: { "Content-Type": "application/json", ...init?.headers },
    cache: "no-store",
  });
  if (!res.ok) throw new ApiError(res.status, `${init?.method ?? "GET"} ${path} -> ${res.status}`);
  return (await res.json()) as T;
}

export const createJob = (body: CreateJobBody) =>
  request<Job>("/jobs", { method: "POST", body: JSON.stringify(body) });

export const getJob = (id: string) => request<Job>(`/jobs/${encodeURIComponent(id)}`);

export const getEvents = (id: string, after = 0) =>
  request<JobEvent[]>(`/jobs/${encodeURIComponent(id)}/events?after=${after}`);

export const getResult = (id: string) => request<Job>(`/jobs/${encodeURIComponent(id)}/result`);

export const approve = (id: string, approvedBy: string) =>
  request<Job>(`/jobs/${encodeURIComponent(id)}/approve`, {
    method: "POST",
    body: JSON.stringify({ approved_by: approvedBy }),
  });

/** Live cost. Returns null if the endpoint is missing (404) so the UI can hide it. */
export async function getCost(id: string): Promise<JobCost | null> {
  try {
    return await request<JobCost>(`/jobs/${encodeURIComponent(id)}/cost`);
  } catch (e) {
    if (e instanceof ApiError && e.status === 404) return null;
    throw e;
  }
}

/** A network failure (backend down, CORS, DNS) as opposed to an HTTP error. */
const isUnreachable = (e: unknown) => !(e instanceof ApiError);

// ---------- Fixtures ----------

export { FIXTURE_KINDS, isFixtureKind, buildFixture } from "./fixtures";
export type { FixtureKind, Snapshot } from "./fixtures";

export async function loadFixture(kind: FixtureKind): Promise<Snapshot> {
  const get = async <T,>(name: string) => (await (await fetch(`/fixtures/${name}`)).json()) as T;
  const [job, events] = await Promise.all([get<Job>(`job-${kind}.json`), get<JobEvent[]>("job-events.json")]);
  return buildFixture(kind, job, events);
}

// ---------- Polling hook ----------

export type Source = "api" | "fixture";

export interface PollState {
  job: Job | null;
  events: JobEvent[];
  cost: JobCost | null;
  source: Source;
  /** Why we are on sample data, or why the last poll failed. */
  notice: string | null;
  /** Fatal error with no data to show (e.g. job not found). */
  error: string | null;
}

export interface PollOptions {
  fixture?: FixtureKind | null;
  initial?: Snapshot;
  intervalMs?: number;
}

/**
 * Polls GET /jobs/{id} and /events?after= every second until the job is terminal,
 * then fetches /result once. Falls back to the completed fixture if the API is
 * unreachable before any data arrives.
 */
export function usePollJob(id: string, opts: PollOptions = {}) {
  const { fixture = null, initial, intervalMs = 1000 } = opts;
  const [state, setState] = useState<PollState>(() => ({
    job: initial?.job ?? null,
    events: initial?.events ?? [],
    cost: null,
    source: fixture ? "fixture" : "api",
    notice: fixture ? `?fixture=${fixture}` : null,
    error: null,
  }));
  const lastSeq = useRef(0);
  const costMissing = useRef(false);

  useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    lastSeq.current = 0;
    costMissing.current = false;

    const fallBackTo = async (kind: FixtureKind, notice: string) => {
      try {
        const snap = await loadFixture(kind);
        if (!cancelled) setState({ ...snap, cost: null, source: "fixture", notice, error: null });
      } catch {
        if (!cancelled) setState((s) => ({ ...s, error: "API unreachable and sample data failed to load." }));
      }
    };

    if (fixture) {
      if (!initial) void fallBackTo(fixture, `?fixture=${fixture}`);
      return () => {
        cancelled = true;
      };
    }

    let gotData = false;
    const tick = async () => {
      try {
        let job = await getJob(id);
        const fresh = await getEvents(id, lastSeq.current);
        if (fresh.length) lastSeq.current = Math.max(lastSeq.current, ...fresh.map((e) => e.seq));
        const terminal = isTerminal(job.status);
        if (job.status === "completed" && !job.result) job = await getResult(id);

        let cost: JobCost | null | undefined;
        if (!costMissing.current) {
          cost = await getCost(id).catch(() => undefined);
          if (cost === null) costMissing.current = true;
        }
        if (cancelled) return;
        gotData = true;
        setState((s) => ({
          job,
          events: fresh.length ? [...s.events, ...fresh.filter((e) => e.seq > (s.events.at(-1)?.seq ?? 0))] : s.events,
          cost: cost === undefined ? s.cost : cost,
          source: "api",
          notice: null,
          error: null,
        }));
        if (terminal) return; // stop polling
      } catch (e) {
        if (cancelled) return;
        if (!gotData && isUnreachable(e)) {
          await fallBackTo("completed", `API unreachable at ${API_URL}`);
          return;
        }
        if (!gotData && e instanceof ApiError && e.status === 404) {
          setState((s) => ({ ...s, error: `Job "${id}" not found at ${API_URL}.` }));
          return;
        }
        // Mid-run failure: keep the last good data on screen and retry.
        setState((s) => ({ ...s, notice: `Connection lost (${(e as Error).message}); retrying` }));
      }
      if (!cancelled) timer = setTimeout(tick, intervalMs);
    };
    void tick();

    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
    };
  }, [id, fixture, initial, intervalMs]);

  /** Lets the approval panel push the job returned by POST /approve into state. */
  const setJob = (job: Job) => setState((s) => ({ ...s, job }));

  return { ...state, setJob };
}
