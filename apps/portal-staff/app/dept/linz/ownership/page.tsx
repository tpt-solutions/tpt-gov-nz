import Link from "next/link";
import { fetchLinzDataForCitizen } from "../actions";

export const metadata = { title: "Ownership — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function LinzStaffOwnershipPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ownership — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchLinzDataForCitizen(did, ["linz:ownership"]);
  const ownership = data?.ownership;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/linz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Ownership — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load ownership records.</p>}
      {data && (!ownership || ownership.length === 0) && <p>No ownership records on record.</p>}

      {ownership && ownership.length > 0 && (
        <ul>
          {ownership.map((o) => (
            <li key={o.titleNumber}>
              <strong>{o.titleNumber}</strong> — {o.ownershipShare}
              <br />
              <small>Registered owners: {o.registeredOwners.join(", ")}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
