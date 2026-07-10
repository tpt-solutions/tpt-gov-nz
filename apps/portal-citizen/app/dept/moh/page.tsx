import Link from "next/link";
import { fetchMohData } from "./actions";
import MohAiPrompt from "./ai-prompt";

export const metadata = { title: "Health — My Gov NZ" };

export default async function MohOverviewPage() {
  const data = await fetchMohData([
    "moh:gp",
    "moh:prescriptions",
    "moh:appointments",
    "moh:vaccinations",
  ]);

  if (!data) {
    return (
      <main>
        <h1>Health (Ministry of Health)</h1>
        <p>Unable to load your health information. Please grant access to continue.</p>
        <Link href="/consent?dept=moh">Grant Health access</Link>
      </main>
    );
  }

  const gp = data.enrolledGP;
  const prescriptions = data.activePrescriptions ?? [];
  const appointments = data.upcomingAppointments ?? [];
  const vaccinations = data.vaccinations ?? [];
  const boosterDue = vaccinations.filter((v) => v.dueForBooster);

  return (
    <main>
      <h1>Health (Ministry of Health)</h1>
      <p>NHI: ending in {data.nhiNumber.slice(-4)}</p>

      {gp && (
        <section>
          <h2>Your GP</h2>
          <p>
            <strong>{gp.practiceName}</strong>
            <br />
            {gp.address}
            <br />
            {gp.phone}
          </p>
          <Link href="/dept/moh/prescriptions">View prescriptions →</Link>
        </section>
      )}

      <section>
        <h2>Prescriptions</h2>
        {prescriptions.length === 0 && <p>No active prescriptions.</p>}
        <ul>
          {prescriptions.map((p, i) => (
            <li key={i}>
              {p.medication} ({p.dose}) — {p.repeatsRemaining} repeat(s) left
            </li>
          ))}
        </ul>
        <Link href="/dept/moh/prescriptions">Manage prescriptions →</Link>
      </section>

      <section>
        <h2>Appointments</h2>
        {appointments.length === 0 && <p>No upcoming appointments.</p>}
        <ul>
          {appointments.map((a, i) => (
            <li key={i}>
              {a.type} with {a.provider} on {new Date(a.date).toLocaleString("en-NZ")}
            </li>
          ))}
        </ul>
        <Link href="/dept/moh/appointments">View appointments →</Link>
      </section>

      <section>
        <h2>Vaccinations</h2>
        {vaccinations.length === 0 && <p>No vaccination records.</p>}
        <ul>
          {vaccinations.map((v, i) => (
            <li key={i}>
              {v.vaccine} ({v.date})
              {v.dueForBooster && <strong style={{ color: "darkorange" }}> — booster due</strong>}
            </li>
          ))}
        </ul>
        {boosterDue.length > 0 && (
          <p style={{ color: "darkorange" }}>
            You have {boosterDue.length} vaccination booster(s) due. Speak to your GP or pharmacist.
          </p>
        )}
        <Link href="/dept/moh/vaccinations">View vaccinations →</Link>
      </section>

      <MohAiPrompt />
    </main>
  );
}
