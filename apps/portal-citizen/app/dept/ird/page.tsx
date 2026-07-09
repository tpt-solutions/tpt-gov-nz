import Link from "next/link";
import { fetchIrdData } from "./actions";

export const metadata = { title: "Inland Revenue — My Gov NZ" };

export default async function IrdOverviewPage() {
  const data = await fetchIrdData(["ird:income", "ird:tax-summary", "ird:kiwisaver", "ird:wff"]);

  if (!data) {
    return (
      <main>
        <h1>Inland Revenue (IRD)</h1>
        <p>Unable to load your IRD information. Please grant access to continue.</p>
        <Link href="/consent?dept=ird">Grant IRD access</Link>
      </main>
    );
  }

  const tax = data.currentTaxYear;
  const ks = data.kiwiSaver;
  const wff = data.workingForFamilies;

  return (
    <main>
      <h1>Inland Revenue (IRD)</h1>
      <p>IRD number: ending in {data.irdNumber.slice(-3)}</p>

      {tax && (
        <section>
          <h2>Tax {tax.assessmentYear}</h2>
          <dl>
            {tax.totalIncome && <><dt>Total income</dt><dd>${Number(tax.totalIncome).toLocaleString()}</dd></>}
            {tax.taxPaid && <><dt>Tax paid</dt><dd>${Number(tax.taxPaid).toLocaleString()}</dd></>}
            {Number(tax.taxRefundDue) > 0 && <><dt>Refund due</dt><dd style={{ color: "green" }}>${Number(tax.taxRefundDue).toLocaleString()}</dd></>}
            {Number(tax.taxOwing) > 0 && <><dt>Tax owing</dt><dd style={{ color: "red" }}>${Number(tax.taxOwing).toLocaleString()}</dd></>}
            <dt>Status</dt><dd>{tax.assessmentStatus}</dd>
          </dl>
          <Link href="/dept/ird/tax-summary">View full tax summary →</Link>
        </section>
      )}

      {wff?.eligible && (
        <section>
          <h2>Working for Families</h2>
          <p>You are receiving Working for Families payments.</p>
          {wff.currentEntitlement && (
            <p>Weekly entitlement: <strong>${Number(wff.currentEntitlement.totalWeeklyEntitlement ?? 0).toFixed(2)}</strong></p>
          )}
          <Link href="/dept/ird/working-for-families">View details →</Link>
        </section>
      )}

      {ks && (
        <section>
          <h2>KiwiSaver</h2>
          <dl>
            <dt>Status</dt><dd>{ks.membershipStatus}</dd>
            <dt>Contribution rate</dt><dd>{ks.contributionRate}%</dd>
            {ks.totalBalance && <><dt>Estimated balance</dt><dd>${Number(ks.totalBalance).toLocaleString()}</dd></>}
          </dl>
          <Link href="/dept/ird/kiwisaver">Manage KiwiSaver →</Link>
        </section>
      )}

      <nav>
        <Link href="/dept/ird/tax-summary">Tax Summary</Link>
        {" · "}
        <Link href="/dept/ird/working-for-families">Working for Families</Link>
        {" · "}
        <Link href="/dept/ird/kiwisaver">KiwiSaver</Link>
        {" · "}
        <Link href="/dept/ird/gst">GST</Link>
      </nav>
    </main>
  );
}
