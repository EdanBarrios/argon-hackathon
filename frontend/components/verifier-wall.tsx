"use client";

import { useEffect, useState } from "react";
import type { WallState } from "@/lib/derive";
import { CountUp } from "./ui";

/** Spread the fill over this long, however many inputs there are. */
const FILL_MS = 1400;
/** How long each earlier round stays on screen before the next one plays. */
const ROUND_MS = 3200;

type CellState = "empty" | "scan" | "match" | "div" | "crash";

// One square per input. Plays the real verify rounds in order (e.g. verify #1 with
// divergences, then verify #2 after the repair), then stays on the latest. Failures
// are grouped at the end; cell order is not meaningful.
export function VerifierWall({ wall, holdout }: { wall: WallState; holdout: { match: number; total: number } | null }) {
  const last = wall.rounds.length - 1;
  const [idx, setIdx] = useState(0);
  const [revealed, setRevealed] = useState(false);

  useEffect(() => {
    const raf = requestAnimationFrame(() => setRevealed(true));
    return () => cancelAnimationFrame(raf);
  }, []);
  // Step through rounds one at a time; new rounds from live polling continue the replay.
  useEffect(() => {
    if (idx >= last) return;
    const t = setTimeout(() => setIdx((i) => Math.min(i + 1, last)), ROUND_MS);
    return () => clearTimeout(t);
  }, [idx, last]);

  const round = wall.rounds[Math.min(idx, Math.max(0, last))];
  const onLatest = idx >= last;
  const scanning = wall.scanning && onLatest;
  const showHoldout = !!holdout && onLatest;

  if (!round) {
    return (
      <div className="flex flex-col gap-4">
        <div className="font-mono text-2xl text-neutral-300">
          Verifier running<span className="cursor text-match">▌</span>
        </div>
        <Grid states={Array.from({ length: 300 }, () => "scan" as CellState)} size={12} revealed />
      </div>
    );
  }

  const states: CellState[] = Array.from({ length: round.total }, (_, i) =>
    scanning ? "scan" : i < round.match ? "match" : i < round.match + round.div ? "div" : "crash",
  );
  const size = round.total > 1500 ? 7 : round.total > 800 ? 9 : 12;
  const full = round.total > 0 && round.match === round.total;

  return (
    <div className="flex h-full flex-col gap-4">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <div className={`font-mono text-6xl font-black tabular-nums ${full ? "text-match glow-match" : "text-white"}`}>
            <CountUp value={round.match} />
            <span className="text-3xl text-dim"> / {round.total.toLocaleString("en-US")}</span>
          </div>
          <div className="mt-1 text-base text-neutral-400">
            inputs where C and Rust print byte-identical output
            {round.iteration !== null && (
              <span className="font-mono text-dim">
                {" "}
                · verify #{round.iteration}
                {wall.rounds.length > 1 && ` of ${wall.rounds.length}`}
              </span>
            )}
          </div>
        </div>
        <div className="flex gap-4 font-mono text-sm">
          <Legend cls="bg-match" label="match" n={round.match} />
          <Legend cls="bg-div" label="divergence" n={round.div} />
          {round.crash > 0 && <Legend cls="bg-crash" label="crash" n={round.crash} />}
        </div>
      </div>

      <Grid states={states} size={size} revealed={revealed} />

      {!onLatest && round.div > 0 && (
        <div className="rise font-mono text-sm text-div">
          {round.div} divergences sent to Argon for repair<span className="cursor">▌</span>
        </div>
      )}

      {showHoldout && (
        <div className="rise mt-auto border-t border-line pt-3">
          <div className="mb-2 flex items-baseline justify-between font-mono text-sm">
            <span className="uppercase tracking-[0.2em] text-dim">Held-out: inputs the model never saw</span>
            <span className={holdout.match === holdout.total ? "text-match" : "text-div"}>
              {holdout.match}/{holdout.total}
            </span>
          </div>
          <Grid
            states={Array.from({ length: holdout.total }, (_, i) => (i < holdout.match ? "match" : "div"))}
            size={size}
            revealed={revealed}
          />
        </div>
      )}
    </div>
  );
}

function Grid({ states, size, revealed }: { states: CellState[]; size: number; revealed: boolean }) {
  const n = states.length || 1;
  return (
    <div
      className="flex flex-wrap gap-[3px]"
      role="img"
      aria-label={`${states.filter((s) => s === "match").length} of ${states.length} inputs match`}
    >
      {states.map((s, i) => (
        <span
          key={i}
          className="cell"
          data-s={revealed ? s : "empty"}
          style={{ width: size, height: size, transitionDelay: `${(i / n) * FILL_MS}ms` }}
        />
      ))}
    </div>
  );
}

function Legend({ cls, label, n }: { cls: string; label: string; n: number }) {
  return (
    <span className="flex items-center gap-1.5 text-neutral-300">
      <span className={`h-2.5 w-2.5 rounded-sm ${cls}`} />
      <CountUp value={n} /> {label}
    </span>
  );
}
