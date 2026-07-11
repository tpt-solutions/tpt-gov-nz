import Link from "next/link";
import { fetchHudDataForCitizen } from "../actions";

export const metadata = { title: "Maintenance — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function HudStaffMaintenancePage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Maintenance — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchHudDataForCitizen(did, ["hud:maintenance"]);
  const requests = data?.maintenanceRequests;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/hud?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Maintenance — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load maintenance requests.</p>}
      {data && (!requests || requests.length === 0) && <p>No maintenance requests on record.</p>}

      {requests && requests.length > 0 && (
        <ul>
          {requests.map((m) => (
            <li key={m.requestNumber}>
              <strong>{m.requestNumber}</strong> — {m.category} ({m.status}): {m.description}
              <br />
              <small>Requested {m.requestedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
