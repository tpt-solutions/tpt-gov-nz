import Link from "next/link";
import { fetchLinzDataForCitizen } from "../actions";

export const metadata = { title: "Titles — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function LinzStaffTitlesPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Titles — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchLinzDataForCitizen(did, ["linz:titles"]);
  const titles = data?.titles;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/linz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Titles — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load titles.</p>}
      {data && (!titles || titles.length === 0) && <p>No property titles on record.</p>}

      {titles && titles.length > 0 && (
        <ul>
          {titles.map((t) => (
            <li key={t.titleNumber}>
              <strong>{t.titleNumber}</strong> — {t.estateType}
              <br />
              <small>{t.propertyAddress} · {t.landAreaSqm} m²</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
