import Link from "next/link";
import { fetchPacificDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Pacific Peoples — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PacificStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Pacific Peoples — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Pacific Peoples records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchPacificDataForCitizen(did, ["pacific:programmes", "pacific:language-services"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Pacific Peoples — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Pacific Peoples information for this citizen.</p>}

      {data && (
        <>
          <p>Pacific Peoples ID: ••••{data.pacificId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data && data.programmes ? (
          <Link href={`/dept/pacific/programmes?did=${encodeURIComponent(did)}`}>View programmes</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Language services</h2>
        {data && data.language_services ? (
          <Link href={`/dept/pacific/language_services?did=${encodeURIComponent(did)}`}>View language services</Link>
        ) : (
          <p>No language services on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
