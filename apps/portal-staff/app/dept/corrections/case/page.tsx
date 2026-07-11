import Link from "next/link";
import { fetchCorrectionsDataForCitizen } from "../actions";

export const metadata = { title: "Case — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CorrectionsStaffCasePage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Case — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchCorrectionsDataForCitizen(did, ["corrections:case"]);
  const cases = data?.case;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/corrections?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Case — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load cases.</p>}
      {data && (!cases || cases.length === 0) && <p>No cases on record.</p>}

      {cases && cases.length > 0 && (
        <ul>
          {cases.map((c) => (
            <li key={c.caseNumber}>
              <strong>{c.caseNumber}</strong> — {c.sentenceType}
              <br />
              <small>{c.startDate}{c.endDate ? ` – ${c.endDate}` : " – ongoing"} · {c.summary}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
