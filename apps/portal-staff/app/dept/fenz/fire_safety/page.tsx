import { fetchFenzDataForCitizen } from "../actions";

export const metadata = { title: "Fire safety — Fire and Emergency New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function FenzFireSafetyStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchFenzDataForCitizen(did, ["fenz:fire-safety"]);
  const item = data?.fire_safety;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/fenz?did=${encodeURIComponent(did)}`}>← Back to FENZ case file</Link>
      <h1>Fire safety</h1>
      {item ? (
        <div>
        <p><strong>property:</strong> {item.property}</p>
        <p><strong>grade:</strong> {item.grade}</p>
        <p><strong>lastInspection:</strong> {item.lastInspection}</p>
        </div>
      ) : (
        <p>No fire safety on file.</p>
      )}
    </main>
  );
}
