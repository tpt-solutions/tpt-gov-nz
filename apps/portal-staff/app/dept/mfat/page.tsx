import Link from "next/link";
import { fetchMfatDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Foreign Affairs and Trade — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfatStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Foreign Affairs and Trade — Case File</h1>
        <p>No citizen selected. Enter a DID to view their MFAT records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMfatDataForCitizen(did, ["mfat:overseas-missions", "mfat:travel-advisories"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Foreign Affairs and Trade — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load MFAT information for this citizen.</p>}

      {data && (
        <>
          <p>MFAT ID: ••••{data.mfatId.slice(-4)}</p>

      <section>
        <h2>Overseas missions</h2>
        {data && data.overseas_missions ? (
          <Link href={`/dept/mfat/overseas_missions?did=${encodeURIComponent(did)}`}>View overseas missions</Link>
        ) : (
          <p>No overseas missions on file.</p>
        )}
      </section>

      <section>
        <h2>Travel advisories</h2>
        {data && data.travel_advisories ? (
          <Link href={`/dept/mfat/travel_advisories?did=${encodeURIComponent(did)}`}>View travel advisories</Link>
        ) : (
          <p>No travel advisories on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
