import Link from "next/link";
import { fetchMoeDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Education — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MoeStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Education — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Education records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMoeDataForCitizen(did, ["moe:enrolment", "moe:student-support"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Education — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Education information for this citizen.</p>}

      {data && (
        <>
          <p>Education ID: ••••{data.moeId.slice(-4)}</p>

      <section>
        <h2>Enrolment</h2>
        {data && data.enrolment ? (
          <Link href={`/dept/moe/enrolment?did=${encodeURIComponent(did)}`}>View enrolment</Link>
        ) : (
          <p>No enrolment on file.</p>
        )}
      </section>

      <section>
        <h2>Student support</h2>
        {data && data.student_support ? (
          <Link href={`/dept/moe/student_support?did=${encodeURIComponent(did)}`}>View student support</Link>
        ) : (
          <p>No student support on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
