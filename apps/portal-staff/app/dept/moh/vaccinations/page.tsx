import Link from "next/link";
import { fetchMohDataForCitizen } from "../actions";

export const metadata = { title: "Vaccinations — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MohStaffVaccinationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Vaccinations — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMohDataForCitizen(did, ["moh:vaccinations"]);
  const vaccinations = data?.vaccinations ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moh?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Vaccinations — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load vaccinations.</p>}
      {data && vaccinations.length === 0 && <p>No vaccination records.</p>}

      {data && (
        <ul>
          {vaccinations.map((v, i) => (
            <li key={i}>
              {v.vaccine} — {v.date}
              {v.dueForBooster && <strong style={{ color: "darkorange" }}> (booster due)</strong>}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
