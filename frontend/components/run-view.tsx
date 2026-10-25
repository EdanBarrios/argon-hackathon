"use client";

import Link from "next/link";
import { usePollJob, type FixtureKind, type Snapshot } from "@/lib/api";
import { crashViews, divergenceRows, holdoutResult, latestSummary, pipelineSteps, wallState } from "@/lib/derive";
import { BaselineCompare } from "./baseline";
import { EventLog } from "./event-log";
import { Evidence } from "./evidence";
import { Lanes } from "./lanes";
import { Pipeline } from "./pipeline";
import { CrashRow, DivergenceRows, HoldoutRow, PlateHeads, RepairRow, TotalsRow } from "./plates";
import { Ticker } from "./ticker";
import { TryPanel } from "./try-panel";
import { SampleBadge, StatusPill } from "./ui";
import { VerifierWall } from "./verifier-wall";

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
        <p className="mt-6 text-xl text-dim">Loading job {id}…</p>
      </main>
    );
  }

  const rows = divergenceRows(job, events);
  const crashes = crashViews(job, events);
  const wall = wallState(job, events);
  const holdout = holdoutResult(events);
  const steps = pipelineSteps(job, events);
  const roundLast = wall?.rounds[wall.rounds.length - 1] ?? null;
  const roundFirst = wall?.rounds[0] ?? null;
  const repair = rows.find((r) => r.repair)?.repair ?? null;
  const openCount = rows.filter((r) => !r.resolved).length;
  const working = job.status === "running" || job.status === "verifying" || job.status === "queued";

  return (
    <main className="mx-auto flex max-w-[1440px] flex-col gap-5 px-6 py-4 md:px-12">
      <header className="flex flex-wrap items-center gap-x-4 gap-y-2 border-b border-line pb-4">
        <Wordmark />
        <span className="text-dim">/</span>
        <span className="text-lg font-semibold">{job.library}</span>
        <StatusPill status={job.status} size="lg" />
        <span className="font-mono text-sm text-dim">{job.id}</span>
        <span className="text-sm text-dim">runner {job.runner}</span>
        {cost && <span className="text-sm text-dim">${cost.total_usd.toFixed(cost.total_usd < 1 ? 3 : 2)} in model calls so far</span>}
        <span className="ml-auto flex items-center gap-4">
          {source === "fixture" ? <SampleBadge notice={notice} /> : notice && <span className="text-sm text-dim">{notice}</span>}
          <a href="#approve" className={`rounded-md px-4 py-2 text-base font-bold transition ${job.status === "completed" ? "bg-paper text-ink hover:bg-white" : "border border-line text-dim"}`}>
            {job.approval.pr_url ? "PR opened" : job.approval.approved_by ? "Approved" : "Approve and open PR"}
          </a>
        </span>
      </header>

      {job.error && (
        <div className="rounded-md border border-div/70 p-4 text-lg">
          <span className="font-mono font-bold text-div">{job.error.code}</span>
          <span className="text-paper">: {job.error.message}</span>
          {job.error.retryable && <span className="ml-2 text-dim">(retryable)</span>}
        </div>
      )}

      <Pipeline steps={steps} />
      <Ticker text={latestSummary(events)} />

      {job.agents.length > 1 && <Lanes agents={job.agents} />}

      {/* The evidence, printed twice. */}
      <section aria-label="Evidence, C beside Rust">
        <PlateHeads />
        <TotalsRow round={roundLast} before={roundFirst} scanning={wall?.scanning ?? false} />
        {rows.length > 0 && (
          <div className="flex items-baseline justify-between px-1 pb-1 pt-6 text-sm">
            <span className="font-bold">
              {rows.length} {rows.length === 1 ? "input" : "inputs"} where the outputs differed
            </span>
            <span className={openCount ? "font-bold text-div" : "text-match"}>{openCount ? `${openCount} open · click a row` : "all registered after the repair"}</span>
          </div>
        )}
        <DivergenceRows rows={rows} confirmed={!!roundLast && roundLast.div === 0 && !(wall?.scanning ?? false)} />
        {repair && <RepairRow repair={repair} />}
        {holdout && <HoldoutRow match={holdout.match} total={holdout.total} />}
        {crashes.map((c, i) => (
          <CrashRow key={i} crash={c} />
        ))}
        <div className="border-t border-line" />
      </section>

      {(working || (wall && wall.rounds.length > 0)) && (
        <section className="rounded-md border border-line p-5">
          <h2 className="mb-4 text-base font-bold">Every input, one mark each</h2>
          {wall ? <VerifierWall wall={wall} holdout={holdout} /> : <p className="text-lg text-dim">Waiting for the first verify round.</p>}
        </section>
      )}

      <section id="approve" className="grid gap-5 lg:grid-cols-2">
        <div className="rounded-md border border-line p-5">
          <h2 className="mb-4 text-base font-bold">Evidence report</h2>
          <Evidence job={job} source={source} onJob={setJob} wall={wall} holdout={holdout} rows={rows} crashes={crashes} />
        </div>
        <div className="rounded-md border border-line p-5">
          <h2 className="mb-4 text-base font-bold">Flash vs Argon</h2>
          <BaselineCompare />
        </div>
      </section>

      <section className="rounded-md border border-line p-5">
        <h2 className="mb-4 text-base font-bold">Try any input</h2>
        <TryPanel compact />
      </section>

      <EventLog events={events} />
    </main>
  );
}
