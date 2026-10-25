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
      <div className="text-sm text-dim">{label}</div>
      <div className="mt-1 font-mono text-5xl font-black tabular-nums">{children}</div>
      {sub && <div className="mt-1 text-sm text-dim">{sub}</div>}
    </div>
  );
}

function Wordmark() {
  return (
    <Link href="/" className="text-lg font-extrabold tracking-[-0.02em] text-paper">
      Theseus
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
        <p className="mt-4 text-lg text-dim">
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
        <p className="mt-6 font-mono text-xl text-dim">
          Loading job {id}…
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
        <span className="text-dim">/</span>
        <span className="text-lg font-semibold">{job.library}</span>
        <StatusPill status={job.status} size="lg" />
        <span className="text-sm text-dim">runner {job.runner}</span>
        <span className="font-mono text-sm text-dim">{job.id}</span>
        <span className="ml-auto">
          {source === "fixture" ? <SampleBadge notice={notice} /> : notice && <span className="text-sm text-dim">{notice}</span>}
        </span>
      </header>

      {job.error && (
        <div className="panel border-div/70 p-4 text-lg">
          <span className="font-mono font-bold text-div">{job.error.code}</span>
          <span className="text-div">: {job.error.message}</span>
          {job.error.retryable && <span className="ml-2 text-dim">(retryable)</span>}
        </div>
      )}

      <Ticker text={latestSummary(events)} />

      <Pipeline steps={steps} />

      {/* Verifier wall + headline numbers */}
      <div className="grid grid-cols-[minmax(0,2fr)_minmax(0,1fr)] gap-4">
        <Card
          title="Verifier: C vs Rust, same inputs"
          right={<span className="text-sm text-dim">1 square = 1 input</span>}
        >
          {wall ? <VerifierWall wall={wall} holdout={holdout} /> : <p className="text-lg text-dim">Waiting for the first verify round.</p>}
        </Card>
        <div className="flex flex-col gap-4">
          <Stat label={`Match rate · baseline → ${stats.final ? "final" : "now"}`}>
            <span className="text-dim">{pct(stats.baseline)}</span>
            <span className="mx-2 text-3xl text-dim">→</span>
            <span className={stats.current !== null && stats.current >= 1 ? "text-match" : "text-paper"}>
              {stats.current === null ? "n/a" : <CountUp value={stats.current * 100} format={(x) => `${Math.round(x)}%`} />}
            </span>
          </Stat>
          <Stat label={preserved ? "Tested behaviors preserved" : "Behaviors tested"} sub={holdout ? `+ ${holdout.match}/${holdout.total} held-out` : undefined}>
            <span className={preserved ? "text-match" : "text-paper"}>
              {stats.behaviors === null ? "n/a" : <CountUp value={stats.behaviors} />}
            </span>
          </Stat>
          <Stat label="Memory bugs caught in C" sub={crashes[0]?.cve ? `${crashes[0].cve} · Rust returns a clean error` : undefined}>
            <span className={stats.crashes ? "text-crash" : "text-paper"}>{stats.crashes}</span>
          </Stat>
          {cost && (
            <Stat label="Model cost, this run">
              <span className="text-paper">${cost.total_usd.toFixed(cost.total_usd < 1 ? 3 : 2)}</span>
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
            <span className="text-base font-bold text-div">{openCount} open · click to inspect</span>
          ) : rows.length ? (
            <span className="text-base text-match">all {rows.length} repaired</span>
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
