import Link from "next/link";
import { fetchMpiData } from "../actions";

export const metadata = { title: "MPI Registrations — My Gov NZ" };

export default async function MpiRegistrationsPage() {
  const data = await fetchMpiData(["mpi:registrations"]);
  const registrations = data?.registrations ?? [];

  return (
    <main>
      <Link href="/dept/mpi">← Back to MPI</Link>
      <h1>MPI Registrations</h1>

      {registrations.length === 0 ? (
        <p>No registrations on record.</p>
      ) : (
        <ul>
          {registrations.map((r) => (
            <li key={r.nzbn}>
              <strong>{r.businessName}</strong> — {r.type} ({r.status})
              <br />
              <small>NZBN {r.nzbn} · registered {r.registeredDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
