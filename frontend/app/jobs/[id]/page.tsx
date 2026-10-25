import { Suspense } from "react";
import { RunView } from "@/components/run-view";
import { isFixtureKind } from "@/lib/fixtures";
import { staticFixture } from "@/lib/fixtures-static";

// Run view. Params and searchParams are request-time data, so they are read
// inside a Suspense boundary (required with cacheComponents). With ?fixture=,
// the fixture is rendered on the server so the page works with no backend.
export default function JobPage(props: PageProps<"/jobs/[id]">) {
  return (
    <Suspense
      fallback={
        <main className="p-10">
          <h1 className="text-lg font-extrabold tracking-[-0.02em] text-paper">Theseus</h1>
          <p className="mt-6 text-xl text-dim">Loading run…</p>
        </main>
      }
    >
      <JobLoader {...props} />
    </Suspense>
  );
}

async function JobLoader({ params, searchParams }: PageProps<"/jobs/[id]">) {
  const { id } = await params;
  const sp = await searchParams;
  const fixture = isFixtureKind(sp.fixture) ? sp.fixture : null;
  return <RunView key={`${id}:${fixture}`} id={id} fixture={fixture} initial={fixture ? staticFixture(fixture) : undefined} />;
}
