import Link from "next/link";
import ClaimsForm from "./form";
import { fetchAccData } from "../actions";

export const metadata = { title: "ACC Claims — My Gov NZ" };

export default async function AccClaimsPage() {
  const data = await fetchAccData(["acc:claims"]);
  const claims = data?.claims ?? [];

  return (
    <main>
      <Link href="/dept/acc">← Back to ACC</Link>
      <h1>ACC Claims</h1>

      <section>
        <h2>Your claims</h2>
        {claims.length === 0 ? (
          <p>No claims on record.</p>
        ) : (
          <ul>
            {claims.map((c) => (
              <li key={c.claimNumber}>
                <strong>{c.claimNumber}</strong> — {c.claimType} ({c.status}): {c.description}
                {c.weeklyCompensation != null ? ` — $${c.weeklyCompensation}/week` : ""}
                <br />
                <small>Injury date: {c.injuryDate}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Lodge a new claim</h2>
        <ClaimsForm />
      </section>
    </main>
  );
}
