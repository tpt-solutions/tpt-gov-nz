import Link from "next/link";
import { fetchTecData } from "./actions";
import TecAiPrompt from "./ai-prompt";

export const metadata = { title: "Tertiary Education Commission — My Gov NZ" };

export default async function TecOverviewPage() {
  const data = await fetchTecData(["tec:funding", "tec:courses"]);

  if (!data) {
    return (
      <main>
        <h1>Tertiary Education Commission</h1>
        <p>Unable to load your TEC information. Please grant access to continue.</p>
        <Link href={"/consent?dept=tec"}>Grant TEC access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Tertiary Education Commission</h1>
      <p>TEC ID: ••••{data.tecId.slice(-4)}</p>

      <section>
        <h2>Funding</h2>
        {data.funding ? (
          <Link href={"/dept/tec/funding"}>View funding →</Link>
        ) : (
          <p>No funding on file.</p>
        )}
      </section>

      <section>
        <h2>Courses</h2>
        {data.courses ? (
          <Link href={"/dept/tec/courses"}>View courses →</Link>
        ) : (
          <p>No courses on file.</p>
        )}
      </section>

      <TecAiPrompt />
    </main>
  );
}
