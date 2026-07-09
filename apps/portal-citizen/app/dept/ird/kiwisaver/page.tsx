import Link from "next/link";
import { fetchIrdData } from "../actions";

export const metadata = { title: "KiwiSaver — IRD — My Gov NZ" };

export default async function KiwiSaverPage() {
  const data = await fetchIrdData(["ird:kiwisaver"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>KiwiSaver</h1>
        <p>Access to KiwiSaver information not granted.</p>
        <Link href="/consent?dept=ird&scope=ird:kiwisaver">Grant access</Link>
      </main>
    );
  }

  const ks = data.kiwiSaver;

  if (!ks) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>KiwiSaver</h1>
        <p>No KiwiSaver data available for your account.</p>
      </main>
    );
  }

  return (
    <main>
      <Link href="/dept/ird">← Back to IRD</Link>
      <h1>KiwiSaver</h1>

      <section>
        <h2>Account overview</h2>
        <dl>
          <dt>Status</dt>
          <dd>{ks.membershipStatus}</dd>
          {ks.scheme && (
            <>
              <dt>Scheme</dt>
              <dd>{ks.scheme}</dd>
            </>
          )}
          {ks.totalBalance != null && (
            <>
              <dt>Estimated balance</dt>
              <dd>${Number(ks.totalBalance).toLocaleString()}</dd>
            </>
          )}
          {ks.lastContributionDate && (
            <>
              <dt>Last contribution</dt>
              <dd>{ks.lastContributionDate}</dd>
            </>
          )}
        </dl>
      </section>

      <section>
        <h2>Contribution rates</h2>
        <dl>
          <dt>Your contribution rate</dt>
          <dd>{ks.contributionRate}%</dd>
          {ks.employerContributionRate != null && (
            <>
              <dt>Employer contribution rate</dt>
              <dd>{ks.employerContributionRate}%</dd>
            </>
          )}
        </dl>
        <Link href="/dept/ird/kiwisaver/update-rate">Change contribution rate →</Link>
      </section>

      <section>
        <h2>Eligibility</h2>
        <dl>
          <dt>Government contribution eligible</dt>
          <dd>{ks.governmentContributionEligible ? "Yes" : "No"}</dd>
          {ks.firstHomeBuyerEligible != null && (
            <>
              <dt>First home buyer eligible</dt>
              <dd>{ks.firstHomeBuyerEligible ? "Yes" : "No"}</dd>
            </>
          )}
        </dl>
      </section>
    </main>
  );
}
