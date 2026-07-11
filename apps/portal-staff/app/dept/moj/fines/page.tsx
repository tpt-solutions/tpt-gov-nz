import Link from "next/link";
import { fetchMojDataForCitizen } from "../actions";

export const metadata = { title: "MOJ Fines — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MojStaffFinesPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Fines — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMojDataForCitizen(did, ["moj:fines"]);
  const fines = data?.fines;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moj?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Fines — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load fines.</p>}
      {data && (!fines || fines.length === 0) && <p>No fines on record.</p>}

      {fines && fines.length > 0 && (
        <ul>
          {fines.map((f) => (
            <li key={f.fineNumber}>
              <strong>{f.fineNumber}</strong> — {f.fineType} ({f.status}): ${f.amount}
              <br />
              <small>{f.description} — due {f.dueDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
