import Link from "next/link";
import { fetchLinzData } from "../actions";

export const metadata = { title: "Ownership — My Gov NZ" };

export default async function LinzOwnershipPage() {
  const data = await fetchLinzData(["linz:ownership"]);
  const ownership = data?.ownership ?? [];

  return (
    <main>
      <Link href="/dept/linz">← Back to LINZ</Link>
      <h1>Ownership</h1>

      {ownership.length === 0 ? (
        <p>No ownership records on record.</p>
      ) : (
        <ul>
          {ownership.map((o) => (
            <li key={o.titleNumber}>
              <strong>{o.titleNumber}</strong> — share {o.ownershipShare}
              <br />
              <small>Registered owners: {o.registeredOwners.join(", ")}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
