import Link from "next/link";
import { fetchMpiData } from "../actions";

export const metadata = { title: "MPI Certifications — My Gov NZ" };

export default async function MpiCertificationsPage() {
  const data = await fetchMpiData(["mpi:certifications"]);
  const certifications = data?.certifications ?? [];

  return (
    <main>
      <Link href="/dept/mpi">← Back to MPI</Link>
      <h1>MPI Certifications</h1>

      {certifications.length === 0 ? (
        <p>No certifications on record.</p>
      ) : (
        <ul>
          {certifications.map((c) => (
            <li key={c.certNumber}>
              <strong>{c.certNumber}</strong> — {c.category}
              <br />
              <small>Issued {c.issuedDate} · expires {c.expiresDate}</small>
            </li>
          ))}
        </ul>
      )}

      <section style={{ marginTop: "2rem" }}>
        <Link href="/dept/mpi/certifications/apply">Apply for an export certificate →</Link>
      </section>
    </main>
  );
}
