import Link from "next/link";
import { fetchMohData } from "../actions";
import RepeatPrescriptionForm from "./RepeatForm";

export const metadata = { title: "Prescriptions — Health — My Gov NZ" };

export default async function MohPrescriptionsPage() {
  const data = await fetchMohData(["moh:prescriptions"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/moh">← Back to Health</Link>
        <h1>Prescriptions</h1>
        <p>Unable to load your prescriptions.</p>
      </main>
    );
  }

  const prescriptions = data.activePrescriptions ?? [];

  return (
    <main>
      <Link href="/dept/moh">← Back to Health</Link>
      <h1>Prescriptions</h1>

      {prescriptions.length === 0 && <p>You have no active prescriptions.</p>}

      {prescriptions.map((p, i) => (
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
          {p.repeatsRemaining > 0 ? (
            <RepeatPrescriptionForm prescriptionId={p.prescriptionId} medication={p.medication} />
          ) : (
            <p>No repeats remaining — contact your GP for a new prescription.</p>
          )}
        </section>
      ))}
    </main>
  );
}
