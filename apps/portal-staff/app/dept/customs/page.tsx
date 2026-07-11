import Link from "next/link";
import { fetchCustomsDataForCitizen } from "./actions";

export const metadata = { title: "Customs — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CustomsStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Customs — Case File</h1>
        <p>No citizen selected. Enter a DID to view their customs records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchCustomsDataForCitizen(did, ["customs:travel", "customs:declarations"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>New Zealand Customs Service — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load customs information for this citizen.</p>}

      {data && (
        <>
          <p>Traveller id: ••••{data.travellerId.slice(-4)}</p>

          <section>
            <h2>Travel</h2>
            {data.travel ? (
              <ul>
                <li>
                  <strong>Passport {data.travel.passportNumber}</strong> — last arrival {data.travel.lastArrival} at {data.travel.arrivalPort}
                  <br />
                  <small>
                    {data.travel.frequentTraveller ? "Frequent traveller" : "Not a frequent traveller"}
                  </small>
                </li>
              </ul>
            ) : (
              <p>No travel record on file.</p>
            )}
          </section>

          <section>
            <h2>Declarations</h2>
            {data.declarations && data.declarations.length > 0 ? (
              <ul>
                {data.declarations.map((d) => (
                  <li key={d.declarationId}>
                    <strong>{d.declarationId}</strong> — from {d.countryFrom} ({d.status}): {d.goodsDeclared}
                    <br />
                    <small>Declared {d.date}</small>
                  </li>
                ))}
              </ul>
            ) : (
              <p>No declarations on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/customs/travel?did=${encodeURIComponent(did)}`}>Travel</Link>
            {" · "}
            <Link href={`/dept/customs/declarations?did=${encodeURIComponent(did)}`}>Declarations</Link>
          </nav>
        </>
      )}
    </main>
  );
}
