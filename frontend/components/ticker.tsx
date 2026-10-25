"use client";

import { useEffect, useState } from "react";

/** Terminal line that types out the latest pipeline summary whenever it changes. */
export function Ticker({ text }: { text: string | null }) {
  const [shown, setShown] = useState("");
  useEffect(() => {
    if (!text) return;
    let i = 0;
    const step = Math.max(1, Math.ceil(text.length / 40));
    const id = setInterval(() => {
      i = Math.min(text.length, i + step);
      setShown(text.slice(0, i));
      if (i >= text.length) clearInterval(id);
    }, 18);
    return () => clearInterval(id);
  }, [text]);
  if (!text) return null;
  return (
    <div className="panel flex items-center gap-3 px-4 py-2.5 font-mono text-base">
      <span className="text-match">theseus $</span>
      <span className="min-w-0 flex-1 truncate text-neutral-100" title={text}>
        {shown}
        <span className="cursor text-match">▌</span>
      </span>
    </div>
  );
}
