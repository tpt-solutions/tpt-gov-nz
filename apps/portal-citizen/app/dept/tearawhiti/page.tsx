import Link from "next/link";
import { fetchTearawhitiData } from "./actions";
import TearawhitiAiPrompt from "./ai-prompt";

export const metadata = { title: "Te Arawhiti — My Gov NZ" };

export default async function TearawhitiOverviewPage() {
  const data = await fetchTearawhitiData(["tearawhiti:treaty-settlements", "tearawhiti:engagements"]);

  if (!data) {
    return (
      <main>
        <h1>Te Arawhiti</h1>
        <p>Unable to load your Te Arawhiti information. Please grant access to continue.</p>
        <Link href={"/consent?dept=tearawhiti"}>Grant Te Arawhiti access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Te Arawhiti</h1>
      <p>Te Arawhiti ID: ••••{data.tearawhitiId.slice(-4)}</p>

      <section>
        <h2>Treaty settlements</h2>
        {data.treaty_settlements ? (
          <Link href={"/dept/tearawhiti/treaty_settlements"}>View treaty settlements →</Link>
        ) : (
          <p>No treaty settlements on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data.engagements ? (
          <Link href={"/dept/tearawhiti/engagements"}>View engagements →</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>

      <TearawhitiAiPrompt />
    </main>
  );
}
