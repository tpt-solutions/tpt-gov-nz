import Link from "next/link";
import InfringementActions from "./form";
import { fetchPoliceData } from "../actions";

export const metadata = { title: "Infringements — My Gov NZ" };

export default async function PoliceInfringementsPage() {
  const data = await fetchPoliceData(["police:infringements"]);
  const infringements = data?.infringements ?? [];

  return (
    <main>
      <Link href="/dept/police">← Back to Police</Link>
      <h1>Infringements</h1>

      {infringements.length === 0 ? (
        <p>No infringements on record.</p>
      ) : (
        <ul>
          {infringements.map((i) => (
            <li key={i.ticketNumber}>
              <strong>{i.ticketNumber}</strong> — {i.offenseType} ({i.status}): ${i.amount}
              <br />
              <small>
                {i.description} — issued {i.issueDate}
                {i.location ? ` at ${i.location}` : ""}
                {i.demeritPoints != null ? ` · ${i.demeritPoints} demerit points` : ""}
              </small>
              <InfringementActions ticketNumber={i.ticketNumber} />
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
