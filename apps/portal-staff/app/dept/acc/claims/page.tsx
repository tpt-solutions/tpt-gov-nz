import Link from "next/link";
import { fetchAccDataForCitizen } from "../actions";

export const metadata = { title: "ACC Claims — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function AccStaffClaimsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>ACC Claims — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchAccDataForCitizen(did, ["acc:claims"]);
  const claims = data?.claims;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/acc?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>ACC Claims — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load claims.</p>}
      {data && (!claims || claims.length === 0) && <p>No claims on record.</p>}

      {claims && claims.length > 0 && (
        <ul>
          {claims.map((c) => (
            <li key={c.claimNumber}>
              <strong>{c.claimNumber}</strong> — {c.claimType} ({c.status}): {c.description}
              <br />
              <small>Injury date: {c.injuryDate}{c.weeklyCompensation != null ? ` · $${c.weeklyCompensation}/week` : ""}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
