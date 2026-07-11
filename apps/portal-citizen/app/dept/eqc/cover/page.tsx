import Link from "next/link";
import { fetchEqcData } from "../actions";

export const metadata = { title: "Cover — Earthquake Commission (Toka Tū Ake) — My Gov NZ" };

export default async function EqcCoverPage() {
  const data = await fetchEqcData(["eqc:cover"]);
  if (!data) {
    return (
      <main>
        <h1>Cover</h1>
        <p>Unable to load your EQC information.</p>
        <Link href={"/consent?dept=eqc"}>Grant EQC access</Link>
      </main>
    );
  }

  const item = data.cover;

  return (
    <main>
      <Link href={"/dept/eqc"}>← Back to EQC</Link>
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
