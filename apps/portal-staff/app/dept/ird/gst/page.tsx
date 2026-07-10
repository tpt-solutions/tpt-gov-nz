import Link from "next/link";
import { fetchIrdDataForCitizen } from "../actions";

export const metadata = { title: "GST — IRD Case File — My Gov NZ" };

const STATUS_LABELS: Record<string, string> = {
  filed: "Filed",
  due: "Due",
  overdue: "Overdue",
  "not-required": "Not required",
};

type SearchParams = { did?: string };

export default async function StaffGstPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  const data = did ? await fetchIrdDataForCitizen(did, ["ird:tax-summary"]) : null;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ird?did=${did ? encodeURIComponent(did) : ""}`}>← Back to IRD case file</Link>
      <h1>GST</h1>
      <p><em>Read-only case worker view.</em></p>

      {!data && <p>No GST information available for this citizen.</p>}

      {data && !data.gstRegistered && <p>Citizen is not registered for GST.</p>}

      {data?.gstRegistered && (
        <>
          <section>
            <h2>Registration</h2>
            <dl>
              <dt>Status</dt>
              <dd>Registered</dd>
            </dl>
          </section>

          {(data.gstPeriods ?? []).length > 0 ? (
            <section>
              <h2>GST periods</h2>
              <table>
                <thead>
                  <tr>
                    <th>Period</th>
                    <th>Due</th>
                    <th>Status</th>
                    <th>Sales</th>
                    <th>GST on sales</th>
                    <th>GST on purchases</th>
                    <th>Refund / payment</th>
                  </tr>
                </thead>
                <tbody>
                  {data.gstPeriods!.map((p) => (
                    <tr key={p.periodId}>
                      <td>
                        {p.periodStart} – {p.periodEnd}
                      </td>
                      <td>{p.filingDue}</td>
                      <td>{STATUS_LABELS[p.status] ?? p.status}</td>
                      <td>{p.salesIncome != null ? `$${Number(p.salesIncome).toLocaleString()}` : "—"}</td>
                      <td>{p.gstOnSales != null ? `$${Number(p.gstOnSales).toLocaleString()}` : "—"}</td>
                      <td>
                        {p.gstOnPurchases != null ? `$${Number(p.gstOnPurchases).toLocaleString()}` : "—"}
                      </td>
                      <td>
                        {p.refundOrPayment != null ? (
                          Number(p.refundOrPayment) >= 0 ? (
                            <span>Refund ${Number(p.refundOrPayment).toLocaleString()}</span>
                          ) : (
                            <span>Owing ${Math.abs(Number(p.refundOrPayment)).toLocaleString()}</span>
                          )
                        ) : (
                          "—"
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>
          ) : (
            <section>
              <p>No GST period data available.</p>
            </section>
          )}
        </>
      )}
    </main>
  );
}
