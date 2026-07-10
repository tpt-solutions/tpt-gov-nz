import Link from "next/link";
import { fetchMohDataForCitizen } from "../actions";

export const metadata = { title: "Prescriptions — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MohStaffPrescriptionsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Prescriptions — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMohDataForCitizen(did, ["moh:prescriptions"]);
  const prescriptions = data?.activePrescriptions ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moh?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Prescriptions — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load prescriptions.</p>}
      {data && prescriptions.length === 0 && <p>No active prescriptions.</p>}

      {data && prescriptions.map((p, i) => (
        <section key={i} style={{ marginBottom: "1rem" }}>
          <h2>{p.medication}</h2>
          <dl>
            <dt>Dose</dt>
            <dd>{p.dose}</dd>
            <dt>Repeats remaining</dt>
            <dd>{p.repeatsRemaining}</dd>
            <dt>Issued</dt>
            <dd>{p.issuedAt}</dd>
          </dl>
        </section>
      ))}
    </main>
  );
}
