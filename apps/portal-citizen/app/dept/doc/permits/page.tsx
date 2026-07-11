import Link from "next/link";
import ApplyPermitForm from "./apply/form";
import { fetchDocData } from "../actions";

export const metadata = { title: "Conservation Permits — My Gov NZ" };

export default async function DocPermitsPage() {
  const data = await fetchDocData(["doc:permits"]);
  const permits = data?.permits ?? [];

  return (
    <main>
      <Link href="/dept/doc">← Back to conservation</Link>
      <h1>Conservation Permits</h1>

      <section>
        <h2>Your permits</h2>
        {permits.length === 0 ? (
          <p>No conservation permits on record.</p>
        ) : (
          <ul>
            {permits.map((p) => (
              <li key={p.permitNumber}>
                <strong>{p.permitNumber}</strong> — {p.activity} ({p.status})
                <br />
                <small>
                  {p.location} · expires {p.expiresDate}
                </small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Apply for a conservation permit</h2>
        <ApplyPermitForm />
      </section>
    </main>
  );
}
