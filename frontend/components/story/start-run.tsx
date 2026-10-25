"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { API_URL, createJob, listJobs, type CreateJobBody, type Job, type StartFrom } from "@/lib/api";
import { pct } from "@/lib/derive";

const START_OPTIONS: { value: StartFrom; label: string; hint: string }[] = [
  { value: "flash-2", label: "From Flash's attempt", hint: "Re-verifies Flash's saved translation, repairs any divergence." },
  { value: "argon-1", label: "From Argon's attempt", hint: "Re-verifies Argon's saved translation (about 1 minute)." },
  { value: "fresh", label: "Fresh translation", hint: "Argon translates cJSON from scratch first (several minutes)." },
];

const MODELS = [
  { value: "gemini-4-argon", label: "Argon (gemini-4-argon)" },
  { value: "gemini-3.8-flash", label: "Flash (gemini-3.8-flash)" },
];

const field =
  "rounded-md border border-line bg-ink px-3 py-2.5 text-lg text-paper outline-none focus:border-dim";

function PreRuns() {
  const [jobs, setJobs] = useState<Job[] | null>(null);
  const [err, setErr] = useState<string | null>(null);
  useEffect(() => {
    listJobs()
      .then((js) => setJobs(js.filter((j) => j.status !== "queued")))
      .catch(() => setErr(`API unreachable at ${API_URL}`));
  }, []);
  if (err) return <p className="text-dim">{err}</p>;
  if (!jobs) return <p className="text-dim">Loading runs...</p>;
  if (!jobs.length) return <p className="text-dim">No runs on this server yet.</p>;
  return (
    <ul className="flex flex-col gap-1.5">
      {jobs.slice(0, 5).map((j) => (
        <li key={j.id} className="flex flex-wrap items-baseline gap-x-3">
          <Link href={`/jobs/${encodeURIComponent(j.id)}`} className="font-mono text-base underline decoration-line underline-offset-4 hover:decoration-paper">
            {j.id}
          </Link>
          <span className="text-dim">
            {j.status}
            {j.result ? `, ${pct(j.result.baseline_match_rate, 1)} → ${pct(j.result.final_match_rate, 1)} on ${j.result.behaviors_tested} inputs` : ""}
          </span>
        </li>
      ))}
    </ul>
  );
}

/** Start a live run against the API, or open an earlier one. The request body is unchanged from the original home page. */
export function StartRun() {
  const router = useRouter();
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);
  const [jobId, setJobId] = useState("");
  const [startFrom, setStartFrom] = useState<StartFrom>("flash-2");
  const [model, setModel] = useState(MODELS[0].value);

  // targeted:str_ adds the 278 string-escape adversarial inputs; they are what expose Flash's real
  // divergence ("\uZ\t\\"), so a run from flash-2 shows a live Argon repair.
  const body: CreateJobBody = {
    library: "cjson", model, start_from: startFrom, thinking: "medium", rounds: 4, inputs: 300,
    extra_inputs: ["targeted:str_"],
  };

  const start = async () => {
    setBusy(true);
    setErr(null);
    try {
      const job = await createJob(body);
      router.push(`/jobs/${encodeURIComponent(job.id)}`);
    } catch (e) {
      setErr(`Could not start a run at ${API_URL} (${(e as Error).message}).`);
      setBusy(false);
    }
  };

  return (
    <div className="grid gap-10 lg:grid-cols-[1.3fr_1fr]">
      <div className="flex flex-col gap-5">
        <div className="flex flex-wrap items-end gap-4">
          <label className="flex flex-col gap-1.5">
            <span className="text-sm text-dim">Start from</span>
            <select value={startFrom} onChange={(e) => setStartFrom(e.target.value as StartFrom)} className={field}>
              {START_OPTIONS.map((o) => (
                <option key={o.value} value={o.value}>
                  {o.label}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-1.5">
            <span className="text-sm text-dim">Repair model</span>
            <select value={model} onChange={(e) => setModel(e.target.value)} className={field}>
              {MODELS.map((m) => (
                <option key={m.value} value={m.value}>
                  {m.label}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            onClick={start}
            disabled={busy}
            className="rounded-md bg-paper px-7 py-3 text-xl font-bold text-ink transition hover:bg-white disabled:bg-line disabled:text-dim"
          >
            {busy ? "Starting..." : "Start a live run"}
          </button>
        </div>
        <p className="text-dim">{START_OPTIONS.find((o) => o.value === startFrom)?.hint}</p>
        <p className="break-all font-mono text-xs text-dim/70">POST {API_URL}/jobs {JSON.stringify(body)}</p>
        {err && <p className="text-lg text-div">{err}</p>}
      </div>

      <div className="flex flex-col gap-4 border-t border-line pt-6 lg:border-t-0 lg:border-l lg:pl-10 lg:pt-0">
        <h3 className="text-lg font-bold">Open an earlier run</h3>
        <PreRuns />
        <form
          className="flex gap-2"
          onSubmit={(e) => {
            e.preventDefault();
            const id = jobId.trim();
            if (id) router.push(`/jobs/${encodeURIComponent(id)}`);
          }}
        >
          <input
            value={jobId}
            onChange={(e) => setJobId(e.target.value)}
            placeholder="job id"
            aria-label="Job id"
            className={`${field} min-w-0 flex-1 font-mono`}
          />
          <button type="submit" className="rounded-md border border-dim px-5 py-2 text-lg font-semibold hover:bg-panel">
            Load
          </button>
        </form>
        <p className="text-sm text-dim">
          Offline screens from the recorded run:{" "}
          {(["running", "completed", "failed"] as const).map((k, i) => (
            <span key={k}>
              {i ? ", " : ""}
              <Link href={`/jobs/sample?fixture=${k}`} className="underline decoration-line underline-offset-4 hover:decoration-paper">
                {k}
              </Link>
            </span>
          ))}
        </p>
      </div>
    </div>
  );
}
