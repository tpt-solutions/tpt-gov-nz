import Link from "next/link";
import { fetchWinzDataForCitizen } from "../actions";

export const metadata = { title: "Benefits — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WinzStaffBenefitsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Benefits — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchWinzDataForCitizen(did, ["winz:benefits"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/winz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Benefits — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load benefits for this citizen.</p>}

      {data && (
        <>
          {data.activeBenefits.length === 0 && <p>No active benefits.</p>}
          {data.activeBenefits.map((b) => (
            <section key={b.type} style={{ marginBottom: "1rem" }}>
              <h2>{b.type}</h2>
              <dl>
                <dt>Weekly amount</dt>
                <dd>${Number(b.weeklyAmount).toFixed(2)}</dd>
                <dt>Status</dt>
                <dd>{b.status}</dd>
                {b.startDate && (<><dt>Started</dt><dd>{b.startDate}</dd></>)}
                {b.reviewDate && (<><dt>Review date</dt><dd>{b.reviewDate}</dd></>)}
              </dl>
            </section>
          ))}
        </>
      )}
    </main>
  );
}
