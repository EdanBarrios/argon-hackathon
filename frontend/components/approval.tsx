"use client";

import { useState } from "react";
import { approve, type Job, type Source } from "@/lib/api";

const APPROVER = process.env.NEXT_PUBLIC_APPROVER || "maintainer";
// The mock pipeline (THESEUS_MODE=mock) answers approve with this made-up URL; no PR exists behind it.
const PLACEHOLDER = /^https:\/\/github\.com\/theseus-demo\//;
export const isPlaceholderPr = (url: string | null | undefined) => !!url && PLACEHOLDER.test(url);

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
      {approval.pr_url && PLACEHOLDER.test(approval.pr_url) ? (
        <div className="flex flex-col gap-1">
          <div className="text-lg text-paper">Approved by {approval.approved_by}.</div>
          <p className="text-dim">Practice mode: the backend replays a recorded run, so no pull request was opened.</p>
        </div>
      ) : approval.pr_url ? (
        <div className="flex flex-col gap-1">
          <div className="text-lg text-match">Approved by {approval.approved_by}. PR opened with the evidence report:</div>
          <a href={approval.pr_url} target="_blank" rel="noreferrer" className="break-all font-mono text-xl text-paper underline">
            {approval.pr_url}
          </a>
        </div>
      ) : approval.approved_by && approval.pr_status === "dry_run" ? (
        <div className="flex flex-col gap-1">
          <div className="text-lg text-crash">Approved by {approval.approved_by}. Dry run: no PR opened.</div>
          <p className="break-all text-paper/80">{approval.pr_detail}</p>
        </div>
      ) : approval.approved_by ? (
        <div className="text-lg text-match">Approved by {approval.approved_by}. Opening PR…</div>
      ) : (
        <button
          type="button"
          onClick={onClick}
          disabled={!enabled}
          className="rounded-md bg-paper px-7 py-3.5 text-lg font-bold text-ink transition hover:bg-white disabled:cursor-not-allowed disabled:bg-line disabled:text-dim"
        >
          {busy ? "Approving..." : "Approve and open PR"}
        </button>
      )}
      {hint && !approval.approved_by && <p className="text-dim">{hint}</p>}
      {msg && <p className="text-paper/80">{msg}</p>}
    </div>
  );
}
