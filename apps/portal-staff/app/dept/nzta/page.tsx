import Link from "next/link";
import { fetchNztaDataForCitizen } from "./actions";

export const metadata = { title: "Transport — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NztaStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Waka Kotahi NZ Transport Agency (NZTA) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their NZTA records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchNztaDataForCitizen(did, [
    "nzta:driver-licence",
    "nzta:vehicles",
    "nzta:ruc",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Waka Kotahi NZ Transport Agency (NZTA) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load NZTA information for this citizen.</p>}

      {data && (
        <>
          <p>Driver licence: ending in {data.driverLicenceNumber.slice(-4)}</p>

          <section>
            <h2>Driver licence</h2>
            {data.driverLicence ? (
              <dl>
                <dt>Licence number</dt>
                <dd>••••{data.driverLicence.licenceNumber.slice(-4)}</dd>
                <dt>Class</dt>
                <dd>{data.driverLicence.licenceClass}</dd>
                <dt>Expiry</dt>
                <dd>{data.driverLicence.expiryDate}</dd>
              </dl>
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
                    <strong>{v.registration}</strong> — {v.make} {v.model} ({v.year}),{" "}
                    {v.fuelType}. Expires {v.registrationExpiry}.
                  </li>
                ))}
              </ul>
            ) : (
              <p>No vehicles registered.</p>
            )}
          </section>

          <section>
            <h2>Road user charges (RUC)</h2>
            {data.ruc && data.ruc.length > 0 ? (
              <ul>
                {data.ruc.map((r) => (
                  <li key={r.vehicleRego}>
                    <strong>{r.vehicleRego}</strong> — {r.licenceType}, {r.unitsRemaining}{" "}
                    units left, expires {r.expiryDate}.
                  </li>
                ))}
              </ul>
            ) : (
              <p>No RUC licences on record.</p>
            )}
          </section>
        </>
      )}
    </main>
  );
}
