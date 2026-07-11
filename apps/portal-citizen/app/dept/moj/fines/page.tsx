import Link from "next/link";
import PayFineForm from "./form";
import { fetchMojData } from "../actions";

export const metadata = { title: "MOJ Fines — My Gov NZ" };

export default async function MojFinesPage() {
  const data = await fetchMojData(["moj:fines"]);
  const fines = data?.fines ?? [];

  return (
    <main>
      <Link href="/dept/moj">← Back to Ministry of Justice</Link>
      <h1>Fines</h1>

      <section>
        <h2>Your fines</h2>
        {fines.length === 0 ? (
          <p>No fines on record.</p>
        ) : (
          <ul>
            {fines.map((f) => (
              <li key={f.fineNumber}>
                <strong>{f.fineNumber}</strong> — {f.fineType} ({f.status}): ${f.amount}
                <br />
                <small>{f.description} — due {f.dueDate}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Pay a fine</h2>
        <PayFineForm />
      </section>
    </main>
  );
}
