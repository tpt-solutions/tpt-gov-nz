import Link from "next/link";
import { fetchHudData } from "../actions";

export const metadata = { title: "Tenancy — My Gov NZ" };

export default async function HudTenancyPage() {
  const data = await fetchHudData(["hud:tenancy"]);
  const tenancies = data?.tenancies ?? [];

  return (
    <main>
      <Link href="/dept/hud">← Back to housing</Link>
      <h1>Tenancy</h1>

      {tenancies.length === 0 ? (
        <p>No tenancy on record.</p>
      ) : (
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
