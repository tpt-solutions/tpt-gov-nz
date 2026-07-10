import Link from "next/link";
import { fetchWinzDataForCitizen } from "./actions";

export const metadata = { title: "Work and Income — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WinzStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Work and Income (MSD) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Work and Income records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchWinzDataForCitizen(did, [
    "winz:benefits",
    "winz:payments",
    "winz:case-notes",
  ]);

  const total = data ? Number(data.totalWeeklyPayment) : 0;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Work and Income (MSD) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Work and Income information for this citizen.</p>}

      {data && (
        <>
          <p>Client reference: {data.clientId}</p>

          <section>
            <h2>Support summary</h2>
            <dl>
              <dt>Total weekly payment</dt>
              <dd>
                <strong>${total.toFixed(2)}</strong>
              </dd>
              <dt>Active benefits</dt>
              <dd>{data.activeBenefits.filter((b) => b.status === "active").length}</dd>
            </dl>
          </section>

          <section>
            <h2>Benefits</h2>
            {data.activeBenefits.length === 0 && <p>No active benefits.</p>}
            {data.activeBenefits.map((b) => (
              <div key={b.type} style={{ marginBottom: "0.5rem" }}>
                <strong>{b.type}</strong> — ${Number(b.weeklyAmount).toFixed(2)}/week ({b.status})
                {b.reviewDate ? `, review ${b.reviewDate}` : ""}
              </div>
            ))}
          </section>

          {data.payments && data.payments.length > 0 && (
            <section>
              <h2>Latest payment</h2>
              <p>
                {data.payments[0].benefitType}: ${Number(data.payments[0].amount).toFixed(2)} on{" "}
                {data.payments[0].paymentDate} via {data.payments[0].method}
              </p>
            </section>
          )}

          <nav>
            <Link href={`/dept/winz/benefits?did=${encodeURIComponent(did)}`}>Benefits</Link>
            {" · "}
            <Link href={`/dept/winz/payments?did=${encodeURIComponent(did)}`}>Payments</Link>
          </nav>
        </>
      )}
    </main>
  );
}
