import Link from "next/link";
import { fetchMohData } from "../actions";

export const metadata = { title: "Appointments — Health — My Gov NZ" };

export default async function MohAppointmentsPage() {
  const data = await fetchMohData(["moh:appointments"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/moh">← Back to Health</Link>
        <h1>Appointments</h1>
        <p>Unable to load your appointments.</p>
      </main>
    );
  }

  const appointments = data.upcomingAppointments ?? [];

  return (
    <main>
      <Link href="/dept/moh">← Back to Health</Link>
      <h1>Appointments</h1>

      {appointments.length === 0 && <p>You have no appointments booked.</p>}

      <ul>
        {appointments.map((a, i) => (
          <li key={i}>
            <strong>{a.type}</strong> with {a.provider}
            <br />
            {new Date(a.date).toLocaleString("en-NZ")} — {a.status}
          </li>
        ))}
      </ul>
    </main>
  );
}
