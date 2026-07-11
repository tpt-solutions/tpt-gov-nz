import Link from "next/link";
import { fetchTecDataForCitizen } from "./actions";

export const metadata = { title: "Tertiary Education Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TecStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Tertiary Education Commission — Case File</h1>
        <p>No citizen selected. Enter a DID to view their TEC records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchTecDataForCitizen(did, ["tec:funding", "tec:courses"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Tertiary Education Commission — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load TEC information for this citizen.</p>}

      {data && (
        <>
          <p>TEC ID: ••••{data.tecId.slice(-4)}</p>

      <section>
        <h2>Funding</h2>
        {data && data.funding ? (
          <Link href={`/dept/tec/funding?did=${encodeURIComponent(did)}`}>View funding</Link>
        ) : (
          <p>No funding on file.</p>
        )}
      </section>

      <section>
        <h2>Courses</h2>
        {data && data.courses ? (
          <Link href={`/dept/tec/courses?did=${encodeURIComponent(did)}`}>View courses</Link>
        ) : (
          <p>No courses on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
