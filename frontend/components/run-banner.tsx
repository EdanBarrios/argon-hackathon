"use client";

import { useEffect, useRef, useState } from "react";
import type { Job } from "@/lib/api";
import type { PipelineStep, WallRound } from "@/lib/derive";

const WORKING = new Set(["queued", "running", "verifying"]);

function clockText(ms: number) {
  const s = Math.max(0, Math.floor(ms / 1000));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

/**
 * Says plainly whether the run is working, done, or failed. While it works, the C
 * and Rust plates slide past each other on a thin track; when it completes during
 * the visit, they snap into register once. Loading an already finished run shows
 * the finished state without the snap.
 */
export function RunBanner({ job, steps, round }: { job: Job; steps: PipelineStep[]; round: WallRound | null }) {
  const working = WORKING.has(job.status);
  const [now, setNow] = useState(() => Date.now());
  const wasWorking = useRef(working);
  const [justFinished, setJustFinished] = useState(false);

  useEffect(() => {
    if (!working) return;
    const t = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(t);
  }, [working]);

  useEffect(() => {
    if (wasWorking.current && !working) setJustFinished(true);
    wasWorking.current = working;
  }, [working]);

  const start = Date.parse(job.created_at);
  const end = working ? now : Date.parse(job.updated_at);
  const elapsed = Number.isFinite(start) && Number.isFinite(end) ? clockText(end - start) : null;
  const current = steps.find((s) => s.active) ?? steps[steps.length - 1];

  if (working) {
    return (
      <div role="status" aria-live="polite" className="flex flex-col gap-3">
        <div className="flex flex-wrap items-baseline justify-between gap-3">
          <p className="text-2xl font-extrabold tracking-[-0.02em]">
            Running<span className="text-dim"> · </span>
            <span className="font-semibold">{current ? current.label : "starting"}</span>
            {current?.detail && <span className="text-lg font-normal text-dim"> · {current.detail}</span>}
          </p>
          <p className="font-mono text-lg tabular-nums text-dim">{elapsed ?? ""} elapsed</p>
        </div>
        <div className="run-track" aria-hidden="true" />
      </div>
    );
  }

  if (job.status === "failed" || job.status === "cancelled") {
    return (
      <div role="status" className="flex flex-col gap-3">
        <p className="text-2xl font-extrabold tracking-[-0.02em] text-div">
          Run {job.status}
          {elapsed && <span className="font-mono text-lg font-normal text-dim"> · after {elapsed}</span>}
        </p>
        <div className="h-[3px] w-full bg-div/60" aria-hidden="true" />
      </div>
    );
  }

  // completed
  const all = round && round.total > 0 && round.match === round.total;
  return (
    <div role="status" className="flex flex-col gap-3">
      <div className="flex flex-wrap items-baseline justify-between gap-3">
        <p className="text-2xl font-extrabold tracking-[-0.02em]">
          <span className="text-match">Run complete</span>
          {round && (
            <span className="font-semibold">
              <span className="text-dim"> · </span>
              {round.match.toLocaleString("en-US")}/{round.total.toLocaleString("en-US")} byte-identical
            </span>
          )}
          {job.approval.required && !job.approval.approved_by && (
            <>
              <span className="text-dim"> · </span>
              <a href="#approve" className="font-semibold underline decoration-dim underline-offset-4 hover:decoration-paper">
                ready for your approval
              </a>
            </>
          )}
        </p>
        {elapsed && <p className="font-mono text-lg tabular-nums text-dim">finished in {elapsed}</p>}
      </div>
      <div className={`run-done ${justFinished ? "run-done-snap" : ""} ${all ? "" : "run-done-partial"}`} aria-hidden="true" />
    </div>
  );
}
