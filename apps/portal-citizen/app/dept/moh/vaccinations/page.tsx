import Link from "next/link";
import { fetchMohData } from "../actions";

export const metadata = { title: "Vaccinations — Health — My Gov NZ" };

export default async function MohVaccinationsPage() {
  const data = await fetchMohData(["moh:vaccinations"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/moh">← Back to Health</Link>
        <h1>Vaccinations</h1>
        <p>Unable to load your vaccination records.</p>
      </main>
    );
  }

  const vaccinations = data.vaccinations ?? [];

  return (
    <main>
      <Link href="/dept/moh">← Back to Health</Link>
      <h1>Vaccinations</h1>

      {vaccinations.length === 0 && <p>No vaccination records found.</p>}

      <ul>
        {vaccinations.map((v, i) => (
          <li key={i}>
            {v.vaccine} — {v.date}
            {v.dueForBooster && <strong style={{ color: "darkorange" }}> (booster due)</strong>}
          </li>
        ))}
      </ul>
    </main>
  );
}
