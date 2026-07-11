import Link from "next/link";
import { fetchRetirementData } from "../actions";

export const metadata = { title: "Retirement plan — Retirement Commission (Te Ara Ahunga Ora) — My Gov NZ" };

export default async function RetirementRetirementPlanPage() {
  const data = await fetchRetirementData(["retirement:retirement-plan"]);
  if (!data) {
    return (
      <main>
        <h1>Retirement plan</h1>
        <p>Unable to load your Retirement information.</p>
        <Link href={"/consent?dept=retirement"}>Grant Retirement access</Link>
      </main>
    );
  }

  const item = data.retirement_plan;

  return (
    <main>
      <Link href={"/dept/retirement"}>← Back to Retirement</Link>
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
