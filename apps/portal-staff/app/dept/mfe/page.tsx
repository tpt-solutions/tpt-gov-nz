import Link from "next/link";
import { fetchMfeDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for the Environment — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfeStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for the Environment — Case File</h1>
        <p>No citizen selected. Enter a DID to view their MfE records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMfeDataForCitizen(did, ["mfe:emissions", "mfe:reports"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for the Environment — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load MfE information for this citizen.</p>}

      {data && (
        <>
          <p>MfE ID: ••••{data.mfeId.slice(-4)}</p>

      <section>
        <h2>Emissions</h2>
        {data && data.emissions ? (
          <Link href={`/dept/mfe/emissions?did=${encodeURIComponent(did)}`}>View emissions</Link>
        ) : (
          <p>No emissions on file.</p>
        )}
      </section>

      <section>
        <h2>Reports</h2>
        {data && data.reports ? (
          <Link href={`/dept/mfe/reports?did=${encodeURIComponent(did)}`}>View reports</Link>
        ) : (
          <p>No reports on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
