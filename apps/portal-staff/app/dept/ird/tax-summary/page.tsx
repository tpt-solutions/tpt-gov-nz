import Link from "next/link";
import { fetchIrdDataForCitizen } from "../actions";

export const metadata = { title: "Tax Summary — IRD Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StaffTaxSummaryPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  const data = did
    ? await fetchIrdDataForCitizen(did, ["ird:income", "ird:tax-summary"])
    : null;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ird?did=${did ? encodeURIComponent(did) : ""}`}>← Back to IRD case file</Link>
      <h1>Tax Summary</h1>
      <p><em>Read-only case worker view.</em></p>

      {!data && <p>No tax summary available for this citizen.</p>}

      {data &&
        [data.currentTaxYear, ...(data.taxHistory ?? [])]
          .filter(Boolean)
          .map((year) => (
            <section key={year.assessmentYear}>
              <h2>{year.assessmentYear} Tax Year</h2>
              <dl>
                <dt>Tax code</dt>
                <dd>{year.taxCode}</dd>
                {year.employmentIncome && (
                  <>
                    <dt>Employment income</dt>
                    <dd>${Number(year.employmentIncome).toLocaleString()}</dd>
                  </>
                )}
                {year.otherIncome && (
                  <>
                    <dt>Other income</dt>
                    <dd>${Number(year.otherIncome).toLocaleString()}</dd>
                  </>
                )}
                <dt>Total income</dt>
                <dd>${Number(year.totalIncome).toLocaleString()}</dd>
                <dt>Tax liability</dt>
                <dd>${Number(year.taxLiability).toLocaleString()}</dd>
                <dt>Tax paid</dt>
                <dd>${Number(year.taxPaid).toLocaleString()}</dd>
                {Number(year.taxRefundDue) > 0 && (
                  <>
                    <dt>Refund due</dt>
                    <dd>${Number(year.taxRefundDue).toLocaleString()}</dd>
                  </>
                )}
                {Number(year.taxOwing) > 0 && (
                  <>
                    <dt>Amount owing</dt>
                    <dd>${Number(year.taxOwing).toLocaleString()}</dd>
                  </>
                )}
                <dt>Assessment status</dt>
                <dd>{year.assessmentStatus}</dd>
              </dl>
            </section>
          ))}
    </main>
  );
}
