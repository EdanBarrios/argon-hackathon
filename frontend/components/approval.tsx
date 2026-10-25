"use client";

import { useState } from "react";
import { approve, type Job, type Source } from "@/lib/api";

const APPROVER = process.env.NEXT_PUBLIC_APPROVER || "maintainer";

export function ApprovalPanel({ job, source, onJob }: { job: Job; source: Source; onJob: (j: Job) => void }) {
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<string | null>(null);
  const { approval } = job;
  const enabled = job.status === "completed" && approval.required && !approval.approved_by && !busy;

  const onClick = async () => {
    if (source === "fixture") {
      setMsg("Sample data: no backend, so no PR is opened. Load a live job to approve.");
      return;
    }
    setBusy(true);
    setMsg(null);
    try {
      onJob(await approve(job.id, APPROVER));
    } catch (e) {
      setMsg(`Approve failed: ${(e as Error).message}`);
    } finally {
      setBusy(false);
    }
  };

  let hint = "";
  if (job.status !== "completed") hint = "Available once the job completes.";
  else if (!approval.required) hint = "This job does not require approval.";

  return (
    <div className="flex flex-col gap-4">
      {approval.pr_url ? (
        <div className="flex flex-col gap-1">
          <div className="text-lg text-green-400">✓ Approved by {approval.approved_by}. PR opened with the evidence report:</div>
          <a href={approval.pr_url} target="_blank" rel="noreferrer" className="break-all font-mono text-xl text-white underline">
            {approval.pr_url}
          </a>
        </div>
      ) : approval.approved_by && approval.pr_status === "dry_run" ? (
        <div className="flex flex-col gap-1">
          <div className="text-lg text-amber-400">Approved by {approval.approved_by}. Dry run: no PR opened.</div>
          <p className="break-all text-neutral-300">{approval.pr_detail}</p>
        </div>
      ) : approval.approved_by ? (
        <div className="text-lg text-green-400">✓ Approved by {approval.approved_by}. Opening PR...</div>
      ) : (
        <button
          type="button"
          onClick={onClick}
          disabled={!enabled}
          className="w-full rounded-md border border-match bg-match/15 px-6 py-4 font-mono text-xl font-bold uppercase tracking-widest text-match shadow-[0_0_24px_rgb(61_220_132/0.25)] transition hover:bg-match hover:text-black disabled:cursor-not-allowed disabled:border-line disabled:bg-transparent disabled:text-neutral-500 disabled:shadow-none"
        >
          {busy ? "Approving..." : "Approve and open PR"}
        </button>
      )}
      {hint && !approval.approved_by && <p className="text-neutral-500">{hint}</p>}
      {msg && <p className="text-neutral-300">{msg}</p>}
    </div>
  );
}
