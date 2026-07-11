import Link from "next/link";
import { fetchPublicserviceDataForCitizen } from "./actions";

export const metadata = { title: "Te Kawa Mataaho Public Service Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PublicserviceStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Te Kawa Mataaho Public Service Commission — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Public Service records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchPublicserviceDataForCitizen(did, ["publicservice:workforce", "publicservice:agency-ratings"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Te Kawa Mataaho Public Service Commission — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Public Service information for this citizen.</p>}

      {data && (
        <>
          <p>Public Service ID: ••••{data.publicserviceId.slice(-4)}</p>

      <section>
        <h2>Workforce</h2>
        {data && data.workforce ? (
          <Link href={`/dept/publicservice/workforce?did=${encodeURIComponent(did)}`}>View workforce</Link>
        ) : (
          <p>No workforce on file.</p>
        )}
      </section>

      <section>
        <h2>Agency ratings</h2>
        {data && data.agency_ratings ? (
          <Link href={`/dept/publicservice/agency_ratings?did=${encodeURIComponent(did)}`}>View agency ratings</Link>
        ) : (
          <p>No agency ratings on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
