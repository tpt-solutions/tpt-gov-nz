import Link from "next/link";
import { fetchMohDataForCitizen } from "../actions";

export const metadata = { title: "Appointments — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MohStaffAppointmentsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Appointments — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMohDataForCitizen(did, ["moh:appointments"]);
  const appointments = data?.upcomingAppointments ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moh?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Appointments — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load appointments.</p>}
      {data && appointments.length === 0 && <p>No appointments.</p>}

      {data && (
        <ul>
          {appointments.map((a, i) => (
            <li key={i}>
              <strong>{a.type}</strong> with {a.provider} on{" "}
              {new Date(a.date).toLocaleString("en-NZ")} — {a.status}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
