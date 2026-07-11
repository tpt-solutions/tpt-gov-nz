import Link from "next/link";
import { fetchTpkData } from "../actions";

export const metadata = { title: "TPK Funding — My Gov NZ" };

export default async function TpkFundingPage() {
  const data = await fetchTpkData(["tpk:funding"]);
  const funding = data?.funding ?? [];

  return (
    <main>
      <Link href="/dept/tpk">← Back to Te Puni Kōkiri</Link>
      <h1>Funding</h1>

      <section>
        <h2>Your funding</h2>
        {funding.length === 0 ? (
          <p>No Te Puni Kōkiri funding on record.</p>
        ) : (
          <ul>
            {funding.map((f) => (
              <li key={f.grantId}>
                <strong>{f.grantId}</strong> — ${f.amount} ({f.status}): {f.purpose}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Apply for funding</h2>
        <Link href="/dept/tpk/funding/apply">Apply for a grant →</Link>
      </section>
    </main>
  );
}
