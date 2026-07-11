import Link from "next/link";
import { fetchHudDataForCitizen } from "../actions";

export const metadata = { title: "Tenancy — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function HudStaffTenancyPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Tenancy — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchHudDataForCitizen(did, ["hud:tenancy"]);
  const tenancies = data?.tenancies;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/hud?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Tenancy — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load tenancy.</p>}
      {data && (!tenancies || tenancies.length === 0) && <p>No tenancy on record.</p>}

      {tenancies && tenancies.length > 0 && (
        <ul>
          {tenancies.map((t) => (
            <li key={t.tenancyId}>
              <strong>{t.propertyAddress}</strong> — {t.status}
              <br />
              <small>
                ${t.weeklyRent}/week{t.incomeRelatedRent ? " (income-related rent)" : ""} · started {t.startDate}
              </small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
