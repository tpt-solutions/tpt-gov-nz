import Link from "next/link";
import ApplicationForm from "./form";
import { fetchHudData } from "../actions";

export const metadata = { title: "Housing Applications — My Gov NZ" };

export default async function HudApplicationsPage() {
  const data = await fetchHudData(["hud:applications"]);
  const applications = data?.applications ?? [];

  return (
    <main>
      <Link href="/dept/hud">← Back to housing</Link>
      <h1>Housing Applications</h1>

      <section>
        <h2>Your applications</h2>
        {applications.length === 0 ? (
          <p>No housing applications on record.</p>
        ) : (
          <ul>
            {applications.map((a) => (
              <li key={a.applicationNumber}>
                <strong>{a.applicationNumber}</strong> — {a.applicationType} ({a.status})
                {a.priorityBand ? ` — priority band ${a.priorityBand}` : ""}
                <br />
                <small>Submitted {a.submittedDate}{a.bedroomsNeeded != null ? ` · ${a.bedroomsNeeded} bedrooms needed` : ""}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Submit a new application</h2>
        <ApplicationForm />
      </section>
    </main>
  );
}
