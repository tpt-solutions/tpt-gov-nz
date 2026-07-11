import Link from "next/link";
import { fetchCorrectionsData } from "../actions";

export const metadata = { title: "Case — My Gov NZ" };

export default async function CorrectionsCasePage() {
  const data = await fetchCorrectionsData(["corrections:case"]);
  const cases = data?.case ?? [];

  return (
    <main>
      <Link href="/dept/corrections">← Back to Corrections</Link>
      <h1>Case</h1>

      <section>
        <h2>Your cases</h2>
        {cases.length === 0 ? (
          <p>No cases on record.</p>
        ) : (
          <ul>
            {cases.map((c) => (
              <li key={c.caseNumber}>
                <strong>{c.caseNumber}</strong> — {c.sentenceType}
                <br />
                <small>
                  {c.startDate}{c.endDate ? ` – ${c.endDate}` : " – ongoing"} · {c.summary}
                </small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Request a sentence summary</h2>
        <Link href="/dept/corrections/case/request-summary">Request summary →</Link>
      </section>
    </main>
  );
}
