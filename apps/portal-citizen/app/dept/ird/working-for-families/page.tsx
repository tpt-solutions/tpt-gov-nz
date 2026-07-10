import Link from "next/link";
import { fetchIrdData } from "../actions";
import { getAiLevel } from "../../../ai/level";
import WffAiPrompt from "./ai-prompt";

export const metadata = { title: "Working for Families — IRD — My Gov NZ" };

export default async function WorkingForFamiliesPage() {
  const data = await fetchIrdData(["ird:wff"]);
  const aiLevel = getAiLevel();

  if (!data) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>Working for Families</h1>
        <p>Access to Working for Families information not granted.</p>
        <Link href="/consent?dept=ird&scope=ird:wff">Grant access</Link>
      </main>
    );
  }

  const wff = data.workingForFamilies;

  if (!wff) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>Working for Families</h1>
        <p>No Working for Families data available for your account.</p>
      </main>
    );
  }

  if (!wff.eligible) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>Working for Families</h1>
        <section>
          <h2>Eligibility</h2>
          <p>You are not currently eligible for Working for Families.</p>
          <dl>
            <dt>Current income</dt>
            <dd>${Number(wff.currentIncome).toLocaleString()}</dd>
            <dt>Income threshold</dt>
            <dd>${Number(wff.incomeThreshold).toLocaleString()}</dd>
          </dl>
          <p>If your circumstances change, you can reapply.</p>
        </section>
      </main>
    );
  }

  const entitlement = wff.currentEntitlement;

  return (
    <main>
      <Link href="/dept/ird">← Back to IRD</Link>
      <h1>Working for Families</h1>

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

      {entitlement && (
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
                <td>${Number(entitlement.familyTaxCredit).toFixed(2)}</td>
              </tr>
              {entitlement.inWorkTaxCredit != null && (
                <tr>
                  <td>In-Work Tax Credit</td>
                  <td>${Number(entitlement.inWorkTaxCredit).toFixed(2)}</td>
                </tr>
              )}
              {entitlement.bestStartPayment != null && (
                <tr>
                  <td>Best Start Payment</td>
                  <td>${Number(entitlement.bestStartPayment).toFixed(2)}</td>
                </tr>
              )}
              {entitlement.minimumFamilyTaxCredit != null && (
                <tr>
                  <td>Minimum Family Tax Credit</td>
                  <td>${Number(entitlement.minimumFamilyTaxCredit).toFixed(2)}</td>
                </tr>
              )}
              <tr style={{ fontWeight: "bold" }}>
                <td>Total weekly entitlement</td>
                <td>${Number(entitlement.totalWeeklyEntitlement).toFixed(2)}</td>
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

      <section style={{ marginTop: "1.5rem" }}>
        <h2>Life events</h2>
        <p>
          <Link href="/dept/ird/working-for-families/wizard">
            I just had a baby — check my eligibility →
          </Link>
        </p>
      </section>

      {aiLevel !== "none" && <WffAiPrompt />}
    </main>
  );
}
