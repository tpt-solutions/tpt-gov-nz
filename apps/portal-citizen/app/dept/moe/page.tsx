import Link from "next/link";
import { fetchMoeData } from "./actions";
import MoeAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry of Education — My Gov NZ" };

export default async function MoeOverviewPage() {
  const data = await fetchMoeData(["moe:enrolment", "moe:student-support"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Education</h1>
        <p>Unable to load your Education information. Please grant access to continue.</p>
        <Link href={"/consent?dept=moe"}>Grant Education access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry of Education</h1>
      <p>Education ID: ••••{data.moeId.slice(-4)}</p>

      <section>
        <h2>Enrolment</h2>
        {data.enrolment ? (
          <Link href={"/dept/moe/enrolment"}>View enrolment →</Link>
        ) : (
          <p>No enrolment on file.</p>
        )}
      </section>

      <section>
        <h2>Student support</h2>
        {data.student_support ? (
          <Link href={"/dept/moe/student_support"}>View student support →</Link>
        ) : (
          <p>No student support on file.</p>
        )}
      </section>

      <MoeAiPrompt />
    </main>
  );
}
