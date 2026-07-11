import Link from "next/link";
import { fetchAccDataForCitizen } from "./actions";
import { staffConsentForDept } from "../../lib/consent";

export const metadata = { title: "ACC — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function AccStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Accident Compensation Corporation (ACC) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their ACC records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const consent = await staffConsentForDept(did, "acc");
  if (!consent.granted) {
    return (
      <main style={{ padding: "1rem" }}>
        <Link href="/citizens">← Back to citizen search</Link>
        <h1>Accident Compensation Corporation (ACC) — Case File</h1>
        <p>
          <strong>Consent required.</strong> The citizen has not granted case-worker access to ACC
          records.
        </p>
      </main>
    );
  }

  const data = await fetchAccDataForCitizen(did, [
    "acc:claims",
    "acc:entitlements",
    "acc:rehabilitation",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Accident Compensation Corporation (ACC) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load ACC information for this citizen.</p>}

      {data && (
        <>
          <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

          <section>
            <h2>Claims</h2>
            {data.claims && data.claims.length > 0 ? (
              <ul>
                {data.claims.map((c) => (
                  <li key={c.claimNumber}>
                    <strong>{c.claimNumber}</strong> — {c.claimType} ({c.status}): {c.description}
                    {c.weeklyCompensation != null ? ` — $${c.weeklyCompensation}/week` : ""}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No claims on record.</p>
            )}
          </section>

          <section>
            <h2>Entitlements</h2>
            {data.entitlements ? (
              <p>
                {data.entitlements.hasEntitlement
                  ? `Active entitlement${data.entitlements.type ? `: ${data.entitlements.type}` : ""}${
                      data.entitlements.weeklyAmount != null
                        ? `, $${data.entitlements.weeklyAmount}/week`
                        : ""
                    }`
                  : "No active entitlement."}
              </p>
            ) : (
              <p>No entitlement information.</p>
            )}
          </section>

          <section>
            <h2>Rehabilitation</h2>
            {data.rehabilitation && data.rehabilitation.length > 0 ? (
              <ul>
                {data.rehabilitation.map((r) => (
                  <li key={r.planId}>
                    <strong>{r.planId}</strong> — {r.status}: {r.description}
                    {r.provider ? ` (provider: ${r.provider})` : ""}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No rehabilitation plans on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/acc/claims?did=${encodeURIComponent(did)}`}>Claims</Link>
            {" · "}
            <Link href={`/dept/acc/entitlements?did=${encodeURIComponent(did)}`}>Entitlements</Link>
            {" · "}
            <Link href={`/dept/acc/rehabilitation?did=${encodeURIComponent(did)}`}>Rehabilitation</Link>
          </nav>
        </>
      )}
    </main>
  );
}
