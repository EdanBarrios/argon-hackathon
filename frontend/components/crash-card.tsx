import type { CrashView } from "@/lib/derive";

export function CrashCard({ crashes }: { crashes: CrashView[] }) {
  if (!crashes.length) return <p className="text-lg text-neutral-500">No crashes caught yet.</p>;
  return (
    <div className="flex flex-col gap-4">
      {crashes.map((c, i) => (
        <div key={i} className="rounded-lg border border-amber-500/60 bg-amber-500/10 p-4">
          <div className="mb-3 truncate font-mono text-base text-neutral-300" title={c.input}>
            input: {c.input}
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="rounded-lg bg-black p-3">
              <div className="text-xs font-bold uppercase tracking-widest text-neutral-400">C</div>
              <div className="text-2xl font-bold text-amber-400">AddressSanitizer {c.kind}</div>
            </div>
            <div className="rounded-lg bg-black p-3">
              <div className="text-xs font-bold uppercase tracking-widest text-neutral-400">Rust</div>
              <div className="font-mono text-2xl font-bold text-green-400">{c.rust_behavior}</div>
            </div>
          </div>
          {c.sanitizer_report && (
            <details className="mt-3">
              <summary className="cursor-pointer text-sm font-semibold uppercase tracking-widest text-neutral-400 hover:text-white">
                Sanitizer report
              </summary>
              <pre className="mt-2 max-h-64 overflow-auto rounded-lg bg-black p-3 font-mono text-sm leading-relaxed text-neutral-200">
                {c.sanitizer_report}
              </pre>
            </details>
          )}
        </div>
      ))}
    </div>
  );
}
