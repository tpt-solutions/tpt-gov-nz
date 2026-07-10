import Link from "next/link";
import { fetchIrdDataForCitizen } from "../actions";

export const metadata = { title: "KiwiSaver — IRD Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StaffKiwiSaverPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  const data = did ? await fetchIrdDataForCitizen(did, ["ird:kiwisaver"]) : null;
  const ks = data?.kiwiSaver;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ird?did=${did ? encodeURIComponent(did) : ""}`}>← Back to IRD case file</Link>
      <h1>KiwiSaver</h1>
      <p><em>Read-only case worker view.</em></p>

      {!ks && <p>No KiwiSaver data available for this citizen.</p>}

      {ks && (
        <>
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
              <dt>Citizen contribution rate</dt>
              <dd>{ks.contributionRate}%</dd>
              {ks.employerContributionRate != null && (
                <>
                  <dt>Employer contribution rate</dt>
                  <dd>{ks.employerContributionRate}%</dd>
                </>
              )}
            </dl>
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
        </>
      )}
    </main>
  );
}
