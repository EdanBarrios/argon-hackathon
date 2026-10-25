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
          className={`min-w-0 flex-1 border-t-[3px] pt-2 text-sm ${i < steps.length - 1 ? "hidden md:block" : ""} ${s.active ? "border-paper" : RULE[s.tone]} ${
            s.tone === "neutral" && !s.active ? "text-dim" : "text-paper"
          }`}
        >
          <div className="flex items-center gap-2 font-bold">
            {s.active && <span className="live-dot inline-block h-2 w-2 rounded-full bg-paper" />}
            <span className="truncate">
              <span className="md:hidden">{i + 1} of {steps.length} · </span>
              <span className="hidden md:inline">{i + 1} · </span>
              {s.label}
            </span>
          </div>
          <div className="font-mono text-xs leading-snug text-dim">{s.detail}</div>
        </li>
      ))}
    </ol>
  );
}
