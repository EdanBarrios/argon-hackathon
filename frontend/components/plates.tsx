"use client";

import { useState, type ReactNode } from "react";
import type { CrashView, DivergenceRow, WallRound } from "@/lib/derive";

// Every piece of evidence is printed twice: C on the left plate, Rust on the
// right, meeting at a registration spine. Agreement registers into one
// paper-white mark; a divergence keeps its two plates apart, in red.

const n = (x: number) => x.toLocaleString("en-US");
const show = (s: string) => s.replace(/\t/g, "  ");

export function Mark({ state, size = 18 }: { state: "registered" | "apart" | "crash" | "pending"; size?: number }) {
  if (state === "apart") {
    const off = size * 0.38;
    return (
      <span className="relative inline-block" style={{ width: size, height: size }} aria-label="plates apart">
        <span className="absolute top-0 rounded-full bg-clang mix-blend-screen" style={{ width: size, height: size, left: -off }} />
        <span className="absolute top-0 rounded-full bg-div mix-blend-screen" style={{ width: size, height: size, left: off }} />
      </span>
    );
  }
  const cls = state === "registered" ? "bg-paper" : state === "crash" ? "bg-crash" : "live-dot bg-dim";
  return <span className={`inline-block rounded-full ${cls}`} style={{ width: size, height: size }} aria-label={state} />;
}

export function PlateHeads() {
  return (
    <div className="grid grid-cols-[minmax(0,1fr)_5.5rem_minmax(0,1fr)] md:grid-cols-[minmax(0,1fr)_11rem_minmax(0,1fr)] items-baseline gap-4 px-1 pb-2 pt-5 text-sm font-bold">
      <span className="text-clang">C · cJSON, the reference</span>
      <span className="text-center font-normal text-dim">registration</span>
      <span className="text-right text-rust">Rust · the translation</span>
    </div>
  );
}

function Row({
  left,
  spine,
  right,
  onClick,
  selected,
  children,
}: {
  left: ReactNode;
  spine: ReactNode;
  right: ReactNode;
  onClick?: () => void;
  selected?: boolean;
  children?: ReactNode;
}) {
  const Tag = onClick ? "button" : "div";
  return (
    <div className={`border-t border-line ${selected ? "bg-panel" : ""}`}>
      <Tag
        type={onClick ? "button" : undefined}
        onClick={onClick}
        className={`grid w-full grid-cols-[minmax(0,1fr)_5.5rem_minmax(0,1fr)] md:grid-cols-[minmax(0,1fr)_11rem_minmax(0,1fr)] items-center gap-4 px-1 py-4 text-left ${onClick ? "cursor-pointer transition hover:bg-panel" : ""}`}
      >
        <div className="min-w-0">{left}</div>
        <div className="flex flex-col items-center gap-1.5">{spine}</div>
        <div className="min-w-0 text-right">{right}</div>
      </Tag>
      {children}
    </div>
  );
}

export function TotalsRow({ round, before, scanning }: { round: WallRound | null; before: WallRound | null; scanning: boolean }) {
  if (!round) {
    return (
      <Row
        left={<div className="text-2xl text-dim">{scanning ? "Running every input through the C…" : "Waiting for the first verify round."}</div>}
        spine={<Mark state="pending" />}
        right={<div className="text-2xl text-dim">{scanning ? "…and through the Rust." : ""}</div>}
      />
    );
  }
  const full = round.match === round.total;
  return (
    <Row
      left={
        <>
          <div className="text-[clamp(40px,6vw,88px)] font-extrabold leading-none tracking-[-0.035em] tabular-nums">{n(round.total)}</div>
          <div className="mt-1 text-dim">inputs run · verify #{round.iteration ?? "?"}</div>
        </>
      }
      spine={
        <>
          <Mark state={scanning ? "pending" : full ? "registered" : "apart"} />
          {!full && !scanning && <span className="text-sm font-bold text-div">{n(round.div + round.crash)} apart</span>}
        </>
      }
      right={
        <>
          <div className={`text-[clamp(40px,6vw,88px)] font-extrabold leading-none tracking-[-0.035em] tabular-nums ${full ? "text-match" : ""}`}>{n(round.match)}</div>
          <div className="mt-1 text-dim">
            byte-identical{before && before.iteration !== round.iteration ? ` (${n(before.match)} before the repair)` : ""}
          </div>
        </>
      }
    />
  );
}

/** `confirmed`: the latest verify round reported zero divergences, so a repaired row's Rust output now equals the C output. */
export function DivergenceRows({ rows, confirmed }: { rows: DivergenceRow[]; confirmed: boolean }) {
  const [open, setOpen] = useState<string | null>(null);
  if (!rows.length) return null;
  return (
    <>
      {rows.map((r) => {
        const selected = open === r.key;
        return (
          <Row
            key={r.key}
            onClick={() => setOpen(selected ? null : r.key)}
            selected={selected}
            left={<div className="break-all font-mono text-lg md:text-xl">{show(r.c_output)}</div>}
            spine={
              <>
                <Mark state={r.resolved ? "registered" : "apart"} />
                <span className="max-w-full truncate font-mono text-sm text-dim" title={r.input}>
                  {r.input}
                </span>
              </>
            }
            right={
              r.resolved && confirmed ? (
                <>
                  <div className="break-all font-mono text-lg text-match md:text-xl">{show(r.c_output)}</div>
                  <div className="break-all font-mono text-sm text-dim line-through decoration-div/60">draft: {show(r.rust_output)}</div>
                </>
              ) : (
                <>
                  <div className={`break-all font-mono text-lg md:text-xl ${r.resolved ? "text-paper/70 line-through decoration-match/60" : "text-div"}`}>{show(r.rust_output)}</div>
                  {r.resolved && <div className="text-sm text-dim">repaired, re-verifying</div>}
                </>
              )
            }
          >
            {selected && (
              <div className="grid grid-cols-[minmax(0,1fr)_5.5rem_minmax(0,1fr)] md:grid-cols-[minmax(0,1fr)_11rem_minmax(0,1fr)] gap-4 px-1 pb-5 text-base">
                <div className="text-dim">
                  {r.function_guessed ? "Likely in" : "In"} <span className="font-mono text-paper">{r.function}</span>.
                  {r.resolved ? " Registered after the repair." : " Open: sent to the model."}
                </div>
                <div />
                <div className="text-right text-dim">
                  {r.repair ? (
                    <>
                      {r.repair.model ?? "Argon"} repair, round {r.repair.iteration ?? "?"}
                      {r.repair.seconds !== null ? `, ${Math.round(r.repair.seconds)} s` : ""}
                    </>
                  ) : (
                    "No repair yet."
                  )}
                </div>
              </div>
            )}
          </Row>
        );
      })}
    </>
  );
}

export function RepairRow({ repair, functions, guessed }: { repair: NonNullable<DivergenceRow["repair"]>; functions: string[]; guessed: boolean }) {
  return (
    <div className="border-t border-line bg-panel px-5 py-4">
      <p className="text-lg">
        <span className="font-bold text-match">
          Argon repair{repair.iteration !== null ? `, round ${repair.iteration}` : ""}
          {repair.seconds !== null ? ` · ${Math.round(repair.seconds)} s.` : "."}
        </span>{" "}
        Returned a corrected file
        {functions.length > 0 && (
          <>
            , {guessed ? "likely in" : "in"} <span className="font-mono">{functions.join(", ")}</span>
          </>
        )}
        . <span className="font-mono text-sm text-dim">{repair.model ?? ""}</span>
      </p>
      {repair.patterns.length > 0 && (
        <p className="mt-1 font-mono text-sm text-dim">Sent as patterns, not answers: {repair.patterns.join(" · ")}</p>
      )}
    </div>
  );
}

export function HoldoutRow({ match, total }: { match: number; total: number }) {
  const full = match === total;
  return (
    <Row
      left={<div className="text-xl">{n(total)} held-out inputs the repair never saw</div>}
      spine={<Mark state={full ? "registered" : "apart"} />}
      right={<div className={`font-mono text-xl ${full ? "text-match" : "text-div"}`}>{n(match)} / {n(total)} identical</div>}
    />
  );
}

export function CrashRow({ crash }: { crash: CrashView }) {
  const [open, setOpen] = useState(false);
  return (
    <Row
      onClick={crash.sanitizer_report ? () => setOpen((o) => !o) : undefined}
      selected={open}
      left={
        <>
          <div className="font-bold text-crash">
            {crash.c_version ?? "Old C"} · AddressSanitizer {crash.kind}
          </div>
          <div className="text-dim">{crash.cve ?? "historical crashing input"}</div>
        </>
      }
      spine={
        <>
          <Mark state="crash" />
          <span className="font-mono text-sm text-dim">{crash.input}</span>
        </>
      }
      right={
        <>
          <div className="font-bold text-match">Rust · rejects it cleanly</div>
          <div className="font-mono text-dim">{crash.rust_behavior}</div>
        </>
      }
    >
      {open && crash.sanitizer_report && (
        <pre className="mx-1 mb-5 max-h-56 overflow-auto rounded-md border border-line p-3 font-mono text-xs leading-relaxed text-paper/80">
          {crash.sanitizer_report}
        </pre>
      )}
    </Row>
  );
}
