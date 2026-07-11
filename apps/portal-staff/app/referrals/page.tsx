import Link from "next/link";
import { listReferrals } from "../lib/referrals";
import { STAFF_DEPARTMENTS } from "../lib/config";

export const metadata = { title: "Referrals — Staff — My Gov NZ" };

export default async function ReferralsPage() {
  const referrals = await listReferrals();
  const byDept = (id: string) => STAFF_DEPARTMENTS.find((d) => d.id === id)?.shortName ?? id;

  return (
    <main style={{ padding: "1rem" }}>
      <h1>Referrals</h1>
      <p>All cross-department referrals created by case workers.</p>
      {referrals.length === 0 && <p>No referrals have been created yet.</p>}
      <table style={{ borderCollapse: "collapse", width: "100%", maxWidth: "60rem" }}>
        <thead>
          <tr>
            <th style={cell}>Created</th>
            <th style={cell}>Citizen</th>
            <th style={cell}>From → To</th>
            <th style={cell}>Status</th>
            <th style={cell}>Reason</th>
          </tr>
        </thead>
        <tbody>
          {referrals.map((r) => (
            <tr key={r.id}>
              <td style={cell}>{new Date(r.createdAt).toLocaleString()}</td>
              <td style={cell}>
                <Link href={`/citizens/${encodeURIComponent(r.did)}`}>{r.did}</Link>
              </td>
              <td style={cell}>
                {byDept(r.fromDept)} → {byDept(r.toDept)}
              </td>
              <td style={cell}>{r.status}</td>
              <td style={cell}>{r.reason}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}

const cell: React.CSSProperties = {
  border: "1px solid #ccc",
  padding: "0.4rem 0.6rem",
  textAlign: "left",
  verticalAlign: "top",
};
