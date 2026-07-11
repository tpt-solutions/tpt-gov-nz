import Link from "next/link";
import { fetchLinzData } from "../actions";

export const metadata = { title: "Property Titles — My Gov NZ" };

export default async function LinzTitlesPage() {
  const data = await fetchLinzData(["linz:titles"]);
  const titles = data?.titles ?? [];

  return (
    <main>
      <Link href="/dept/linz">← Back to LINZ</Link>
      <h1>Property Titles</h1>

      <section>
        <h2>Your titles</h2>
        {titles.length === 0 ? (
          <p>No property titles on record.</p>
        ) : (
          <ul>
            {titles.map((t) => (
              <li key={t.titleNumber}>
                <strong>{t.titleNumber}</strong> — {t.estateType}
                <br />
                <small>
                  {t.propertyAddress} · {t.landAreaSqm} m²
                </small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Request a title copy</h2>
        <Link href="/dept/linz/titles/request-copy">Request a copy →</Link>
      </section>
    </main>
  );
}
