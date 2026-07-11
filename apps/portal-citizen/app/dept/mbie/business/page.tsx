import Link from "next/link";
import { fetchMbieData } from "../actions";

export const metadata = { title: "Business Registrations — My Gov NZ" };

export default async function MbieBusinessPage() {
  const data = await fetchMbieData(["mbie:business"]);
  const businesses = data?.businessRegistrations ?? [];

  return (
    <main>
      <Link href="/dept/mbie">← Back to business</Link>
      <h1>Business Registrations</h1>

      <section>
        <h2>Your businesses</h2>
        {businesses.length === 0 ? (
          <p>No business registrations on record.</p>
        ) : (
          <ul>
            {businesses.map((b) => (
              <li key={b.nzbn}>
                <strong>{b.entityName}</strong> — {b.entityType} ({b.status})
                <br />
                <small>NZBN {b.nzbn} · registered {b.registeredDate}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Register a new business</h2>
        <Link href="/dept/mbie/business/register">Register a business →</Link>
      </section>
    </main>
  );
}
