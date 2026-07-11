import Link from "next/link";
import { fetchWorksafeData } from "./actions";
import WorksafeAiPrompt from "./ai-prompt";

export const metadata = { title: "WorkSafe New Zealand — My Gov NZ" };

export default async function WorksafeOverviewPage() {
  const data = await fetchWorksafeData(["worksafe:inspections", "worksafe:investigations"]);

  if (!data) {
    return (
      <main>
        <h1>WorkSafe New Zealand</h1>
        <p>Unable to load your WorkSafe information. Please grant access to continue.</p>
        <Link href={"/consent?dept=worksafe"}>Grant WorkSafe access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>WorkSafe New Zealand</h1>
      <p>WorkSafe ID: ••••{data.worksafeId.slice(-4)}</p>

      <section>
        <h2>Inspections</h2>
        {data.inspections ? (
          <Link href={"/dept/worksafe/inspections"}>View inspections →</Link>
        ) : (
          <p>No inspections on file.</p>
        )}
      </section>

      <section>
        <h2>Investigations</h2>
        {data.investigations ? (
          <Link href={"/dept/worksafe/investigations"}>View investigations →</Link>
        ) : (
          <p>No investigations on file.</p>
        )}
      </section>

      <WorksafeAiPrompt />
    </main>
  );
}
