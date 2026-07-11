import Link from "next/link";
import { fetchEroDataForCitizen } from "./actions";

export const metadata = { title: "Education Review Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EroStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Education Review Office — Case File</h1>
        <p>No citizen selected. Enter a DID to view their ERO records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchEroDataForCitizen(did, ["ero:reviews", "ero:reports"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Education Review Office — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load ERO information for this citizen.</p>}

      {data && (
        <>
          <p>ERO ID: ••••{data.eroId.slice(-4)}</p>

      <section>
        <h2>Reviews</h2>
        {data && data.reviews ? (
          <Link href={`/dept/ero/reviews?did=${encodeURIComponent(did)}`}>View reviews</Link>
        ) : (
          <p>No reviews on file.</p>
        )}
      </section>

      <section>
        <h2>Reports</h2>
        {data && data.reports ? (
          <Link href={`/dept/ero/reports?did=${encodeURIComponent(did)}`}>View reports</Link>
        ) : (
          <p>No reports on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
