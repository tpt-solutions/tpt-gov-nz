import Link from "next/link";
import { fetchRetirementDataForCitizen } from "./actions";

export const metadata = { title: "Retirement Commission (Te Ara Ahunga Ora) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RetirementStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Retirement Commission (Te Ara Ahunga Ora) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Retirement records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchRetirementDataForCitizen(did, ["retirement:retirement-plan", "retirement:guidance"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Retirement Commission (Te Ara Ahunga Ora) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Retirement information for this citizen.</p>}

      {data && (
        <>
          <p>Retirement ID: ••••{data.retirementId.slice(-4)}</p>

      <section>
        <h2>Retirement plan</h2>
        {data && data.retirement_plan ? (
          <Link href={`/dept/retirement/retirement_plan?did=${encodeURIComponent(did)}`}>View retirement plan</Link>
        ) : (
          <p>No retirement plan on file.</p>
        )}
      </section>

      <section>
        <h2>Guidance</h2>
        {data && data.guidance ? (
          <Link href={`/dept/retirement/guidance?did=${encodeURIComponent(did)}`}>View guidance</Link>
        ) : (
          <p>No guidance on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
