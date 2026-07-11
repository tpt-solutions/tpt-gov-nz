import Link from "next/link";
import { fetchPacificDataForCitizen } from "../actions";

export const metadata = { title: "Language services — Ministry for Pacific Peoples — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PacificLanguageServicesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchPacificDataForCitizen(did, ["pacific:language-services"]);
  const rows = data?.language_services ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/pacific?did=${encodeURIComponent(did)}`}>← Back to Pacific Peoples case file</Link>
      <h1>Language services</h1>
      {rows.length === 0 ? (
        <p>No language services on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>service</th>
                <th>region</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.service}</td>
                  <td>{row.region}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
