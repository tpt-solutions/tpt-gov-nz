import Link from "next/link";
import { fetchAccData } from "../actions";

export const metadata = { title: "ACC Rehabilitation — My Gov NZ" };

export default async function AccRehabilitationPage() {
  const data = await fetchAccData(["acc:rehabilitation"]);
  const plans = data?.rehabilitation ?? [];

  return (
    <main>
      <Link href="/dept/acc">← Back to ACC</Link>
      <h1>ACC Rehabilitation</h1>

      {plans.length === 0 ? (
        <p>No rehabilitation plans on record.</p>
      ) : (
        <ul>
          {plans.map((r) => (
            <li key={r.planId}>
              <strong>{r.planId}</strong> — {r.status}: {r.description}
              {r.provider ? ` (provider: ${r.provider})` : ""}
              {r.nextReview ? ` — next review ${r.nextReview}` : ""}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
