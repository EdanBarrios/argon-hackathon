# Theseus frontend

Next.js (App Router, Tailwind, no component libraries, system fonts only).

```bash
cd frontend
npm install
npm run dev          # http://localhost:3000, syncs contracts/fixtures first
```

`NEXT_PUBLIC_API_URL` sets the backend (default `http://localhost:8000`).

## Views

- `/` Start run (POST /jobs `{"library":"cjson","modules":["parser","printer"]}`), load a job by id, sample links.
- `/jobs/<id>` run view, polls the API every 1s until a terminal status.
- `/jobs/<anything>?fixture=running|completed|failed` sample data, rendered on the server, no backend needed.

If the API is unreachable on first load, the run view falls back to the completed fixture and shows a "sample data" badge.

## Layout

- `lib/api.ts` types (mirror `contracts/fixtures`), API calls, `usePollJob`.
- `lib/fixtures.ts` fixture helpers; `lib/fixtures-static.ts` server-side copy for `?fixture=`.
- `lib/derive.ts` view-model helpers (divergence rows, crash cards, chart series, hero stats, diff).
- `components/` one file per run-view section.
- `scripts/sync-fixtures.mjs` copies `../contracts/fixtures/*.json` to `public/fixtures/` (runs on predev/prebuild).
