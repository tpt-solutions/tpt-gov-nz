import Link from "next/link";
import { fetchIrdDataForCitizen } from "../actions";

export const metadata = { title: "Working for Families — IRD Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StaffWorkingForFamiliesPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  const data = did ? await fetchIrdDataForCitizen(did, ["ird:wff"]) : null;
  const wff = data?.workingForFamilies;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ird?did=${did ? encodeURIComponent(did) : ""}`}>← Back to IRD case file</Link>
      <h1>Working for Families</h1>
      <p><em>Read-only case worker view.</em></p>

      {!wff && <p>No Working for Families data available for this citizen.</p>}

      {wff && !wff.eligible && (
        <section>
          <h2>Eligibility</h2>
          <p>Citizen is not currently eligible for Working for Families.</p>
          <dl>
            <dt>Current income</dt>
            <dd>${Number(wff.currentIncome).toLocaleString()}</dd>
            <dt>Income threshold</dt>
            <dd>${Number(wff.incomeThreshold).toLocaleString()}</dd>
          </dl>
        </section>
      )}

      {wff?.eligible && (
        <>
          <section>
            <h2>Eligibility</h2>
            <dl>
              <dt>Status</dt>
              <dd>Eligible</dd>
              <dt>Dependant children</dt>
              <dd>{wff.numberOfDependantChildren}</dd>
              <dt>Current income</dt>
              <dd>${Number(wff.currentIncome).toLocaleString()}</dd>
              <dt>Income threshold</dt>
              <dd>${Number(wff.incomeThreshold).toLocaleString()}</dd>
              {wff.nextReviewDate && (
                <>
                  <dt>Next review</dt>
                  <dd>{wff.nextReviewDate}</dd>
                </>
              )}
            </dl>
          </section>

          {wff.currentEntitlement && (
            <section>
              <h2>Entitlement breakdown</h2>
              <table>
                <thead>
                  <tr>
                    <th>Component</th>
                    <th>Weekly amount</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Family Tax Credit</td>
                    <td>${Number(wff.currentEntitlement.familyTaxCredit).toFixed(2)}</td>
                  </tr>
                  {wff.currentEntitlement.inWorkTaxCredit != null && (
                    <tr>
                      <td>In-Work Tax Credit</td>
                      <td>${Number(wff.currentEntitlement.inWorkTaxCredit).toFixed(2)}</td>
                    </tr>
                  )}
                  {wff.currentEntitlement.bestStartPayment != null && (
                    <tr>
                      <td>Best Start Payment</td>
                      <td>${Number(wff.currentEntitlement.bestStartPayment).toFixed(2)}</td>
                    </tr>
                  )}
                  {wff.currentEntitlement.minimumFamilyTaxCredit != null && (
                    <tr>
                      <td>Minimum Family Tax Credit</td>
                      <td>${Number(wff.currentEntitlement.minimumFamilyTaxCredit).toFixed(2)}</td>
                    </tr>
                  )}
                  <tr style={{ fontWeight: "bold" }}>
                    <td>Total weekly entitlement</td>
                    <td>${Number(wff.currentEntitlement.totalWeeklyEntitlement).toFixed(2)}</td>
                  </tr>
                </tbody>
              </table>
            </section>
          )}

          {wff.paymentFrequency && (
            <section>
              <h2>Payment details</h2>
              <dl>
                <dt>Payment frequency</dt>
                <dd>{wff.paymentFrequency}</dd>
              </dl>
            </section>
          )}
        </>
      )}
    </main>
  );
}
