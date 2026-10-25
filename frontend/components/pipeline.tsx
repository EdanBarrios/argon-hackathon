import type { PipelineStep, StepTone } from "@/lib/derive";

const RULE: Record<StepTone, string> = {
  neutral: "border-line",
  match: "border-match",
  div: "border-div",
  crash: "border-crash",
};

// One rule per real pipeline event, left to right; the rule's colour is the step's verdict.
export function Pipeline({ steps }: { steps: PipelineStep[] }) {
  if (!steps.length) return null;
  return (
    <ol className="flex gap-2">
      {steps.map((s, i) => (
        <li
          key={s.key}
          className={`rise min-w-0 flex-1 border-t-[3px] pt-2 text-sm ${s.active ? "border-paper" : RULE[s.tone]} ${
            s.tone === "neutral" && !s.active ? "text-dim" : "text-paper"
          }`}
        >
          <div className="flex items-center gap-2 font-bold">
            {s.active && <span className="live-dot inline-block h-2 w-2 rounded-full bg-paper" />}
            <span className="truncate">
              {i + 1} · {s.label}
            </span>
          </div>
          <div className="truncate font-mono text-xs text-dim">{s.detail}</div>
        </li>
      ))}
    </ol>
  );
}
