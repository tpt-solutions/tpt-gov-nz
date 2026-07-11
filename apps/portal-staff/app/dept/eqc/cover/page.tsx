import Link from "next/link";
import { fetchEqcDataForCitizen } from "../actions";

export const metadata = { title: "Cover — Earthquake Commission (Toka Tū Ake) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EqcCoverStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchEqcDataForCitizen(did, ["eqc:cover"]);
  const item = data?.cover;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/eqc?did=${encodeURIComponent(did)}`}>← Back to EQC case file</Link>
      <h1>Cover</h1>
      {item ? (
        <div>
        <p><strong>property:</strong> {item.property}</p>
        <p><strong>sumInsured:</strong> {item.sumInsured}</p>
        <p><strong>validTo:</strong> {item.validTo}</p>
        </div>
      ) : (
        <p>No cover on file.</p>
      )}
    </main>
  );
}
