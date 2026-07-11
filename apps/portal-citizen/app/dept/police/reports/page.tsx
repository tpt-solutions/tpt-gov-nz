import Link from "next/link";
import ReportForm from "./form";
import { fetchPoliceData } from "../actions";

export const metadata = { title: "Reports — My Gov NZ" };

export default async function PoliceReportsPage() {
  const data = await fetchPoliceData(["police:reports"]);
  const reports = data?.reports ?? [];

  return (
    <main>
      <Link href="/dept/police">← Back to Police</Link>
      <h1>Reports</h1>

      <section>
        <h2>Your reports</h2>
        {reports.length === 0 ? (
          <p>No reports on record.</p>
        ) : (
          <ul>
            {reports.map((r) => (
              <li key={r.reportNumber}>
                <strong>{r.reportNumber}</strong> — {r.reportType} ({r.status}): {r.description}
                <br />
                <small>Filed {r.filedDate}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>File a new report</h2>
        <ReportForm />
      </section>
    </main>
  );
}
