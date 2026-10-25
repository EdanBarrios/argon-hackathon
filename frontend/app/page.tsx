"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { API_URL, createJob } from "@/lib/api";

const START_BODY = { library: "cjson", modules: ["parser", "printer"] };

export default function Home() {
  const router = useRouter();
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);
  const [jobId, setJobId] = useState("");

  const start = async () => {
    setBusy(true);
    setErr(null);
    try {
      const job = await createJob(START_BODY);
      router.push(`/jobs/${encodeURIComponent(job.id)}`);
    } catch (e) {
      setErr(`Could not start a run at ${API_URL} (${(e as Error).message}). Use sample data below.`);
      setBusy(false);
    }
  };

  return (
    <main className="mx-auto flex max-w-3xl flex-col gap-10 px-6 py-16">
      <div>
        <h1 className="text-6xl font-black tracking-[0.3em]">THESEUS</h1>
        <p className="mt-3 text-2xl text-neutral-400">C to memory-safe Rust, with evidence that behavior is preserved.</p>
      </div>

      <section className="flex flex-col gap-3">
        <button
          type="button"
          onClick={start}
          disabled={busy}
          className="self-start rounded-lg bg-white px-8 py-4 text-2xl font-bold text-black hover:bg-neutral-200 disabled:bg-neutral-700 disabled:text-neutral-400"
        >
          {busy ? "Starting..." : "Start run"}
        </button>
        <p className="font-mono text-sm text-neutral-500">
          POST {API_URL}/jobs {JSON.stringify(START_BODY)}
        </p>
        {err && <p className="text-lg text-red-400">{err}</p>}
      </section>

      <form
        className="flex flex-col gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          const id = jobId.trim();
          if (id) router.push(`/jobs/${encodeURIComponent(id)}`);
        }}
      >
        <label htmlFor="job-id" className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">
          Load job by id
        </label>
        <div className="flex gap-3">
          <input
            id="job-id"
            value={jobId}
            onChange={(e) => setJobId(e.target.value)}
            placeholder="job_7f3a2c"
            className="flex-1 rounded-lg border border-neutral-700 bg-neutral-900 px-4 py-3 font-mono text-xl text-white outline-none focus:border-neutral-300"
          />
          <button type="submit" className="rounded-lg border border-neutral-500 px-6 py-3 text-xl font-semibold hover:bg-neutral-800">
            Load
          </button>
        </div>
      </form>

      <section className="flex flex-col gap-2">
        <h2 className="text-sm font-bold uppercase tracking-[0.2em] text-neutral-400">Sample data (no backend needed)</h2>
        <div className="flex flex-wrap gap-3 text-xl">
          {(["running", "completed", "failed"] as const).map((k) => (
            <Link key={k} href={`/jobs/sample?fixture=${k}`} className="rounded-lg border border-neutral-700 px-5 py-2 hover:bg-neutral-800">
              {k}
            </Link>
          ))}
        </div>
      </section>
    </main>
  );
}
