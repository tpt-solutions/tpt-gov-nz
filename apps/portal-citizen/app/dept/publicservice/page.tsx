import Link from "next/link";
import { fetchPublicserviceData } from "./actions";
import PublicserviceAiPrompt from "./ai-prompt";

export const metadata = { title: "Te Kawa Mataaho Public Service Commission — My Gov NZ" };

export default async function PublicserviceOverviewPage() {
  const data = await fetchPublicserviceData(["publicservice:workforce", "publicservice:agency-ratings"]);

  if (!data) {
    return (
      <main>
        <h1>Te Kawa Mataaho Public Service Commission</h1>
        <p>Unable to load your Public Service information. Please grant access to continue.</p>
        <Link href={"/consent?dept=publicservice"}>Grant Public Service access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Te Kawa Mataaho Public Service Commission</h1>
      <p>Public Service ID: ••••{data.publicserviceId.slice(-4)}</p>

      <section>
        <h2>Workforce</h2>
        {data.workforce ? (
          <Link href={"/dept/publicservice/workforce"}>View workforce →</Link>
        ) : (
          <p>No workforce on file.</p>
        )}
      </section>

      <section>
        <h2>Agency ratings</h2>
        {data.agency_ratings ? (
          <Link href={"/dept/publicservice/agency_ratings"}>View agency ratings →</Link>
        ) : (
          <p>No agency ratings on file.</p>
        )}
      </section>

      <PublicserviceAiPrompt />
    </main>
  );
}
