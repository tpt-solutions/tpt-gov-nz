import Link from "next/link";
import { fetchIrdData } from "../actions";

export const metadata = { title: "GST — IRD — My Gov NZ" };

const STATUS_LABELS: Record<string, string> = {
  filed: "Filed",
  due: "Due",
  overdue: "Overdue",
  "not-required": "Not required",
};

export default async function GstPage() {
  const data = await fetchIrdData(["ird:tax-summary"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>GST</h1>
        <p>Access to GST information not granted.</p>
        <Link href="/consent?dept=ird&scope=ird:tax-summary">Grant access</Link>
      </main>
    );
  }

  if (!data.gstRegistered) {
    return (
      <main>
        <Link href="/dept/ird">← Back to IRD</Link>
        <h1>GST</h1>
        <p>You are not registered for GST.</p>
      </main>
    );
  }

  const periods = data.gstPeriods ?? [];

  return (
    <main>
      <Link href="/dept/ird">← Back to IRD</Link>
      <h1>GST</h1>

      <section>
        <h2>Registration</h2>
        <dl>
          <dt>Status</dt>
          <dd>Registered</dd>
        </dl>
      </section>

      {periods.length > 0 ? (
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
                <th></th>
              </tr>
            </thead>
            <tbody>
              {periods.map((p) => (
                <tr key={p.periodId}>
                  <td>{p.periodStart} – {p.periodEnd}</td>
                  <td>{p.filingDue}</td>
                  <td>{STATUS_LABELS[p.status] ?? p.status}</td>
                  <td>{p.salesIncome != null ? `$${Number(p.salesIncome).toLocaleString()}` : "—"}</td>
                  <td>{p.gstOnSales != null ? `$${Number(p.gstOnSales).toLocaleString()}` : "—"}</td>
                  <td>{p.gstOnPurchases != null ? `$${Number(p.gstOnPurchases).toLocaleString()}` : "—"}</td>
                  <td>
                    {p.refundOrPayment != null
                      ? Number(p.refundOrPayment) >= 0
                        ? <span style={{ color: "green" }}>Refund ${Number(p.refundOrPayment).toLocaleString()}</span>
                        : <span style={{ color: "red" }}>Owing ${Math.abs(Number(p.refundOrPayment)).toLocaleString()}</span>
                      : "—"}
                  </td>
                  <td>
                    {p.status === "due" && (
                      <Link href={`/dept/ird/gst/file-return?period=${p.periodId}`}>File return</Link>
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
    </main>
  );
}
