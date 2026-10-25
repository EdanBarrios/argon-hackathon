"use client";

/** The latest line from the pipeline, restated plainly each time it changes. */
export function Ticker({ text }: { text: string | null }) {
  if (!text) return null;
  return (
    <p key={text} className="rise text-lg leading-snug">
      <span className="text-dim">Now </span>
      {text}
    </p>
  );
}
