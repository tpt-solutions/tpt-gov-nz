import Link from "next/link";
import { fetchCustomsDataForCitizen } from "../actions";

export const metadata = { title: "Declarations — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CustomsStaffDeclarationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Declarations — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchCustomsDataForCitizen(did, ["customs:declarations"]);
  const declarations = data?.declarations;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/customs?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Declarations — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load declarations.</p>}
      {data && (!declarations || declarations.length === 0) && <p>No declarations on record.</p>}

      {declarations && declarations.length > 0 && (
        <ul>
          {declarations.map((d) => (
            <li key={d.declarationId}>
              <strong>{d.declarationId}</strong> — from {d.countryFrom} ({d.status}): {d.goodsDeclared}
              <br />
              <small>Declared {d.date}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
