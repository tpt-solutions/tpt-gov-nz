import Link from "next/link";
import { fetchDiaDataForCitizen } from "./actions";
import { staffConsentForDept } from "../../lib/consent";

export const metadata = { title: "Internal Affairs — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DiaStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Internal Affairs (DIA) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their DIA records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const consent = await staffConsentForDept(did, "dia");
  if (!consent.granted) {
    return (
      <main style={{ padding: "1rem" }}>
        <Link href="/citizens">← Back to citizen search</Link>
        <h1>Internal Affairs (DIA) — Case File</h1>
        <p>
          <strong>Consent required.</strong> The citizen has not granted case-worker access to
          Internal Affairs records.
        </p>
      </main>
    );
  }

  const data = await fetchDiaDataForCitizen(did, [
    "dia:passport",
    "dia:documents",
    "dia:citizenship",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Internal Affairs (DIA) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Internal Affairs information for this citizen.</p>}

      {data && (
        <>
          <p>Passport: ending in {data.passportNumber.slice(-4)}</p>

          <section>
            <h2>Passport</h2>
            {data.passport ? (
              <dl>
                <dt>Expiry</dt>
                <dd>{data.passport.expiryDate}</dd>
                <dt>Renewable</dt>
                <dd>{data.passport.renewable ? "Yes" : "No"}</dd>
              </dl>
            ) : (
              <p>No passport on record.</p>
            )}
          </section>

          <section>
            <h2>Citizenship</h2>
            {data.citizenship ? (
              <p>Status: <strong>{data.citizenship.status}</strong></p>
            ) : (
              <p>No citizenship record.</p>
            )}
          </section>

          <section>
            <h2>Documents</h2>
            {data.birthCertificate ? (
              <p>
                Birth certificate {data.birthCertificate.certificateNumber} — born{" "}
                {data.birthCertificate.dateOfBirth} in {data.birthCertificate.placeOfBirth}
              </p>
            ) : (
              <p>No birth certificate on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/dia/passports?did=${encodeURIComponent(did)}`}>Passport</Link>
            {" · "}
            <Link href={`/dept/dia/documents?did=${encodeURIComponent(did)}`}>Documents</Link>
          </nav>
        </>
      )}
    </main>
  );
}
