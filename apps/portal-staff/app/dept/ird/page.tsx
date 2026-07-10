import Link from "next/link";
import { fetchIrdDataForCitizen } from "./actions";

export const metadata = { title: "Inland Revenue — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function IrdStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Inland Revenue (IRD) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their IRD records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchIrdDataForCitizen(did, [
    "ird:income",
    "ird:tax-summary",
    "ird:kiwisaver",
    "ird:wff",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Inland Revenue (IRD) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load IRD information for this citizen.</p>}

      {data && (
        <>
          <p>IRD number: ending in {data.irdNumber.slice(-3)}</p>

          {data.currentTaxYear && (
            <section>
              <h2>Tax {data.currentTaxYear.assessmentYear}</h2>
              <dl>
                {data.currentTaxYear.totalIncome && (
                  <>
                    <dt>Total income</dt>
                    <dd>${Number(data.currentTaxYear.totalIncome).toLocaleString()}</dd>
                  </>
                )}
                {data.currentTaxYear.taxPaid && (
                  <>
                    <dt>Tax paid</dt>
                    <dd>${Number(data.currentTaxYear.taxPaid).toLocaleString()}</dd>
                  </>
                )}
                {Number(data.currentTaxYear.taxRefundDue) > 0 && (
                  <>
                    <dt>Refund due</dt>
                    <dd>${Number(data.currentTaxYear.taxRefundDue).toLocaleString()}</dd>
                  </>
                )}
                {Number(data.currentTaxYear.taxOwing) > 0 && (
                  <>
                    <dt>Tax owing</dt>
                    <dd>${Number(data.currentTaxYear.taxOwing).toLocaleString()}</dd>
                  </>
                )}
                <dt>Status</dt>
                <dd>{data.currentTaxYear.assessmentStatus}</dd>
              </dl>
            </section>
          )}

          {data.workingForFamilies?.eligible && (
            <section>
              <h2>Working for Families</h2>
              <p>Citizen is receiving Working for Families payments.</p>
              {data.workingForFamilies.currentEntitlement && (
                <p>
                  Weekly entitlement:{" "}
                  <strong>
                    $
                    {Number(
                      data.workingForFamilies.currentEntitlement.totalWeeklyEntitlement ?? 0,
                    ).toFixed(2)}
                  </strong>
                </p>
              )}
            </section>
          )}

          {data.kiwiSaver && (
            <section>
              <h2>KiwiSaver</h2>
              <dl>
                <dt>Status</dt>
                <dd>{data.kiwiSaver.membershipStatus}</dd>
                <dt>Contribution rate</dt>
                <dd>{data.kiwiSaver.contributionRate}%</dd>
                {data.kiwiSaver.totalBalance && (
                  <>
                    <dt>Estimated balance</dt>
                    <dd>${Number(data.kiwiSaver.totalBalance).toLocaleString()}</dd>
                  </>
                )}
              </dl>
            </section>
          )}

          <nav>
            <Link href={`/dept/ird/tax-summary?did=${encodeURIComponent(did)}`}>Tax Summary</Link>
            {" · "}
            <Link href={`/dept/ird/working-for-families?did=${encodeURIComponent(did)}`}>
              Working for Families
            </Link>
            {" · "}
            <Link href={`/dept/ird/kiwisaver?did=${encodeURIComponent(did)}`}>KiwiSaver</Link>
            {" · "}
            <Link href={`/dept/ird/gst?did=${encodeURIComponent(did)}`}>GST</Link>
          </nav>
        </>
      )}
    </main>
  );
}
