import Link from "next/link";
import { searchCitizens } from "../lib/directory";
import { STAFF_DEPARTMENTS } from "../lib/config";

export const metadata = { title: "Citizen Search — Staff — My Gov NZ" };

type SearchParams = { q?: string };

export default async function CitizenSearchPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { q = "" } = await searchParams;
  const results = q.trim() ? await searchCitizens(q) : [];

  return (
    <main style={{ padding: "1rem" }}>
      <h1>Citizen Search</h1>
      <p>
        Search by government DID or a department-local identifier. Results are shown regardless of
        consent; consent gating is applied when you open the case file.
      </p>

      <form action="/citizens" method="get" style={{ margin: "1rem 0", maxWidth: "32rem" }}>
        <label htmlFor="q">Search</label>
        <input
          id="q"
          name="q"
          defaultValue={q}
          placeholder="did:gov:nz:demo-alex-tane"
          style={{ display: "block", width: "100%", margin: "0.5rem 0" }}
        />
        <button type="submit">Search</button>
      </form>

      {q.trim() && results.length === 0 && (
        <p>No citizens matched &ldquo;{q}&rdquo;.</p>
      )}

      {results.map((entry) => (
        <article
          key={entry.did}
          style={{ border: "1px solid #ccc", padding: "1rem", margin: "1rem 0" }}
        >
          <h2>{entry.displayName}</h2>
          <p>
            <code>{entry.did}</code>
          </p>
          <ul>
            {STAFF_DEPARTMENTS.map((d) => {
              const local = entry.deptIds[d.id];
              return (
                <li key={d.id}>
                  {d.shortName}
                  {local ? `: ${local}` : " — not known to department"}
                </li>
              );
            })}
          </ul>
          <Link href={`/citizens/${encodeURIComponent(entry.did)}`}>Open case file →</Link>
        </article>
      ))}
    </main>
  );
}
