import Link from "next/link";
import { fetchDiaData } from "./actions";
import DiaAiPrompt from "./ai-prompt";

export const metadata = { title: "Internal Affairs — My Gov NZ" };

export default async function DiaOverviewPage() {
  const data = await fetchDiaData(["dia:passport", "dia:documents", "dia:citizenship"]);

  if (!data) {
    return (
      <main>
        <h1>Internal Affairs (DIA)</h1>
        <p>Unable to load your Internal Affairs information. Please grant access to continue.</p>
        <Link href="/consent?dept=dia">Grant Internal Affairs access</Link>
      </main>
    );
  }

  const passport = data.passport;
  const expiringSoon =
    passport &&
    new Date(passport.expiryDate).getTime() - Date.now() < 1000 * 60 * 60 * 24 * 365;

  return (
    <main>
      <h1>Internal Affairs (DIA)</h1>
      <p>Passport: ending in {data.passportNumber.slice(-4)}</p>

      <section>
        <h2>Passport</h2>
        {passport ? (
          <>
            <dl>
              <dt>Passport number</dt>
              <dd>••••{passport.passportNumber.slice(-4)}</dd>
              <dt>Expiry</dt>
              <dd>{passport.expiryDate}</dd>
              <dt>Renewable</dt>
              <dd>{passport.renewable ? "Yes" : "No"}</dd>
            </dl>
            {expiringSoon && (
              <p style={{ color: "darkorange" }}>
                Your passport expires soon. You can renew it online.
              </p>
            )}
            <Link href="/dept/dia/passport-renewal">Renew passport →</Link>
          </>
        ) : (
          <p>No passport on record.</p>
        )}
      </section>

      <section>
        <h2>Citizenship</h2>
        {data.citizenship ? (
          <p>
            Status: <strong>{data.citizenship.status}</strong>
            {data.citizenship.certificateNumber
              ? ` (cert. ${data.citizenship.certificateNumber})`
              : ""}
          </p>
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
        <Link href="/dept/dia/birth-certificate">Request a birth certificate →</Link>
      </section>

      <DiaAiPrompt />
    </main>
  );
}
