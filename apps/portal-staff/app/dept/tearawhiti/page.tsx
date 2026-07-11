import Link from "next/link";
import { fetchTearawhitiDataForCitizen } from "./actions";

export const metadata = { title: "Te Arawhiti — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TearawhitiStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Te Arawhiti — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Te Arawhiti records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchTearawhitiDataForCitizen(did, ["tearawhiti:treaty-settlements", "tearawhiti:engagements"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Te Arawhiti — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Te Arawhiti information for this citizen.</p>}

      {data && (
        <>
          <p>Te Arawhiti ID: ••••{data.tearawhitiId.slice(-4)}</p>

      <section>
        <h2>Treaty settlements</h2>
        {data && data.treaty_settlements ? (
          <Link href={`/dept/tearawhiti/treaty_settlements?did=${encodeURIComponent(did)}`}>View treaty settlements</Link>
        ) : (
          <p>No treaty settlements on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data && data.engagements ? (
          <Link href={`/dept/tearawhiti/engagements?did=${encodeURIComponent(did)}`}>View engagements</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
