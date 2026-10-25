"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { API_URL, createJob, listJobs, type CreateJobBody, type Job, type StartFrom } from "@/lib/api";
import { pct } from "@/lib/derive";
import { TryPanel } from "@/components/try-panel";

const START_OPTIONS: { value: StartFrom; label: string; hint: string }[] = [
  { value: "flash-2", label: "From Flash's attempt", hint: "Re-verifies Flash's saved translation, repairs any divergence." },
  { value: "argon-1", label: "From Argon's attempt", hint: "Re-verifies Argon's saved translation (about 1 minute)." },
  { value: "fresh", label: "Fresh translation", hint: "Argon translates cJSON from scratch first (several minutes)." },
];

const MODELS = [
  { value: "gemini-4-argon", label: "Argon (gemini-4-argon)" },
  { value: "gemini-3.8-flash", label: "Flash (gemini-3.8-flash)" },
];

function PreRuns() {
  const [jobs, setJobs] = useState<Job[] | null>(null);
  const [err, setErr] = useState<string | null>(null);
  useEffect(() => {
    listJobs()
      .then((js) => setJobs(js.filter((j) => j.status !== "queued")))
      .catch(() => setErr(`API unreachable at ${API_URL}`));
  }, []);
  if (err) return <p className="text-neutral-500">{err}</p>;
  if (!jobs) return <p className="text-neutral-500">Loading runs...</p>;
  if (!jobs.length) return <p className="text-neutral-500">No runs on this server yet.</p>;
  return (
    <ul className="flex flex-col gap-1">
      {jobs.slice(0, 6).map((j) => (
        <li key={j.id}>
          <Link href={`/jobs/${encodeURIComponent(j.id)}`} className="font-mono text-lg underline-offset-4 hover:underline">
            {j.id}
          </Link>
          <span className="ml-3 text-neutral-400">
            {j.status}
            {j.result ? `, ${pct(j.result.baseline_match_rate, 1)} → ${pct(j.result.final_match_rate, 1)} on ${j.result.behaviors_tested} inputs` : ""}
          </span>
        </li>
      ))}
    </ul>
  );
}

export default function Home() {
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
    <main className="mx-auto flex max-w-5xl flex-col gap-10 px-4 py-12 sm:px-6">
      <div>
        <h1 className="text-5xl font-black tracking-[0.3em]">THESEUS</h1>
        <p className="mt-4 text-3xl font-semibold leading-snug text-neutral-100">
          Theseus rewrites C into memory-safe Rust and proves it behaves the same.
        </p>
      </div>

      <section className="flex flex-col gap-3 rounded-xl border border-neutral-800 bg-neutral-900 p-5">
        <h2 className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">Try any input</h2>
        <TryPanel />
      </section>

      <section className="flex flex-col gap-4 rounded-xl border border-neutral-800 bg-neutral-900 p-5">
        <h2 className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">Start live run</h2>
        <div className="flex flex-wrap items-end gap-4">
          <label className="flex flex-col gap-1 text-neutral-300">
            <span className="text-xs uppercase tracking-widest text-neutral-500">Start from</span>
            <select
              value={startFrom}
              onChange={(e) => setStartFrom(e.target.value as StartFrom)}
              className="rounded-lg border border-neutral-700 bg-black px-3 py-2 text-lg"
            >
              {START_OPTIONS.map((o) => (
                <option key={o.value} value={o.value}>
                  {o.label}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-1 text-neutral-300">
            <span className="text-xs uppercase tracking-widest text-neutral-500">Repair model</span>
            <select value={model} onChange={(e) => setModel(e.target.value)} className="rounded-lg border border-neutral-700 bg-black px-3 py-2 text-lg">
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
            className="rounded-lg bg-white px-8 py-3 text-2xl font-bold text-black hover:bg-neutral-200 disabled:bg-neutral-700 disabled:text-neutral-400"
          >
            {busy ? "Starting..." : "Start live run"}
          </button>
        </div>
        <p className="text-neutral-400">{START_OPTIONS.find((o) => o.value === startFrom)?.hint}</p>
        <p className="break-all font-mono text-xs text-neutral-500">POST {API_URL}/jobs {JSON.stringify(body)}</p>
        {err && <p className="text-lg text-red-400">{err}</p>}
      </section>

      <section className="flex flex-col gap-3">
        <h2 className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">Load pre-run</h2>
        <PreRuns />
        <form
          className="flex gap-3"
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
            className="min-w-0 flex-1 rounded-lg border border-neutral-700 bg-neutral-900 px-4 py-2 font-mono text-lg text-white outline-none focus:border-neutral-300"
          />
          <button type="submit" className="rounded-lg border border-neutral-500 px-5 py-2 text-lg font-semibold hover:bg-neutral-800">
            Load
          </button>
        </form>
        <p className="text-sm text-neutral-500">
          Offline sample screens (labeled as sample data):{" "}
          {(["running", "completed", "failed"] as const).map((k, i) => (
            <span key={k}>
              {i ? ", " : ""}
              <Link href={`/jobs/sample?fixture=${k}`} className="underline">
                {k}
              </Link>
            </span>
          ))}
        </p>
      </section>

      <footer className="border-t border-neutral-800 pt-4 text-sm text-neutral-500">
        Built today: verifier, pipeline, UI. Used as inputs: cJSON and libexpat (third-party C libraries).
      </footer>
    </main>
  );
}
