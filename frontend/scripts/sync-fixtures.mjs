// Copies ../contracts/fixtures/*.json into public/fixtures/ so the UI can
// fall back to sample data when the API is down. Runs in predev and prebuild.
// The copied files are committed too, so the frontend also works standalone.
import { copyFileSync, existsSync, mkdirSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const src = join(here, "..", "..", "contracts", "fixtures");
const dest = join(here, "..", "public", "fixtures");

if (!existsSync(src)) {
  console.warn(`[sync-fixtures] ${src} not found; keeping committed copies.`);
  process.exit(0);
}

mkdirSync(dest, { recursive: true });
const files = readdirSync(src).filter((f) => f.endsWith(".json"));
for (const f of files) copyFileSync(join(src, f), join(dest, f));
console.log(`[sync-fixtures] copied ${files.length} fixture(s) to public/fixtures/`);
