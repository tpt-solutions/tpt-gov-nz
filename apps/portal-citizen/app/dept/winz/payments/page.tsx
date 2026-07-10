import Link from "next/link";
import { fetchWinzData } from "../actions";

export const metadata = { title: "Payment History — Work and Income — My Gov NZ" };

export default async function WinzPaymentsPage() {
  const data = await fetchWinzData(["winz:payments"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/winz">← Back to Work and Income</Link>
        <h1>Payment History</h1>
        <p>Unable to load your payment history.</p>
      </main>
    );
  }

  const payments = data.payments ?? [];

  return (
    <main>
      <Link href="/dept/winz">← Back to Work and Income</Link>
      <h1>Payment History</h1>

      {payments.length === 0 && <p>No payments recorded yet.</p>}

      <table style={{ width: "100%", borderCollapse: "collapse", marginTop: "1rem" }}>
        <thead>
          <tr>
            <th style={{ textAlign: "left" }}>Date</th>
            <th style={{ textAlign: "left" }}>Benefit</th>
            <th style={{ textAlign: "right" }}>Amount</th>
            <th style={{ textAlign: "left" }}>Method</th>
          </tr>
        </thead>
        <tbody>
          {payments.map((p) => (
            <tr key={p.paymentId}>
              <td>{p.paymentDate}</td>
              <td>{p.benefitType}</td>
              <td style={{ textAlign: "right" }}>${Number(p.amount).toFixed(2)}</td>
              <td>{p.method}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <nav style={{ marginTop: "1.5rem" }}>
        <Link href="/dept/winz/benefits">Your benefits</Link>
      </nav>
    </main>
  );
}
