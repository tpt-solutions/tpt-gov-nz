import Link from "next/link";
import { fetchDpmcDataForCitizen } from "./actions";

export const metadata = { title: "Department of the Prime Minister and Cabinet — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DpmcStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Department of the Prime Minister and Cabinet — Case File</h1>
        <p>No citizen selected. Enter a DID to view their DPMC records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchDpmcDataForCitizen(did, ["dpmc:honours", "dpmc:engagements"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Department of the Prime Minister and Cabinet — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load DPMC information for this citizen.</p>}

      {data && (
        <>
          <p>DPMC ID: ••••{data.dpmcId.slice(-4)}</p>

      <section>
        <h2>Honours</h2>
        {data && data.honours ? (
          <Link href={`/dept/dpmc/honours?did=${encodeURIComponent(did)}`}>View honours</Link>
        ) : (
          <p>No honours on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data && data.engagements ? (
          <Link href={`/dept/dpmc/engagements?did=${encodeURIComponent(did)}`}>View engagements</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
