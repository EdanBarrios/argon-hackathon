// Pure view-model helpers: turn a Job plus its events into what the run view shows.
import type { Job, JobEvent } from "./api";

export interface DivergenceRow {
  key: string;
  input: string;
  c_output: string;
  rust_output: string;
  function: string;
  resolved: boolean;
  agent_id: string | null;
}

/**
 * Prefer result.divergences (authoritative once completed). Before that, build
 * rows from `divergence` events: a row counts as resolved once a later `repair`
 * touches the same function, or a later `verify` for that agent hits 100%.
 */
export function divergenceRows(job: Job, events: JobEvent[]): DivergenceRow[] {
  if (job.result) {
    return job.result.divergences.map((d, i) => ({ ...d, key: `r${i}`, agent_id: null }));
  }
  return events
    .filter((e) => e.type === "divergence")
    .map((e) => {
      const fn = e.data.function ?? "?";
      const resolved = events.some(
        (later) =>
          later.seq > e.seq &&
          later.agent_id === e.agent_id &&
          ((later.type === "repair" && later.data.function === fn) ||
            (later.type === "verify" && (later.data.match_rate === 1 || later.data.divergences === 0))),
      );
      return {
        key: `e${e.seq}`,
        input: e.data.input ?? "",
        c_output: e.data.c_output ?? "",
        rust_output: e.data.rust_output ?? "",
        function: fn,
        resolved,
        agent_id: e.agent_id,
      };
    });
}

export interface CrashView {
  input: string;
  kind: string;
  rust_behavior: string;
  sanitizer_report: string | null;
}

const kindFrom = (s: string | undefined) => s?.match(/AddressSanitizer:\s*([\w-]+)/)?.[1] ?? "crash";

export function crashViews(job: Job, events: JobEvent[]): CrashView[] {
  if (job.result) {
    return job.result.crashes.map((c) => ({
      input: c.input,
      kind: kindFrom(c.sanitizer_report),
      rust_behavior: c.rust_behavior,
      sanitizer_report: c.sanitizer_report,
    }));
  }
  return events
    .filter((e) => e.type === "crash")
    .map((e) => ({
      input: e.data.input ?? "",
      kind: kindFrom(e.data.sanitizer),
      rust_behavior: e.data.rust_behavior ?? "?",
      sanitizer_report: null,
    }));
}

export interface Series {
  agent_id: string;
  label: string;
  points: { iteration: number; match_rate: number }[];
}

/** One series per agent from `verify` events. */
export function verifySeries(job: Job, events: JobEvent[]): Series[] {
  const byAgent = new Map<string, Series>();
  for (const e of events) {
    if (e.type !== "verify" || typeof e.data.match_rate !== "number") continue;
    const id = e.agent_id ?? "job";
    const label = job.agents.find((a) => a.id === id)?.module ?? id;
    const s = byAgent.get(id) ?? { agent_id: id, label, points: [] };
    s.points.push({ iteration: e.data.iteration ?? s.points.length + 1, match_rate: e.data.match_rate });
    byAgent.set(id, s);
  }
  return [...byAgent.values()];
}

export interface HeroStats {
  baseline: number | null;
  current: number | null;
  final: boolean;
  behaviors: number | null;
  crashes: number;
}

const mean = (xs: number[]) => (xs.length ? xs.reduce((a, b) => a + b, 0) / xs.length : null);

export function heroStats(job: Job, events: JobEvent[]): HeroStats {
  const crashes = crashViews(job, events).length;
  if (job.result) {
    return {
      baseline: job.result.baseline_match_rate,
      current: job.result.final_match_rate,
      final: true,
      behaviors: job.result.behaviors_tested,
      crashes,
    };
  }
  const series = verifySeries(job, events);
  const latestBehaviors = series.map((s) => {
    const last = [...events].reverse().find((e) => e.type === "verify" && (e.agent_id ?? "job") === s.agent_id);
    return last?.data.behaviors_tested ?? 0;
  });
  return {
    baseline: mean(series.map((s) => s.points[0].match_rate)),
    current: mean(job.agents.map((a) => a.match_rate)),
    final: false,
    behaviors: latestBehaviors.length ? latestBehaviors.reduce((a, b) => a + b, 0) : null,
    crashes,
  };
}

/** Split two strings into [common prefix, differing middle, common suffix] by code point. */
export function diffParts(a: string, b: string): [string[], string[]] {
  const A = Array.from(a);
  const B = Array.from(b);
  let p = 0;
  while (p < A.length && p < B.length && A[p] === B[p]) p++;
  let s = 0;
  while (s < A.length - p && s < B.length - p && A[A.length - 1 - s] === B[B.length - 1 - s]) s++;
  const split = (X: string[]) => [X.slice(0, p).join(""), X.slice(p, X.length - s).join(""), X.slice(X.length - s).join("")];
  return [split(A), split(B)];
}

export const pct = (x: number | null | undefined, digits = 0) =>
  x === null || x === undefined ? "n/a" : `${(x * 100).toFixed(digits)}%`;

/** HH:MM:SS straight from the ISO string, so server and client render the same text. */
export const clock = (iso: string) => iso.slice(11, 19);
