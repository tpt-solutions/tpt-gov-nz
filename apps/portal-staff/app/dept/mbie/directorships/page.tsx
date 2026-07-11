import Link from "next/link";
import { fetchMbieDataForCitizen } from "../actions";

export const metadata = { title: "Directorships — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MbieStaffDirectorshipsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Directorships — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMbieDataForCitizen(did, ["mbie:directorships"]);
  const directorships = data?.directorships;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mbie?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Directorships — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load directorships.</p>}
      {data && (!directorships || directorships.length === 0) && <p>No directorships on record.</p>}

      {directorships && directorships.length > 0 && (
        <ul>
          {directorships.map((d) => (
            <li key={`${d.nzbn}-${d.appointedDate}`}>
              <strong>{d.entityName}</strong> — {d.role}
              <br />
              <small>NZBN {d.nzbn} · appointed {d.appointedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
