import Link from "next/link";
import { fetchFenzData } from "../actions";

export const metadata = { title: "Fire safety — Fire and Emergency New Zealand — My Gov NZ" };

export default async function FenzFireSafetyPage() {
  const data = await fetchFenzData(["fenz:fire-safety"]);
  if (!data) {
    return (
      <main>
        <h1>Fire safety</h1>
        <p>Unable to load your FENZ information.</p>
        <Link href={"/consent?dept=fenz"}>Grant FENZ access</Link>
      </main>
    );
  }

  const item = data.fire_safety;

  return (
    <main>
      <Link href={"/dept/fenz"}>← Back to FENZ</Link>
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
