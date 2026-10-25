import type { Series } from "@/lib/derive";

// Tiny inline SVG line chart (no chart library). One neutral line per agent,
// labeled at its last point; points that reach 100% turn green.
const W = 560;
const H = 220;
const PAD = { l: 48, r: 90, t: 14, b: 32 };

export function MatchChart({ series }: { series: Series[] }) {
  if (!series.length || series.every((s) => !s.points.length)) {
    return <p className="text-lg text-dim">No verify rounds yet.</p>;
  }
  const maxIt = Math.max(2, ...series.flatMap((s) => s.points.map((p) => p.iteration)));
  const x = (it: number) => PAD.l + ((it - 1) / (maxIt - 1)) * (W - PAD.l - PAD.r);
  const y = (r: number) => PAD.t + (1 - r) * (H - PAD.t - PAD.b);
  const shades = ["#e5e5e5", "#a3a3a3", "#737373"];

  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="h-auto w-full" role="img" aria-label="Match rate per iteration">
      {[0, 0.5, 1].map((t) => (
        <g key={t}>
          <line x1={PAD.l} x2={W - PAD.r} y1={y(t)} y2={y(t)} stroke={t === 1 ? "#22c55e" : "#262626"} strokeDasharray={t === 1 ? "4 4" : undefined} />
          <text x={PAD.l - 8} y={y(t) + 5} textAnchor="end" fontSize="14" fill="#a3a3a3">
            {t * 100}%
          </text>
        </g>
      ))}
      {Array.from({ length: maxIt }, (_, i) => (
        <text key={i} x={x(i + 1)} y={H - 8} textAnchor="middle" fontSize="14" fill="#a3a3a3">
          {i + 1}
        </text>
      ))}
      {series.map((s, si) => {
        const color = shades[si % shades.length];
        const pts = s.points;
        const last = pts[pts.length - 1];
        return (
          <g key={s.agent_id}>
            <polyline
              points={pts.map((p) => `${x(p.iteration)},${y(p.match_rate)}`).join(" ")}
              fill="none"
              stroke={color}
              strokeWidth="3"
            />
            {pts.map((p) => (
              <circle
                key={p.iteration}
                cx={x(p.iteration)}
                cy={y(p.match_rate)}
                r="5"
                fill={p.match_rate >= 1 ? "#22c55e" : color}
              />
            ))}
            <text x={x(last.iteration) + 10} y={y(last.match_rate) + 5} fontSize="15" fill={color}>
              {s.label}
            </text>
          </g>
        );
      })}
    </svg>
  );
}
