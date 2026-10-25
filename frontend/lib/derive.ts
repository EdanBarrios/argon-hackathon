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
  repair: RepairView | null;
  /** True when the pipeline inferred the function from the divergence pattern. */
  function_guessed: boolean;
}

const guessed = (events: JobEvent[], input: string) =>
  events.some(
    (e) => e.type === "divergence" && e.data.input === input && typeof e.data.function_source === "string" && /guess/i.test(e.data.function_source),
  );

export interface RepairView {
  iteration: number | null;
  summary: string;
  model: string | null;
  seconds: number | null;
  /** Deduped divergence patterns the repair prompt was given (from the "started" event). */
  patterns: string[];
}

/**
 * The `repair` event after `afterSeq` that touched `fn` (any agent when `agentId` is null).
 * A repair round may list several `functions`; prefer its `phase: "done"` event.
 */
function findRepair(events: JobEvent[], fn: string, agentId: string | null, afterSeq = 0): RepairView | null {
  const touches = (r: JobEvent) => {
    const fns = Array.isArray(r.data.functions) ? (r.data.functions as string[]) : [];
    return r.data.function === fn || fns.includes(fn);
  };
  const matching = events.filter(
    (r) =>
      r.type === "repair" &&
      r.seq > afterSeq &&
      (agentId === null || r.agent_id === agentId) &&
      typeof r.data.summary === "string",
  );
  // The "done" event often names only one function, so match the round through its "started" event.
  const started = matching.find(touches);
  if (!started) return null;
  const done = matching.find((r) => r.data.phase === "done" && r.data.iteration === started.data.iteration);
  const e = done ?? started;
  return {
    iteration: e.data.iteration ?? null,
    summary: e.data.summary as string,
    model: typeof started.data.model === "string" ? started.data.model : null,
    seconds: typeof done?.data.seconds === "number" ? done.data.seconds : null,
    patterns: Array.isArray(started.data.patterns) ? (started.data.patterns as string[]) : [],
  };
}

/**
 * Prefer result.divergences (authoritative once completed). Before that, build
 * rows from `divergence` events: a row counts as resolved once a later `repair`
 * touches the same function, or a later `verify` for that agent hits 100%.
 */
export function divergenceRows(job: Job, events: JobEvent[]): DivergenceRow[] {
  if (job.result) {
    return job.result.divergences.map((d, i) => ({
      ...d,
      key: `r${i}`,
      agent_id: null,
      repair: findRepair(events, d.function, null),
      function_guessed: guessed(events, d.input),
    }));
  }
  // A repair counts only once the model has returned a file: the functions a
  // round was sent (its "started" event) become repaired at its "done" event.
  const repairedAt = new Map<string, number>();
  for (const done of events) {
    if (done.type !== "repair" || done.data.phase === "started") continue;
    const started = events.find((s) => s.type === "repair" && s.data.phase === "started" && s.data.iteration === done.data.iteration);
    const fns = Array.isArray(started?.data.functions) ? (started.data.functions as string[]) : [];
    for (const f of [...fns, done.data.function].filter((x): x is string => typeof x === "string")) {
      if (!repairedAt.has(f)) repairedAt.set(f, done.seq);
    }
  }
  return events
    .filter((e) => e.type === "divergence")
    .map((e) => {
      const fn = e.data.function ?? "?";
      const repair = findRepair(events, fn, e.agent_id, e.seq);
      const resolved =
        (repairedAt.get(fn) ?? -1) > e.seq ||
        events.some(
          (later) =>
            later.seq > e.seq &&
            later.agent_id === e.agent_id &&
            later.type === "verify" &&
            later.data.compile_ok !== false &&
            (later.data.match_rate === 1 || later.data.divergences === 0),
        );
      return {
        key: `e${e.seq}`,
        input: e.data.input ?? "",
        c_output: e.data.c_output ?? "",
        rust_output: e.data.rust_output ?? "",
        function: fn,
        resolved,
        agent_id: e.agent_id,
        repair,
        function_guessed: guessed(events, e.data.input ?? ""),
      };
    });
}

export interface CrashView {
  input: string;
  kind: string;
  rust_behavior: string;
  sanitizer_report: string | null;
  cve: string | null;
  c_version: string | null;
}

/** CVE id and C version live on the crash event, not on result.crashes. */
function crashMeta(events: JobEvent[], input: string) {
  const e = events.find((x) => x.type === "crash" && x.data.input === input);
  return {
    cve: typeof e?.data.cve === "string" ? e.data.cve : null,
    c_version: typeof e?.data.c_version === "string" ? e.data.c_version : null,
  };
}

const kindFrom = (s: string | undefined) => s?.match(/AddressSanitizer:\s*([\w-]+)/)?.[1] ?? "crash";

export function crashViews(job: Job, events: JobEvent[]): CrashView[] {
  if (job.result) {
    return job.result.crashes.map((c) => ({
      input: c.input,
      kind: kindFrom(c.sanitizer_report),
      rust_behavior: c.rust_behavior,
      sanitizer_report: c.sanitizer_report,
      ...crashMeta(events, c.input),
    }));
  }
  return events
    .filter((e) => e.type === "crash")
    .map((e) => ({
      input: e.data.input ?? "",
      kind: kindFrom(e.data.sanitizer),
      rust_behavior: e.data.rust_behavior ?? "?",
      sanitizer_report: null,
      ...crashMeta(events, e.data.input ?? ""),
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
    if (e.data.holdout || e.data.final_full) continue; // extra checks on other input sets, not a repair round
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

// ---------------------------------------------------------------------------
// Pipeline, verifier wall, held-out: everything below is read from real events.

export type StepTone = "neutral" | "match" | "div" | "crash";

export interface PipelineStep {
  key: string;
  label: string;
  detail: string;
  tone: StepTone;
  active: boolean;
}

const isHoldout = (e: JobEvent) => e.type === "verify" && e.data.holdout === true;

/** One step per pipeline event, in order. A trailing active step while the job is still working. */
export function pipelineSteps(job: Job, events: JobEvent[]): PipelineStep[] {
  const steps: PipelineStep[] = [];
  const repairs = new Map<number, PipelineStep>();
  for (const e of events) {
    const d = e.data;
    if (e.type === "translate") {
      const fromSaved = typeof d.source_label === "string";
      steps.push({
        key: `t${e.seq}`,
        label: fromSaved ? (d.source_label as string) : "Argon translates",
        detail: [typeof d.rust_lines === "number" ? `${d.rust_lines} lines of Rust` : null, fromSaved ? (d.source_model as string) : d.module]
          .filter(Boolean)
          .join(" · "),
        tone: "neutral",
        active: false,
      });
    } else if (e.type === "verify" && d.compile_ok === false) {
      // The model's file did not compile; the pipeline sends the compiler errors back.
      steps.push({ key: `v${e.seq}`, label: "Build failed", detail: "compiler errors sent back to the model", tone: "div", active: false });
    } else if (e.type === "verify") {
      const n = d.behaviors_tested ?? 0;
      const ok = Math.round((d.match_rate ?? 0) * n);
      const full = d.match_rate === 1;
      steps.push({
        key: `v${e.seq}`,
        label: isHoldout(e) ? "Held-out check" : `Verify #${d.iteration ?? steps.length}`,
        detail: isHoldout(e) ? `${ok}/${n} never-seen inputs` : `${ok}/${n} identical · ${pct(d.match_rate, 1)}`,
        tone: full ? "match" : "div",
        active: false,
      });
    } else if (e.type === "repair") {
      const it = d.iteration ?? 0;
      const existing = repairs.get(it);
      const done = d.phase === "done" || d.phase === undefined;
      const model = typeof d.model === "string" ? d.model : existing ? null : "Argon";
      const step: PipelineStep = existing ?? {
        key: `r${e.seq}`,
        label: "Argon repair",
        detail: "",
        tone: "neutral",
        active: false,
      };
      step.active = !done;
      step.detail = done
        ? typeof d.seconds === "number"
          ? `fixed in ${Math.round(d.seconds)} s`
          : `round ${it}`
        : `${Array.isArray(d.patterns) ? d.patterns.length : "?"} divergence patterns → ${model ?? "model"}`;
      if (!existing) {
        repairs.set(it, step);
        steps.push(step);
      }
    } else if (e.type === "crash") {
      steps.push({
        key: `c${e.seq}`,
        label: typeof d.cve === "string" ? d.cve : "Crash check",
        detail: "C crashes · Rust clean error",
        tone: "crash",
        active: false,
      });
    } else if (e.type === "approval_required" || e.type === "approved") {
      const approved = e.type === "approved" || !!job.approval.approved_by;
      const prev = steps.find((s) => s.key === "pr");
      const step = prev ?? { key: "pr", label: "Pull request", detail: "", tone: "neutral" as StepTone, active: false };
      step.detail = job.approval.pr_url ? "opened with evidence" : approved ? "approved" : "awaiting human approval";
      step.tone = job.approval.pr_url ? "match" : "neutral";
      if (!prev) steps.push(step);
    }
  }
  const working = job.status === "running" || job.status === "verifying" || job.status === "queued";
  if (working && !steps.some((s) => s.active)) {
    steps.push({
      key: "now",
      label: job.status === "verifying" ? "Verifier running" : job.status === "queued" ? "Queued" : "Working",
      detail: job.status === "verifying" ? "C and Rust on every input" : "",
      tone: "neutral",
      active: true,
    });
  }
  return steps;
}

export interface WallRound {
  /** Total inputs in this verify round. */
  total: number;
  match: number;
  div: number;
  crash: number;
  iteration: number | null;
}

export interface WallState {
  /** Every main (non-held-out) verify round, in order. */
  rounds: WallRound[];
  /** True while a verify round is in progress and its result has not landed yet. */
  scanning: boolean;
}

/** Counts from each non-held-out verify event. Order of cells is not meaningful. */
export function wallState(job: Job, events: JobEvent[]): WallState | null {
  // A round whose build failed tested nothing, so it has no marks to draw.
  const main = events.filter((e) => e.type === "verify" && !isHoldout(e) && e.data.compile_ok !== false);
  const last = main[main.length - 1];
  const lastStatus = [...events].reverse().find((e) => e.type === "job_status");
  const scanning = job.status === "verifying" && !!lastStatus && (!last || lastStatus.seq > last.seq);
  if (!main.length && !scanning) return null;
  const rounds = main.map((e) => {
    const total = e.data.behaviors_tested ?? 0;
    const div = typeof e.data.divergences === "number" ? e.data.divergences : 0;
    const crash = typeof e.data.crashes === "number" ? e.data.crashes : 0;
    return { total, match: Math.max(0, total - div - crash), div, crash, iteration: e.data.iteration ?? null };
  });
  return { rounds, scanning };
}

export function holdoutResult(events: JobEvent[]): { match: number; total: number } | null {
  const e = [...events].reverse().find(isHoldout);
  if (!e) return null;
  const total = e.data.behaviors_tested ?? 0;
  return { match: Math.round((e.data.match_rate ?? 0) * total), total };
}

/** Latest human-readable line from the pipeline. */
export function latestSummary(events: JobEvent[]): string | null {
  const e = [...events].reverse().find((x) => typeof x.data.summary === "string");
  return e ? (e.data.summary as string) : null;
}

// ---------------------------------------------------------------------------
// Repair timeline and activity feed: what the pipeline is doing, from real events.

export interface RepairRound {
  iteration: number;
  model: string;
  /** "divergences": the verifier's deduped patterns; "compile": the compiler's errors. */
  kind: "divergences" | "compile";
  patterns: string[];
  functions: string[];
  startedAt: string;
  seconds: number | null;
  done: boolean;
  /** First lines of the compiler output when this round's file did not build. */
  buildError: string | null;
  buildErrorCount: number;
  /** The verify that followed this round, when it built. */
  verified: { match: number; total: number } | null;
}

export function repairRounds(events: JobEvent[]): RepairRound[] {
  const rounds: RepairRound[] = [];
  for (const e of events) {
    const d = e.data;
    if (e.type === "repair" && d.phase === "started") {
      const fns = Array.isArray(d.functions) ? (d.functions as string[]) : [];
      const compile = fns.some((f) => /compile/i.test(f)) || /compiler/i.test(String(d.summary ?? ""));
      rounds.push({
        iteration: d.iteration ?? rounds.length + 2,
        model: typeof d.model === "string" ? d.model : "the model",
        kind: compile ? "compile" : "divergences",
        patterns: Array.isArray(d.patterns) ? (d.patterns as string[]) : [],
        functions: compile ? [] : fns,
        startedAt: e.at,
        seconds: null,
        done: false,
        buildError: null,
        buildErrorCount: 0,
        verified: null,
      });
    } else if (e.type === "repair" && (d.phase === "done" || d.phase === undefined)) {
      const r = rounds.find((x) => x.iteration === d.iteration) ?? rounds[rounds.length - 1];
      if (r) {
        r.done = true;
        r.seconds = typeof d.seconds === "number" ? d.seconds : null;
      }
    } else if (e.type === "verify" && !d.holdout) {
      const r = rounds.find((x) => x.iteration === d.iteration);
      if (!r) continue;
      if (d.compile_ok === false) {
        const errs = typeof d.errors === "string" ? d.errors : "";
        const lines = errs.split("\n").filter((l) => l.trim());
        r.buildError = lines.slice(0, 2).join("\n") || "cargo build failed";
        r.buildErrorCount = lines.filter((l) => /^error/.test(l.trim())).length;
      } else if (typeof d.match_rate === "number") {
        const total = d.behaviors_tested ?? 0;
        r.verified = { match: Math.round(d.match_rate * total), total };
      }
    }
  }
  return rounds;
}

export interface ActivityLine {
  seq: number;
  at: string;
  text: string;
  tone: StepTone;
}

/** Plain-English lines for the activity feed, oldest first. */
export function activityLines(events: JobEvent[]): ActivityLine[] {
  const out: ActivityLine[] = [];
  const push = (e: JobEvent, text: string, tone: StepTone = "neutral") => out.push({ seq: e.seq, at: e.at, text, tone });
  for (const e of events) {
    const d = e.data;
    switch (e.type) {
      case "job_status":
        if (d.status === "running" && Array.isArray(d.plan)) push(e, "Run started");
        else if (d.status === "completed") push(e, "Run complete", "match");
        else if (d.status === "failed") push(e, "Run failed", "div");
        break;
      case "translate":
        if (d.phase === "loaded" && typeof d.source_label === "string")
          push(e, `Loaded ${d.source_label}: ${d.rust_lines ?? "?"} lines of Rust by ${d.source_model ?? "a model"}`);
        else if (typeof d.summary === "string") push(e, d.summary);
        break;
      case "verify":
        if (d.compile_ok === false) push(e, "The repaired file did not compile; the compiler errors go back to the model", "div");
        else if (typeof d.summary === "string") push(e, d.summary.replace(/^verify\.sh: /, "Verifier: "), d.match_rate === 1 ? "match" : "div");
        break;
      case "divergence":
        push(e, `Divergence on ${d.input ?? "an input"}: C ${String(d.c_output ?? "").replace(/\t/g, " ")}, Rust ${d.rust_output ?? ""}`, "div");
        break;
      case "repair":
        if (d.phase === "started") {
          const n = Array.isArray(d.patterns) ? d.patterns.length : 0;
          push(e, n ? `Sent ${n} divergence patterns to ${d.model ?? "the model"}` : `Sent the compiler errors to ${d.model ?? "the model"}`);
        } else if (typeof d.summary === "string") push(e, d.summary, "match");
        break;
      case "crash":
        push(e, `${d.cve ?? "Crash input"}: old C crashes under AddressSanitizer; the Rust returns a clean error`, "crash");
        break;
      case "approval_required":
        push(e, "Waiting for a person to approve the pull request");
        break;
      case "approved":
        push(e, `Approved by ${d.approved_by ?? "a reviewer"}`, "match");
        break;
    }
  }
  return out;
}
