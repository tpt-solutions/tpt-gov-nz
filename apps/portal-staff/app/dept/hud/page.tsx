import Link from "next/link";
import { fetchHudDataForCitizen } from "./actions";

export const metadata = { title: "Housing — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function HudStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Housing — Case File</h1>
        <p>No citizen selected. Enter a DID to view their housing records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchHudDataForCitizen(did, ["hud:applications", "hud:tenancy", "hud:maintenance"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Housing and Urban Development / Kāinga Ora — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load housing information for this citizen.</p>}

      {data && (
        <>
          <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

          <section>
            <h2>Applications</h2>
            {data.applications && data.applications.length > 0 ? (
              <ul>
                {data.applications.map((a) => (
                  <li key={a.applicationNumber}>
                    <strong>{a.applicationNumber}</strong> — {a.applicationType} ({a.status})
                  </li>
                ))}
              </ul>
            ) : (
              <p>No housing applications on record.</p>
            )}
          </section>

          <section>
            <h2>Tenancy</h2>
            {data.tenancies && data.tenancies.length > 0 ? (
              <ul>
                {data.tenancies.map((t) => (
                  <li key={t.tenancyId}>
                    <strong>{t.propertyAddress}</strong> — {t.status}, ${t.weeklyRent}/week
                  </li>
                ))}
              </ul>
            ) : (
              <p>No tenancy on record.</p>
            )}
          </section>

          <section>
            <h2>Maintenance</h2>
            {data.maintenanceRequests && data.maintenanceRequests.length > 0 ? (
              <ul>
                {data.maintenanceRequests.map((m) => (
                  <li key={m.requestNumber}>
                    <strong>{m.requestNumber}</strong> — {m.category} ({m.status}): {m.description}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No maintenance requests on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/hud/applications?did=${encodeURIComponent(did)}`}>Applications</Link>
            {" · "}
            <Link href={`/dept/hud/tenancy?did=${encodeURIComponent(did)}`}>Tenancy</Link>
            {" · "}
            <Link href={`/dept/hud/maintenance?did=${encodeURIComponent(did)}`}>Maintenance</Link>
          </nav>
        </>
      )}
    </main>
  );
}
