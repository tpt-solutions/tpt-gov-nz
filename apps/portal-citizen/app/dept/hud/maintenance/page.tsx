import Link from "next/link";
import MaintenanceForm from "./form";
import { fetchHudData } from "../actions";

export const metadata = { title: "Maintenance — My Gov NZ" };

export default async function HudMaintenancePage() {
  const data = await fetchHudData(["hud:maintenance"]);
  const requests = data?.maintenanceRequests ?? [];

  return (
    <main>
      <Link href="/dept/hud">← Back to housing</Link>
      <h1>Maintenance</h1>

      <section>
        <h2>Your requests</h2>
        {requests.length === 0 ? (
          <p>No maintenance requests on record.</p>
        ) : (
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
      </section>

      <section>
        <h2>Log a new request</h2>
        <MaintenanceForm />
      </section>
    </main>
  );
}
