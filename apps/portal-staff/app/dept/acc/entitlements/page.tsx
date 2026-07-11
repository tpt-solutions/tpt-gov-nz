import Link from "next/link";
import { fetchAccDataForCitizen } from "../actions";

export const metadata = { title: "ACC Entitlements — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function AccStaffEntitlementsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>ACC Entitlements — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchAccDataForCitizen(did, ["acc:entitlements"]);
  const e = data?.entitlements;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/acc?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>ACC Entitlements — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load entitlements.</p>}
      {data && !e && <p>No entitlement information.</p>}

      {e && (
        <dl>
          <dt>Has entitlement</dt>
          <dd>{e.hasEntitlement ? "Yes" : "No"}</dd>
          {e.type && (
            <>
              <dt>Type</dt>
              <dd>{e.type}</dd>
            </>
          )}
          {e.weeklyAmount != null && (
            <>
              <dt>Weekly amount</dt>
              <dd>${e.weeklyAmount}</dd>
            </>
          )}
          {e.remainingWeeks != null && (
            <>
              <dt>Remaining weeks</dt>
              <dd>{e.remainingWeeks}</dd>
            </>
          )}
        </dl>
      )}
    </main>
  );
}
