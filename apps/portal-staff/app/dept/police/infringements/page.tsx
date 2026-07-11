import Link from "next/link";
import { fetchPoliceDataForCitizen } from "../actions";

export const metadata = { title: "Infringements — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PoliceStaffInfringementsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Infringements — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchPoliceDataForCitizen(did, ["police:infringements"]);
  const infringements = data?.infringements;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/police?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Infringements — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load infringements.</p>}
      {data && (!infringements || infringements.length === 0) && <p>No infringements on record.</p>}

      {infringements && infringements.length > 0 && (
        <ul>
          {infringements.map((i) => (
            <li key={i.ticketNumber}>
              <strong>{i.ticketNumber}</strong> — {i.offenseType} ({i.status}): ${i.amount}
              <br />
              <small>Issued {i.issueDate}{i.location ? ` at ${i.location}` : ""}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
