import baseline from "@/data/baseline.json";

// Flash vs Argon comes from data/baseline.json, copied by hand from
// results/cjson-flash-vs-argon. Wording follows that README: Flash hit a token
// cap, it did not fail at translating. Laid out on the run view's spine:
// Argon on the left plate, Flash on the right, the metric between them.

interface Run {
  id: string;
  high_thinking: "finished" | "token_cap";
  retried_at?: string;
  match: number | null;
  of: number | null;
  seconds: number | null;
  tokens: number | null;
  thought_tokens: number | null;
}

interface ModelResult {
  name: string;
  model: string;
  runs: Run[];
  adversarial: { run: string; targeted: Score; argon_written: Score };
  footnote?: string;
  divergence_note?: string;
}

interface Score {
  match: number;
  of: number;
}

const n = (x: number) => x.toLocaleString("en-US");
const k = (x: number) => `${Math.round(x / 1000)}K`;

/** "333–355 s" style range over the runs that have a value. */
function range(vals: (number | null)[], fmt: (x: number) => string, unit = "") {
  const xs = vals.filter((v): v is number => v !== null).sort((a, b) => a - b);
  if (!xs.length) return "n/a";
  const lo = fmt(xs[0]);
  const hi = fmt(xs[xs.length - 1]);
  return (lo === hi ? lo : `${lo}–${hi}`) + unit;
}

type Tone = "plain" | "good" | "bad";
interface Cell {
  text: string;
  tone: Tone;
  note?: string;
}

const working = (m: ModelResult) => m.runs.filter((r) => r.match !== null);

const METRICS: { label: string; cell: (m: ModelResult) => Cell }[] = [
  {
    label: "Finished first try, high thinking",
    cell: (m) => {
      const done = m.runs.filter((r) => r.high_thinking === "finished").length;
      return { text: `${done} of ${m.runs.length}`, tone: done === m.runs.length ? "good" : "plain", note: done < m.runs.length ? "hit 64K output cap" : undefined };
    },
  },
  {
    label: "Generated inputs",
    cell: (m) => {
      const w = working(m);
      const all = w.length > 0 && w.every((r) => r.match === r.of);
      return { text: [...new Set(w.map((r) => `${n(r.match!)} / ${n(r.of!)}`))].join(", ") || "n/a", tone: all ? "good" : "plain" };
    },
  },
  {
    label: "Targeted edge inputs",
    cell: (m) => {
      const s = m.adversarial.targeted;
      return { text: `${n(s.match)} / ${n(s.of)}`, tone: s.match === s.of ? "good" : "bad", note: s.match === s.of ? undefined : `${s.of - s.match} divergences` };
    },
  },
  {
    label: "Inputs Argon wrote to break it",
    cell: (m) => {
      const s = m.adversarial.argon_written;
      return { text: `${n(s.match)} / ${n(s.of)}`, tone: s.match === s.of ? "good" : "bad" };
    },
  },
  { label: "Time to a working port", cell: (m) => ({ text: range(working(m).map((r) => r.seconds), (x) => String(Math.round(x)), " s"), tone: "plain" }) },
  { label: "Total tokens", cell: (m) => ({ text: range(working(m).map((r) => r.tokens), k), tone: "plain" }) },
];

const TONE: Record<Tone, string> = { plain: "text-paper", good: "text-match", bad: "text-div" };
const grid = "grid grid-cols-[minmax(0,1fr)_7rem_minmax(0,1fr)] md:grid-cols-[minmax(0,1fr)_16rem_minmax(0,1fr)] items-baseline gap-4";

function Value({ c, align }: { c: Cell; align: "left" | "right" }) {
  return (
    <div className={align === "right" ? "text-right" : ""}>
      <div className={`font-mono text-xl tabular-nums ${TONE[c.tone]}`}>{c.text}</div>
      {c.note && <div className={`text-sm ${c.tone === "bad" ? "text-div" : "text-crash"}`}>{c.note}</div>}
    </div>
  );
}

export function BaselineCompare() {
  const models = baseline.models as ModelResult[];
  const [left, right] = models;
  if (!left || !right) return null;
  return (
    <div>
      <p className="mb-4 max-w-[70ch] text-dim">
        {baseline.library}. {baseline.note}
      </p>
      <div className={`${grid} pb-2`}>
        <div>
          <div className="text-2xl font-extrabold">{left.name}</div>
          <div className="font-mono text-sm text-dim">{left.model}</div>
        </div>
        <div />
        <div className="text-right">
          <div className="text-2xl font-extrabold">{right.name}</div>
          <div className="font-mono text-sm text-dim">{right.model}</div>
        </div>
      </div>
      {METRICS.map((m) => (
        <div key={m.label} className={`${grid} border-t border-line py-3`}>
          <Value c={m.cell(left)} align="left" />
          <div className="text-center text-sm text-dim">{m.label}</div>
          <Value c={m.cell(right)} align="right" />
        </div>
      ))}
      <div className="grid gap-2 border-t border-line pt-3 text-sm md:grid-cols-2 md:gap-8">
        <p className="text-dim">{left.footnote ?? left.divergence_note ?? ""}</p>
        <div className="flex flex-col gap-2 md:text-right">
          {right.divergence_note && <p className="text-div/90">{right.divergence_note}</p>}
          {right.footnote && <p className="text-dim">{right.footnote}</p>}
        </div>
      </div>
    </div>
  );
}
