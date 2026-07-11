import Link from "next/link";
import { fetchLinzDataForCitizen } from "./actions";

export const metadata = { title: "Toitū Te Whenua Land Information New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function LinzStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Toitū Te Whenua Land Information New Zealand — Case File</h1>
        <p>No citizen selected. Enter a DID to view their land records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchLinzDataForCitizen(did, ["linz:titles", "linz:ownership"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Toitū Te Whenua Land Information New Zealand — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load land information for this citizen.</p>}

      {data && (
        <>
          <p>Customer ID: ••••{data.customerId.slice(-4)}</p>

          <section>
            <h2>Titles</h2>
            {data.titles && data.titles.length > 0 ? (
              <ul>
                {data.titles.map((t) => (
                  <li key={t.titleNumber}>
                    <strong>{t.titleNumber}</strong> — {t.estateType}
                    <br />
                    <small>{t.propertyAddress} · {t.landAreaSqm} m²</small>
                  </li>
                ))}
              </ul>
            ) : (
              <p>No property titles on record.</p>
            )}
          </section>

          <section>
            <h2>Ownership</h2>
            {data.ownership && data.ownership.length > 0 ? (
              <ul>
                {data.ownership.map((o) => (
                  <li key={o.titleNumber}>
                    <strong>{o.titleNumber}</strong> — {o.ownershipShare}
                    <br />
                    <small>Registered owners: {o.registeredOwners.join(", ")}</small>
                  </li>
                ))}
              </ul>
            ) : (
              <p>No ownership records on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/linz/titles?did=${encodeURIComponent(did)}`}>Titles</Link>
            {" · "}
            <Link href={`/dept/linz/ownership?did=${encodeURIComponent(did)}`}>Ownership</Link>
          </nav>
        </>
      )}
    </main>
  );
}
