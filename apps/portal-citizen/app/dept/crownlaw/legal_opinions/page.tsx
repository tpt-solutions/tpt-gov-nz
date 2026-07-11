import Link from "next/link";
import { fetchCrownlawData } from "../actions";

export const metadata = { title: "Legal opinions — Crown Law Office — My Gov NZ" };

export default async function CrownlawLegalOpinionsPage() {
  const data = await fetchCrownlawData(["crownlaw:legal-opinions"]);
  if (!data) {
    return (
      <main>
        <h1>Legal opinions</h1>
        <p>Unable to load your Crown Law information.</p>
        <Link href={"/consent?dept=crownlaw"}>Grant Crown Law access</Link>
      </main>
    );
  }

  const rows = data.legal_opinions ?? [];

  return (
    <main>
      <Link href={"/dept/crownlaw"}>← Back to Crown Law</Link>
      <h1>Legal opinions</h1>
      {rows.length === 0 ? (
        <p>No legal opinions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>topic</th>
                <th>issuedDate</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.topic}</td>
                  <td>{row.issuedDate}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
