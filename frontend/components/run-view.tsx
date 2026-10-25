"use client";

import Link from "next/link";
import { usePollJob, type FixtureKind, type Snapshot } from "@/lib/api";
import { crashViews, divergenceRows, heroStats, pct, verifySeries } from "@/lib/derive";
import { ApprovalPanel } from "./approval";
import { MatchChart } from "./chart";
import { CrashCard } from "./crash-card";
import { Divergences } from "./divergences";
import { EventLog } from "./event-log";
import { Lanes } from "./lanes";
import { Card, SampleBadge, StatusPill } from "./ui";

function Stat({ label, value, tone = "text-white" }: { label: string; value: string; tone?: string }) {
  return (
    <div className="rounded-xl border border-neutral-800 bg-neutral-900 px-5 py-4">
      <div className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">{label}</div>
      <div className={`mt-1 text-5xl font-black tabular-nums ${tone}`}>{value}</div>
    </div>
  );
}

function BaselineGrid() {
  const cells = ["Flash, single pass", "Flash + repair loop", "Argon, single pass", "Argon + repair loop"];
  return (
    <div>
      <p className="mb-3 text-neutral-400">
        Same harness, prompts and repair budget. Metrics: match rate, behaviors tested, rounds to 100%, crashes, time, cost.
      </p>
      <div className="grid grid-cols-2 gap-3">
        {cells.map((c) => (
          <div key={c} className="rounded-lg border border-dashed border-neutral-700 p-4">
            <div className="text-lg font-semibold">{c}</div>
            <div className="mt-1 font-mono text-sm uppercase tracking-widest text-neutral-500">pending baseline runs</div>
          </div>
        ))}
      </div>
    </div>
  );
}

export function RunView({ id, fixture, initial }: { id: string; fixture: FixtureKind | null; initial?: Snapshot }) {
  const { job, events, cost, source, notice, error, setJob } = usePollJob(id, { fixture, initial });

  if (error) {
    return (
      <main className="mx-auto max-w-3xl p-10">
        <h1 className="text-4xl font-black tracking-[0.3em]">THESEUS</h1>
        <p className="mt-6 text-xl text-red-400">{error}</p>
        <p className="mt-4 text-lg text-neutral-400">
          Try sample data: <Link className="underline" href={`/jobs/${id}?fixture=completed`}>completed</Link>,{" "}
          <Link className="underline" href={`/jobs/${id}?fixture=running`}>running</Link>, or{" "}
          <Link className="underline" href="/">go home</Link>.
        </p>
      </main>
    );
  }
  if (!job) {
    return (
      <main className="p-10">
        <h1 className="text-4xl font-black tracking-[0.3em]">THESEUS</h1>
        <p className="mt-6 animate-pulse text-xl text-neutral-400">Loading job {id}...</p>
      </main>
    );
  }

  const stats = heroStats(job, events);
  const rows = divergenceRows(job, events);
  const openCount = rows.filter((r) => !r.resolved).length;

  return (
    <main className="mx-auto flex max-w-[1400px] flex-col gap-5 px-6 py-5">
      {/* Header */}
      <header className="flex flex-wrap items-center gap-x-6 gap-y-2">
        <Link href="/" className="text-4xl font-black tracking-[0.3em] text-white">
          THESEUS
        </Link>
        <span className="text-3xl font-semibold text-neutral-200">{job.library}</span>
        <StatusPill status={job.status} size="lg" />
        <span className="rounded-full border border-neutral-700 px-3 py-1 text-sm uppercase tracking-widest text-neutral-300">
          runner: {job.runner}
        </span>
        <span className="font-mono text-sm text-neutral-500">{job.id}</span>
        <span className="ml-auto">{source === "fixture" ? <SampleBadge notice={notice} /> : notice && <span className="text-sm text-neutral-400">{notice}</span>}</span>
      </header>

      {job.error && (
        <div className="rounded-xl border border-red-500 bg-red-500/10 p-4 text-lg">
          <span className="font-mono font-bold text-red-300">{job.error.code}</span>
          <span className="text-red-100">: {job.error.message}</span>
          {job.error.retryable && <span className="ml-2 text-neutral-400">(retryable)</span>}
        </div>
      )}

      {/* Hero stats */}
      <div className={`grid gap-4 ${cost ? "grid-cols-[2fr_1fr_1fr_1fr]" : "grid-cols-[2fr_1fr_1fr]"}`}>
        <div className="rounded-xl border border-neutral-800 bg-neutral-900 px-5 py-4">
          <div className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">
            Match rate: baseline → {stats.final ? "final" : "now"}
          </div>
          <div className="mt-1 flex items-baseline gap-4 text-6xl font-black tabular-nums">
            <span className="text-neutral-400">{pct(stats.baseline)}</span>
            <span className="text-4xl text-neutral-600">→</span>
            <span className={stats.current !== null && stats.current >= 1 ? "text-green-400" : "text-white"}>
              {pct(stats.current)}
            </span>
          </div>
        </div>
        <Stat label="Behaviors tested" value={stats.behaviors === null ? "n/a" : stats.behaviors.toLocaleString("en-US")} />
        <Stat label="Crashes caught" value={String(stats.crashes)} tone={stats.crashes ? "text-amber-400" : "text-white"} />
        {cost && <Stat label="Live cost" value={`$${cost.total_usd.toFixed(cost.total_usd < 1 ? 3 : 2)}`} />}
      </div>

      <Card title="Agents">
        <Lanes agents={job.agents} />
      </Card>

      <Card
        title="Divergences"
        right={
          openCount > 0 ? (
            <span className="animate-pulse text-lg font-bold text-red-400">{openCount} open: click to inspect</span>
          ) : rows.length ? (
            <span className="text-lg font-semibold text-green-400">all {rows.length} resolved</span>
          ) : null
        }
      >
        <Divergences rows={rows} />
      </Card>

      <div className="grid grid-cols-2 gap-5">
        <Card title="Match rate per iteration">
          <MatchChart series={verifySeries(job, events)} />
        </Card>
        <Card title="Crashes">
          <CrashCard crashes={crashViews(job, events)} />
        </Card>
      </div>

      <div className="grid grid-cols-2 gap-5">
        <Card title="Approval">
          <ApprovalPanel job={job} source={source} onJob={setJob} />
        </Card>
        <Card title="Flash vs Argon">
          <BaselineGrid />
        </Card>
      </div>

      <EventLog events={events} />
    </main>
  );
}
