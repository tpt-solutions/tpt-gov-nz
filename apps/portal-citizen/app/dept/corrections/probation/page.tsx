import Link from "next/link";
import { fetchCorrectionsData } from "../actions";

export const metadata = { title: "Probation — My Gov NZ" };

export default async function CorrectionsProbationPage() {
  const data = await fetchCorrectionsData(["corrections:probation"]);
  const probation = data?.probation;

  return (
    <main>
      <Link href="/dept/corrections">← Back to Corrections</Link>
      <h1>Probation</h1>

      {!probation ? (
        <p>No probation record on file.</p>
      ) : (
        <ul>
          <li>
            <strong>Status:</strong> {probation.status}
            <br />
            <small>
              Officer {probation.officerName} · {probation.location} · next report {probation.nextReportDate}
            </small>
          </li>
        </ul>
      )}
    </main>
  );
}
