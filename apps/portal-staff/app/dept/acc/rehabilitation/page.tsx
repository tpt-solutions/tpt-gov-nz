import Link from "next/link";
import { fetchAccDataForCitizen } from "../actions";

export const metadata = { title: "ACC Rehabilitation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function AccStaffRehabilitationPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>ACC Rehabilitation — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchAccDataForCitizen(did, ["acc:rehabilitation"]);
  const plans = data?.rehabilitation;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/acc?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>ACC Rehabilitation — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load rehabilitation.</p>}
      {data && (!plans || plans.length === 0) && <p>No rehabilitation plans on record.</p>}

      {plans && plans.length > 0 && (
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
