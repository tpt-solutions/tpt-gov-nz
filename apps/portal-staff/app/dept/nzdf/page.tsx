import Link from "next/link";
import { fetchNzdfDataForCitizen } from "./actions";

export const metadata = { title: "New Zealand Defence Force — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzdfStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>New Zealand Defence Force — Case File</h1>
        <p>No citizen selected. Enter a DID to view their NZDF records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchNzdfDataForCitizen(did, ["nzdf:service-records", "nzdf:deployments"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>New Zealand Defence Force — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load NZDF information for this citizen.</p>}

      {data && (
        <>
          <p>NZDF ID: ••••{data.nzdfId.slice(-4)}</p>

      <section>
        <h2>Service records</h2>
        {data && data.service_records ? (
          <Link href={`/dept/nzdf/service_records?did=${encodeURIComponent(did)}`}>View service records</Link>
        ) : (
          <p>No service records on file.</p>
        )}
      </section>

      <section>
        <h2>Deployments</h2>
        {data && data.deployments ? (
          <Link href={`/dept/nzdf/deployments?did=${encodeURIComponent(did)}`}>View deployments</Link>
        ) : (
          <p>No deployments on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
