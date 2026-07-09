import Link from "next/link";
import { fetchIrdData } from "../actions";

export const metadata = { title: "Tax Summary — IRD — My Gov NZ" };

export default async function TaxSummaryPage() {
  const data = await fetchIrdData(["ird:income", "ird:tax-summary"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>Tax Summary</h1>
        <p>Access to tax summary not granted.</p>
        <Link href="/consent?dept=ird&scope=ird:tax-summary">Grant access</Link>
      </main>
    );
  }

  const allYears = [data.currentTaxYear, ...(data.taxHistory ?? [])].filter(Boolean);

  return (
    <main>
      <Link href="/dept/ird">← Back to IRD</Link>
      <h1>Tax Summary</h1>

      {allYears.map((year) => (
        <section key={year.assessmentYear}>
          <h2>{year.assessmentYear} Tax Year</h2>
          <dl>
            <dt>Tax code</dt><dd>{year.taxCode}</dd>
            {year.employmentIncome && <><dt>Employment income</dt><dd>${Number(year.employmentIncome).toLocaleString()}</dd></>}
            {year.otherIncome && <><dt>Other income</dt><dd>${Number(year.otherIncome).toLocaleString()}</dd></>}
            <dt>Total income</dt><dd>${Number(year.totalIncome).toLocaleString()}</dd>
            <dt>Tax liability</dt><dd>${Number(year.taxLiability).toLocaleString()}</dd>
            <dt>Tax paid</dt><dd>${Number(year.taxPaid).toLocaleString()}</dd>
            {Number(year.taxRefundDue) > 0 && (
              <><dt>Refund due</dt><dd style={{ color: "green" }}>${Number(year.taxRefundDue).toLocaleString()}</dd></>
            )}
            {Number(year.taxOwing) > 0 && (
              <><dt>Amount owing</dt><dd style={{ color: "red" }}>${Number(year.taxOwing).toLocaleString()}</dd></>
            )}
            <dt>Assessment status</dt><dd>{year.assessmentStatus}</dd>
          </dl>
        </section>
      ))}
    </main>
  );
}
