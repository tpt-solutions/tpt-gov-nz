import Link from "next/link";
import { fetchWorksafeDataForCitizen } from "./actions";

export const metadata = { title: "WorkSafe New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WorksafeStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>WorkSafe New Zealand — Case File</h1>
        <p>No citizen selected. Enter a DID to view their WorkSafe records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchWorksafeDataForCitizen(did, ["worksafe:inspections", "worksafe:investigations"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>WorkSafe New Zealand — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load WorkSafe information for this citizen.</p>}

      {data && (
        <>
          <p>WorkSafe ID: ••••{data.worksafeId.slice(-4)}</p>

      <section>
        <h2>Inspections</h2>
        {data && data.inspections ? (
          <Link href={`/dept/worksafe/inspections?did=${encodeURIComponent(did)}`}>View inspections</Link>
        ) : (
          <p>No inspections on file.</p>
        )}
      </section>

      <section>
        <h2>Investigations</h2>
        {data && data.investigations ? (
          <Link href={`/dept/worksafe/investigations?did=${encodeURIComponent(did)}`}>View investigations</Link>
        ) : (
          <p>No investigations on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
