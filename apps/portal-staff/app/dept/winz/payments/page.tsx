import Link from "next/link";
import { fetchWinzDataForCitizen } from "../actions";

export const metadata = { title: "Payments — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WinzStaffPaymentsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Payments — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchWinzDataForCitizen(did, ["winz:payments"]);
  const payments = data?.payments ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/winz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Payment History — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load payments for this citizen.</p>}

      {data && payments.length === 0 && <p>No payments recorded.</p>}

      {data && payments.length > 0 && (
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
      )}
    </main>
  );
}
