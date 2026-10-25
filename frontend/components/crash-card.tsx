import type { CrashView } from "@/lib/derive";

export function CrashCard({ crashes }: { crashes: CrashView[] }) {
  if (!crashes.length) return <p className="text-lg text-dim">No crashes caught yet.</p>;
  return (
    <div className="flex flex-col gap-4">
      {crashes.map((c, i) => (
        <div key={i} className="rise flex flex-col gap-3">
          <div className="flex flex-wrap items-baseline justify-between gap-2">
            {c.cve && <span className="font-mono text-2xl font-black text-crash">{c.cve}</span>}
            <span className="truncate font-mono text-base text-paper/80" title={c.input}>
              input <span className="rounded bg-ink px-2 py-0.5 text-paper">{c.input}</span>
            </span>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div className="rounded-md border border-crash/50 bg-crash/[0.07] p-3">
              <div className="text-sm font-bold text-clang">C{c.c_version ? ` · ${c.c_version}` : ""}</div>
              <div className="mt-1 text-xl font-bold leading-tight text-crash">AddressSanitizer</div>
              <div className="font-mono text-lg text-crash">{c.kind}</div>
            </div>
            <div className="rounded-md border border-match/40 bg-match/[0.05] p-3">
              <div className="text-sm font-bold text-rust">Rust · Theseus</div>
              <div className="mt-1 text-xl font-bold leading-tight text-match">No memory error</div>
              <div className="font-mono text-lg text-match">{c.rust_behavior}</div>
            </div>
          </div>
          {c.sanitizer_report && (
            <details>
              <summary className="cursor-pointer text-sm text-dim hover:text-paper">
                Sanitizer report
              </summary>
              <pre className="mt-2 max-h-56 overflow-auto rounded-md bg-ink p-3 font-mono text-xs leading-relaxed text-paper/80">
                {c.sanitizer_report}
              </pre>
            </details>
          )}
        </div>
      ))}
    </div>
  );
}
