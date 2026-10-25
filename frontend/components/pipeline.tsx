import { Fragment } from "react";
import type { PipelineStep, StepTone } from "@/lib/derive";

const TONE: Record<StepTone, { box: string; text: string; dot: string }> = {
  neutral: { box: "border-line", text: "text-white", dot: "bg-neutral-400" },
  match: { box: "border-match/50 bg-match/[0.06]", text: "text-match glow-match", dot: "bg-match" },
  div: { box: "border-div/60 bg-div/[0.07]", text: "text-div glow-div", dot: "bg-div" },
  crash: { box: "border-crash/60 bg-crash/[0.07]", text: "text-crash glow-crash", dot: "bg-crash" },
};

// One card per real pipeline event, connected left to right. New steps rise in;
// the connector into a new step fills; the active step has a sweep.
export function Pipeline({ steps }: { steps: PipelineStep[] }) {
  if (!steps.length) return null;
  return (
    <div className="flex items-stretch">
      {steps.map((s, i) => {
        const t = TONE[s.tone];
        return (
          <Fragment key={s.key}>
            {i > 0 && (
              <div className="flex w-4 shrink-0 items-center">
                <div className="connector h-0.5 w-full" data-on="true" />
              </div>
            )}
            <div
              className={`rise min-w-0 flex-1 rounded-md border px-3 py-2.5 ${t.box} ${s.active ? "step-active border-match/70" : ""}`}
            >
              <div className="flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.2em] text-dim">
                <span className={`h-1.5 w-1.5 rounded-full ${s.active ? "live-dot bg-match" : t.dot}`} />
                step {i + 1}
              </div>
              <div className={`mt-1 text-base font-bold leading-tight ${s.active ? "text-white" : t.text}`}>{s.label}</div>
              <div className="mt-0.5 font-mono text-xs leading-snug text-neutral-400">
                {s.detail}
                {s.active && <span className="cursor">▌</span>}
              </div>
            </div>
          </Fragment>
        );
      })}
    </div>
  );
}
