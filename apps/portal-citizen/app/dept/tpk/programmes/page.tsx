import Link from "next/link";
import { fetchTpkData } from "../actions";

export const metadata = { title: "TPK Programmes — My Gov NZ" };

export default async function TpkProgrammesPage() {
  const data = await fetchTpkData(["tpk:programmes"]);
  const programmes = data?.programmes ?? [];

  return (
    <main>
      <Link href="/dept/tpk">← Back to Te Puni Kōkiri</Link>
      <h1>Programmes</h1>

      <section>
        <h2>Your programmes</h2>
        {programmes.length === 0 ? (
          <p>No Te Puni Kōkiri programmes on record.</p>
        ) : (
          <ul>
            {programmes.map((p) => (
              <li key={p.programmeName}>
                <strong>{p.programmeName}</strong> — {p.status}
                <br />
                <small>{p.region}</small>
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}
