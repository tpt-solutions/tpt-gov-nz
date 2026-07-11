import Link from "next/link";
import { STAFF_DEPARTMENTS, DEMO_CITIZEN_NAME, DEMO_CITIZEN_DID } from "./lib/config";

export const metadata = { title: "Case Worker Home — Staff — My Gov NZ" };

export default function StaffHomePage() {
  return (
    <main style={{ padding: "1rem" }}>
      <h1>Case Worker Home</h1>
      <p>
        Search for a citizen by their government DID or department-local identifier to open their
        cross-department case file. Data access is gated by the citizen&apos;s consent grants.
      </p>

      <p style={{ margin: "1rem 0" }}>
        <Link href="/citizens">→ Go to citizen search</Link>
      </p>

      <section style={{ border: "1px solid #ccc", padding: "1rem", margin: "1rem 0" }}>
        <h2>Quick open (demo citizen)</h2>
        <p>
          {DEMO_CITIZEN_NAME} — <code>{DEMO_CITIZEN_DID}</code>
        </p>
        <Link href={`/citizens/${encodeURIComponent(DEMO_CITIZEN_DID)}`}>
          Open case file
        </Link>
      </section>

      <section>
        <h2>Departments</h2>
        <ul>
          {STAFF_DEPARTMENTS.map((d) => (
            <li key={d.id}>
              <strong>{d.shortName}</strong> — {d.name}: {d.description}
            </li>
          ))}
        </ul>
      </section>
    </main>
  );
}
