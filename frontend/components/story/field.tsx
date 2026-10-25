"use client";

import { useEffect, useRef } from "react";

/**
 * The verifier as a press sheet: one mark per input, printed in two plates.
 * C is the blue plate, Rust the orange one. `reg` (0..1) brings the plates of
 * agreeing inputs into register, where they read as one paper-white mark.
 * The `divergent` inputs keep their plates apart, in red, until `repair`
 * (0..1) brings them in too. Cell order carries no meaning.
 */
export interface FieldState {
  reg: number;
  repair: number;
}

const COLS = 34; // 34 x 17 = 578, the run's input count
const PAPER = "#ece7dc";
const BLUE = "#74b3ff";
const ORANGE = "#ff7a3a";
const RED = "#ff5a3c";

export function Field({ state, total, divergent }: { state: FieldState; total: number; divergent: number }) {
  const ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = ref.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    const rows = Math.ceil(total / COLS);

    // Deterministic plate offsets per mark and a spread set of divergent marks.
    const angle = new Float32Array(total);
    for (let i = 0; i < total; i++) angle[i] = ((i * 2654435761) % 360) * (Math.PI / 180);
    const div = new Set<number>();
    for (let k = 0; k < divergent; k++) div.add((k * 193 + 11) % total);

    let w = 0;
    let h = 0;
    let dpr = 1;
    let last = -1;
    let raf = 0;
    const resize = () => {
      const r = canvas.getBoundingClientRect();
      dpr = Math.min(2, window.devicePixelRatio || 1);
      w = r.width;
      h = r.height;
      canvas.width = Math.round(w * dpr);
      canvas.height = Math.round(h * dpr);
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      last = -1;
    };
    const ro = new ResizeObserver(resize);
    ro.observe(canvas);
    resize();

    const dot =(x: number, y: number, r: number, color: string) => {
      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fill();
    };

    const draw = () => {
      raf = requestAnimationFrame(draw);
      const key = state.reg * 1000 + state.repair * 1000000 + w;
      if (key === last) return;
      last = key;

      ctx.clearRect(0, 0, w, h);
      const pitch = Math.min(w / (COLS + 1), h / (rows + 1));
      const r = pitch * 0.17;
      const spread = pitch * 0.42;
      const x0 = (w - pitch * (COLS - 1)) / 2;
      const y0 = (h - pitch * (rows - 1)) / 2;

      for (let i = 0; i < total; i++) {
        const cx = x0 + (i % COLS) * pitch;
        const cy = y0 + Math.floor(i / COLS) * pitch;
        const isDiv = div.has(i);
        const p = isDiv ? state.repair : state.reg;
        const apart = 1 - p;
        if (apart < 0.015) {
          ctx.globalCompositeOperation = "source-over";
          dot(cx, cy, r, PAPER);
          continue;
        }
        const dx = Math.cos(angle[i]) * spread * apart;
        const dy = Math.sin(angle[i]) * spread * apart;
        ctx.globalCompositeOperation = "screen";
        ctx.globalAlpha = 0.55 + 0.45 * apart;
        dot(cx - dx / 2, cy - dy / 2, r, BLUE);
        dot(cx + dx / 2, cy + dy / 2, isDiv ? r * 1.25 : r, isDiv ? RED : ORANGE);
        ctx.globalAlpha = 1;
        if (isDiv && apart > 0.5) {
          ctx.globalCompositeOperation = "source-over";
          ctx.strokeStyle = RED;
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.arc(cx, cy, pitch * 0.4, 0, Math.PI * 2);
          ctx.stroke();
        }
      }
      ctx.globalCompositeOperation = "source-over";
    };
    raf = requestAnimationFrame(draw);
    return () => {
      cancelAnimationFrame(raf);
      ro.disconnect();
    };
  }, [state, total, divergent]);

  return <canvas ref={ref} className="block h-full w-full" aria-hidden="true" />;
}
