import Link from "next/link";
import { fetchHudDataForCitizen } from "../actions";

export const metadata = { title: "Applications — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function HudStaffApplicationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Applications — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchHudDataForCitizen(did, ["hud:applications"]);
  const applications = data?.applications;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/hud?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Applications — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load applications.</p>}
      {data && (!applications || applications.length === 0) && <p>No housing applications on record.</p>}

      {applications && applications.length > 0 && (
        <ul>
          {applications.map((a) => (
            <li key={a.applicationNumber}>
              <strong>{a.applicationNumber}</strong> — {a.applicationType} ({a.status})
              {a.priorityBand ? ` — priority band ${a.priorityBand}` : ""}
              <br />
              <small>Submitted {a.submittedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
