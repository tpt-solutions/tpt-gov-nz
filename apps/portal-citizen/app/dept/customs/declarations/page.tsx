import Link from "next/link";
import { fetchCustomsData } from "../actions";

export const metadata = { title: "Declarations — My Gov NZ" };

export default async function CustomsDeclarationsPage() {
  const data = await fetchCustomsData(["customs:declarations"]);
  const declarations = data?.declarations ?? [];

  return (
    <main>
      <Link href="/dept/customs">← Back to customs</Link>
      <h1>Declarations</h1>

      {declarations.length === 0 ? (
        <p>No declarations on record.</p>
      ) : (
        <ul>
          {declarations.map((d) => (
            <li key={d.declarationId}>
              <strong>{d.declarationId}</strong> — from {d.countryFrom} ({d.status})
              <br />
              <small>Declared {d.date} · {d.goodsDeclared}</small>
            </li>
          ))}
        </ul>
      )}

      <section>
        <h2>Submit a new declaration</h2>
        <Link href="/dept/customs/declarations/submit">Submit a traveller declaration →</Link>
      </section>
    </main>
  );
}
