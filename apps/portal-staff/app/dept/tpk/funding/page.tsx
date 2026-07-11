import Link from "next/link";
import { fetchTpkDataForCitizen } from "../actions";

export const metadata = { title: "Funding — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TpkStaffFundingPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Funding — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchTpkDataForCitizen(did, ["tpk:funding"]);
  const funding = data?.funding;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tpk?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Funding — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load funding.</p>}
      {data && (!funding || funding.length === 0) && <p>No Te Puni Kōkiri funding on record.</p>}

      {funding && funding.length > 0 && (
        <ul>
          {funding.map((f) => (
            <li key={f.grantId}>
              <strong>{f.grantId}</strong> — ${f.amount} ({f.status}): {f.purpose}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
