import Link from "next/link";
import { fetchTpkDataForCitizen } from "../actions";

export const metadata = { title: "Programmes — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TpkStaffProgrammesPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Programmes — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchTpkDataForCitizen(did, ["tpk:programmes"]);
  const programmes = data?.programmes;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tpk?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Programmes — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load programmes.</p>}
      {data && (!programmes || programmes.length === 0) && <p>No Te Puni Kōkiri programmes on record.</p>}

      {programmes && programmes.length > 0 && (
        <ul>
          {programmes.map((p) => (
            <li key={p.programmeName}>
              <strong>{p.programmeName}</strong> — {p.status}
              <br />
              <small>{p.region}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
