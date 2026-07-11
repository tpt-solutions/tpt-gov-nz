import Link from "next/link";
import { fetchEroData } from "./actions";
import EroAiPrompt from "./ai-prompt";

export const metadata = { title: "Education Review Office — My Gov NZ" };

export default async function EroOverviewPage() {
  const data = await fetchEroData(["ero:reviews", "ero:reports"]);

  if (!data) {
    return (
      <main>
        <h1>Education Review Office</h1>
        <p>Unable to load your ERO information. Please grant access to continue.</p>
        <Link href={"/consent?dept=ero"}>Grant ERO access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Education Review Office</h1>
      <p>ERO ID: ••••{data.eroId.slice(-4)}</p>

      <section>
        <h2>Reviews</h2>
        {data.reviews ? (
          <Link href={"/dept/ero/reviews"}>View reviews →</Link>
        ) : (
          <p>No reviews on file.</p>
        )}
      </section>

      <section>
        <h2>Reports</h2>
        {data.reports ? (
          <Link href={"/dept/ero/reports"}>View reports →</Link>
        ) : (
          <p>No reports on file.</p>
        )}
      </section>

      <EroAiPrompt />
    </main>
  );
}
