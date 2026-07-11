import { fetchRetirementDataForCitizen } from "../actions";

export const metadata = { title: "Retirement plan — Retirement Commission (Te Ara Ahunga Ora) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RetirementRetirementPlanStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchRetirementDataForCitizen(did, ["retirement:retirement-plan"]);
  const item = data?.retirement_plan;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/retirement?did=${encodeURIComponent(did)}`}>← Back to Retirement case file</Link>
      <h1>Retirement plan</h1>
      {item ? (
        <div>
        <p><strong>hasPlan:</strong> {item.hasPlan}</p>
        <p><strong>retirementAge:</strong> {item.retirementAge}</p>
        <p><strong>lastReview:</strong> {item.lastReview}</p>
        </div>
      ) : (
        <p>No retirement plan on file.</p>
      )}
    </main>
  );
}
