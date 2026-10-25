"use client";

import { useEffect, useState } from "react";
import type { WallState } from "@/lib/derive";

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
        <div className="font-mono text-2xl text-paper/80">
          Verifier running…
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
      <div className="text-base text-dim">
        One mark per input, {round.total.toLocaleString("en-US")} in all
        {round.iteration !== null && <> · verify #{round.iteration}{wall.rounds.length > 1 && ` of ${wall.rounds.length}`}</>}
        {round.div > 0 && <span className="text-div"> · {round.div} apart</span>}
        {full && <span className="text-match"> · all byte-identical</span>}
      </div>
      <Grid states={states} size={size} revealed={revealed} />

      {!onLatest && round.div > 0 && (
        <div className="rise font-mono text-sm text-div">
          {round.div} divergences sent to Argon for repair
        </div>
      )}

      {showHoldout && (
        <div className="rise mt-auto border-t border-line pt-3">
          <div className="mb-2 flex items-baseline justify-between font-mono text-sm">
            <span className="font-sans text-dim">Held-out inputs the model never saw</span>
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
