import baseline from "@/data/baseline.json";

// Flash vs Argon comes from data/baseline.json, copied by hand from
// results/cjson-flash-vs-argon. Wording follows that README: Flash hit a token
// cap, it did not fail at translating.

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

const score = (s: Score) => `${n(s.match)}/${n(s.of)}`;

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

function Row({ label, value, good, bad }: { label: string; value: string; good?: boolean; bad?: boolean }) {
  const tone = bad ? "text-div" : good ? "text-match" : "text-paper";
  return (
    <div className="flex items-baseline justify-between gap-3 border-t border-line py-2">
      <span className="text-base text-dim">{label}</span>
      <span className={`text-right text-xl font-bold tabular-nums ${tone}`}>{value}</span>
    </div>
  );
}

function ScoreRow({ label, s }: { label: string; s: Score }) {
  const full = s.match === s.of;
  const misses = s.of - s.match;
  return <Row label={label} value={full ? score(s) : `${score(s)} (${misses} divergences)`} good={full} bad={!full} />;
}

function Column({ m }: { m: ModelResult }) {
  const finishedHigh = m.runs.filter((r) => r.high_thinking === "finished").length;
  const working = m.runs.filter((r) => r.match !== null);
  const verified = working.map((r) => `${n(r.match!)}/${n(r.of!)}`);
  return (
    <div className="flex flex-col rounded-lg border border-line p-4">
      <div className="flex items-baseline justify-between">
        <span className="text-2xl font-black">{m.name}</span>
        <span className="font-mono text-sm text-dim">{m.model}</span>
      </div>
      <Row
        label="Finished first try, high thinking"
        value={`${finishedHigh} of ${m.runs.length}`}
        good={finishedHigh === m.runs.length}
      />
      {finishedHigh < m.runs.length && (
        <div className="text-right text-sm text-crash">hit 64K output cap</div>
      )}
      <Row
        label="2,000-input verifier"
        value={[...new Set(verified)].join(", ") || "n/a"}
        good={working.length > 0 && working.every((r) => r.match === r.of)}
      />
      <Row label="Time to working port" value={range(working.map((r) => r.seconds), (x) => String(Math.round(x)), " s")} />
      <Row label="Total tokens" value={range(working.map((r) => r.tokens), k)} />
      <Row label="Thinking tokens" value={range(working.map((r) => r.thought_tokens), k)} />
      <ScoreRow label="Targeted edge inputs" s={m.adversarial.targeted} />
      <ScoreRow label="Argon-written inputs" s={m.adversarial.argon_written} />
      {m.divergence_note && <p className="mt-2 text-sm leading-snug text-div/90">{m.divergence_note}</p>}
      {m.footnote && <p className="mt-2 text-sm leading-snug text-dim">{m.footnote}</p>}
    </div>
  );
}

export function BaselineCompare() {
  const models = baseline.models as ModelResult[];
  return (
    <div>
      <p className="mb-3 text-dim">
        {baseline.library}. {baseline.note}
      </p>
      <div className="grid grid-cols-2 gap-3">
        {models.map((m) => (
          <Column key={m.name} m={m} />
        ))}
      </div>
    </div>
  );
}
