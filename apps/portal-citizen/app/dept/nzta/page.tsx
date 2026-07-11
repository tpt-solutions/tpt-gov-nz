import Link from "next/link";
import { fetchNztaData } from "./actions";
import NztaAiPrompt from "./ai-prompt";

export const metadata = { title: "Transport — My Gov NZ" };

export default async function NztaOverviewPage() {
  const data = await fetchNztaData(["nzta:driver-licence", "nzta:vehicles", "nzta:ruc"]);

  if (!data) {
    return (
      <main>
        <h1>Waka Kotahi NZ Transport Agency (NZTA)</h1>
        <p>Unable to load your transport information. Please grant access to continue.</p>
        <Link href="/consent?dept=nzta">Grant NZTA access</Link>
      </main>
    );
  }

  const licence = data.driverLicence;
  const licenceExpiringSoon =
    licence &&
    new Date(licence.expiryDate).getTime() - Date.now() < 1000 * 60 * 60 * 24 * 365;

  return (
    <main>
      <h1>Waka Kotahi NZ Transport Agency (NZTA)</h1>
      <p>Driver licence: ending in {data.driverLicenceNumber.slice(-4)}</p>

      <section>
        <h2>Driver licence</h2>
        {licence ? (
          <>
            <dl>
              <dt>Licence number</dt>
              <dd>••••{licence.licenceNumber.slice(-4)}</dd>
              <dt>Class</dt>
              <dd>{licence.licenceClass}</dd>
              <dt>Expiry</dt>
              <dd>{licence.expiryDate}</dd>
            </dl>
            {licenceExpiringSoon && (
              <p style={{ color: "darkorange" }}>
                Your licence expires soon. You can request a replacement card online.
              </p>
            )}
            <Link href="/dept/nzta/request-licence-replacement">Replace licence card →</Link>
          </>
        ) : (
          <p>No driver licence on record.</p>
        )}
      </section>

      <section>
        <h2>Vehicles</h2>
        {data.vehicles && data.vehicles.length > 0 ? (
          <ul>
            {data.vehicles.map((v) => (
              <li key={v.registration}>
                <strong>{v.registration}</strong> — {v.make} {v.model} ({v.year}), {v.fuelType}.
                Registration expires {v.registrationExpiry}.
              </li>
            ))}
          </ul>
        ) : (
          <p>No vehicles registered to you.</p>
        )}
        <Link href="/dept/nzta/renew-vehicle-registration">Renew vehicle registration →</Link>
      </section>

      <section>
        <h2>Road user charges (RUC)</h2>
        {data.ruc && data.ruc.length > 0 ? (
          <ul>
            {data.ruc.map((r) => (
              <li key={r.vehicleRego}>
                <strong>{r.vehicleRego}</strong> — {r.licenceType}, {r.unitsRemaining} units
                remaining, expires {r.expiryDate}.
              </li>
            ))}
          </ul>
        ) : (
          <p>No RUC licences on record.</p>
        )}
      </section>

      <NztaAiPrompt />
    </main>
  );
}
