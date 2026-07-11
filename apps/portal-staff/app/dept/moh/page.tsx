import Link from "next/link";
import { fetchMohDataForCitizen } from "./actions";
import { staffConsentForDept } from "../../lib/consent";

export const metadata = { title: "Health — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MohStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Health (Ministry of Health) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their health records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const consent = await staffConsentForDept(did, "moh");
  if (!consent.granted) {
    return (
      <main style={{ padding: "1rem" }}>
        <Link href="/citizens">← Back to citizen search</Link>
        <h1>Health (Ministry of Health) — Case File</h1>
        <p>
          <strong>Consent required.</strong> The citizen has not granted case-worker access to
          health records.
        </p>
      </main>
    );
  }

  const data = await fetchMohDataForCitizen(did, [
    "moh:gp",
    "moh:prescriptions",
    "moh:appointments",
    "moh:vaccinations",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Health (Ministry of Health) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load health information for this citizen.</p>}

      {data && (
        <>
          <p>NHI: ending in {data.nhiNumber.slice(-4)}</p>

          {data.enrolledGP && (
            <section>
              <h2>GP enrolment</h2>
              <p>
                <strong>{data.enrolledGP.practiceName}</strong>
                <br />
                {data.enrolledGP.address}
                <br />
                {data.enrolledGP.phone}
              </p>
            </section>
          )}

          <section>
            <h2>Prescriptions</h2>
            {!data.activePrescriptions || data.activePrescriptions.length === 0
              ? <p>No active prescriptions.</p>
              : (
                <ul>
                  {data.activePrescriptions.map((p, i) => (
                    <li key={i}>
                      {p.medication} ({p.dose}) — {p.repeatsRemaining} repeat(s)
                    </li>
                  ))}
                </ul>
              )}
          </section>

          <section>
            <h2>Appointments</h2>
            {!data.upcomingAppointments || data.upcomingAppointments.length === 0
              ? <p>No appointments.</p>
              : (
                <ul>
                  {data.upcomingAppointments.map((a, i) => (
                    <li key={i}>
                      {a.type} with {a.provider} on {new Date(a.date).toLocaleString("en-NZ")}
                    </li>
                  ))}
                </ul>
              )}
          </section>

          <nav>
            <Link href={`/dept/moh/prescriptions?did=${encodeURIComponent(did)}`}>Prescriptions</Link>
            {" · "}
            <Link href={`/dept/moh/appointments?did=${encodeURIComponent(did)}`}>Appointments</Link>
            {" · "}
            <Link href={`/dept/moh/vaccinations?did=${encodeURIComponent(did)}`}>Vaccinations</Link>
          </nav>
        </>
      )}
    </main>
  );
}
