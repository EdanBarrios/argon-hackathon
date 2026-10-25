"use client";

import Link from "next/link";
import { usePollJob, type FixtureKind, type Snapshot } from "@/lib/api";
import { crashViews, divergenceRows, heroStats, holdoutResult, latestSummary, pct, pipelineSteps, wallState } from "@/lib/derive";
import { BaselineCompare } from "./baseline";
import { CrashCard } from "./crash-card";
import { Divergences } from "./divergences";
import { EventLog } from "./event-log";
import { Evidence } from "./evidence";
import { Lanes } from "./lanes";
import { Pipeline } from "./pipeline";
import { Ticker } from "./ticker";
import { TryPanel } from "./try-panel";
import { Card, CountUp, SampleBadge, StatusPill } from "./ui";
import { VerifierWall } from "./verifier-wall";

function Stat({ label, children, sub }: { label: string; children: React.ReactNode; sub?: React.ReactNode }) {
  return (
    <div className="panel rise px-5 py-4">
      <div className="font-mono text-xs uppercase tracking-[0.25em] text-dim">{label}</div>
      <div className="mt-1 font-mono text-5xl font-black tabular-nums">{children}</div>
      {sub && <div className="mt-1 text-sm text-neutral-400">{sub}</div>}
    </div>
  );
}

function Wordmark() {
  return (
    <Link href="/" className="font-mono text-3xl font-black tracking-[0.35em] text-white">
      THESEUS
    </Link>
  );
}

export function RunView({ id, fixture, initial }: { id: string; fixture: FixtureKind | null; initial?: Snapshot }) {
  const { job, events, cost, source, notice, error, setJob } = usePollJob(id, { fixture, initial });

  if (error) {
    return (
      <main className="mx-auto max-w-3xl p-10">
        <Wordmark />
        <p className="mt-6 text-xl text-div">{error}</p>
        <p className="mt-4 text-lg text-neutral-400">
          Try the recorded run: <Link className="underline" href={`/jobs/${id}?fixture=completed`}>completed</Link>,{" "}
          <Link className="underline" href={`/jobs/${id}?fixture=running`}>running</Link>, or{" "}
          <Link className="underline" href="/">go home</Link>.
        </p>
      </main>
    );
  }
  if (!job) {
    return (
      <main className="p-10">
        <Wordmark />
        <p className="mt-6 font-mono text-xl text-neutral-400">
          Loading job {id}
          <span className="cursor text-match">▌</span>
        </p>
      </main>
    );
  }

  const stats = heroStats(job, events);
  const rows = divergenceRows(job, events);
  const crashes = crashViews(job, events);
  const wall = wallState(job, events);
  const holdout = holdoutResult(events);
  const steps = pipelineSteps(job, events);
  const openCount = rows.filter((r) => !r.resolved).length;
  const preserved = stats.final && stats.current !== null && stats.current >= 1;

  return (
    <main className="mx-auto flex max-w-[1440px] flex-col gap-4 px-6 py-5">
      {/* Header */}
      <header className="flex flex-wrap items-center gap-x-5 gap-y-2">
        <Wordmark />
        <span className="font-mono text-2xl text-dim">▸</span>
        <span className="font-mono text-2xl font-semibold text-white">{job.library}</span>
        <StatusPill status={job.status} size="lg" />
        <span className="font-mono text-sm uppercase tracking-widest text-dim">runner {job.runner}</span>
        <span className="font-mono text-sm text-dim">{job.id}</span>
        <span className="ml-auto">
          {source === "fixture" ? <SampleBadge notice={notice} /> : notice && <span className="text-sm text-neutral-400">{notice}</span>}
        </span>
      </header>

      {job.error && (
        <div className="panel border-div/70 p-4 text-lg">
          <span className="font-mono font-bold text-div">{job.error.code}</span>
          <span className="text-red-100">: {job.error.message}</span>
          {job.error.retryable && <span className="ml-2 text-neutral-400">(retryable)</span>}
        </div>
      )}

      <Ticker text={latestSummary(events)} />

      <Pipeline steps={steps} />

      {/* Verifier wall + headline numbers */}
      <div className="grid grid-cols-[minmax(0,2fr)_minmax(0,1fr)] gap-4">
        <Card
          title="Verifier: C vs Rust, same inputs"
          right={<span className="font-mono text-xs uppercase tracking-[0.2em] text-dim">1 square = 1 input</span>}
        >
          {wall ? <VerifierWall wall={wall} holdout={holdout} /> : <p className="text-lg text-neutral-500">Waiting for the first verify round.</p>}
        </Card>
        <div className="flex flex-col gap-4">
          <Stat label={`Match rate · baseline → ${stats.final ? "final" : "now"}`}>
            <span className="text-neutral-500">{pct(stats.baseline)}</span>
            <span className="mx-2 text-3xl text-dim">→</span>
            <span className={stats.current !== null && stats.current >= 1 ? "text-match glow-match" : "text-white"}>
              {stats.current === null ? "n/a" : <CountUp value={stats.current * 100} format={(x) => `${Math.round(x)}%`} />}
            </span>
          </Stat>
          <Stat label={preserved ? "Tested behaviors preserved" : "Behaviors tested"} sub={holdout ? `+ ${holdout.match}/${holdout.total} held-out` : undefined}>
            <span className={preserved ? "text-match glow-match" : "text-white"}>
              {stats.behaviors === null ? "n/a" : <CountUp value={stats.behaviors} />}
            </span>
          </Stat>
          <Stat label="Memory bugs caught in C" sub={crashes[0]?.cve ? `${crashes[0].cve} · Rust returns a clean error` : undefined}>
            <span className={stats.crashes ? "text-crash glow-crash" : "text-white"}>{stats.crashes}</span>
          </Stat>
          {cost && (
            <Stat label="Model cost, this run">
              <span className="text-white">${cost.total_usd.toFixed(cost.total_usd < 1 ? 3 : 2)}</span>
            </Stat>
          )}
        </div>
      </div>

      {job.agents.length > 1 && (
        <Card title="Agents">
          <Lanes agents={job.agents} />
        </Card>
      )}

      <Card
        title="Divergences"
        right={
          openCount > 0 ? (
            <span className="font-mono text-base font-bold text-div glow-div">{openCount} open · click to inspect</span>
          ) : rows.length ? (
            <span className="font-mono text-base text-match">all {rows.length} repaired</span>
          ) : null
        }
      >
        <Divergences rows={rows} />
      </Card>

      <div className="grid grid-cols-2 gap-4">
        <Card title="Memory safety">
          <CrashCard crashes={crashes} />
        </Card>
        <Card title="Evidence report">
          <Evidence job={job} source={source} onJob={setJob} wall={wall} holdout={holdout} rows={rows} crashes={crashes} />
        </Card>
      </div>

      <Card title="Flash vs Argon">
        <BaselineCompare />
      </Card>

      <Card title="Try any input">
        <TryPanel compact />
      </Card>

      <EventLog events={events} />
    </main>
  );
}
