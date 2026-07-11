import Link from "next/link";
import { fetchNzdfData } from "./actions";
import NzdfAiPrompt from "./ai-prompt";

export const metadata = { title: "New Zealand Defence Force — My Gov NZ" };

export default async function NzdfOverviewPage() {
  const data = await fetchNzdfData(["nzdf:service-records", "nzdf:deployments"]);

  if (!data) {
    return (
      <main>
        <h1>New Zealand Defence Force</h1>
        <p>Unable to load your NZDF information. Please grant access to continue.</p>
        <Link href={"/consent?dept=nzdf"}>Grant NZDF access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>New Zealand Defence Force</h1>
      <p>NZDF ID: ••••{data.nzdfId.slice(-4)}</p>

      <section>
        <h2>Service records</h2>
        {data.service_records ? (
          <Link href={"/dept/nzdf/service_records"}>View service records →</Link>
        ) : (
          <p>No service records on file.</p>
        )}
      </section>

      <section>
        <h2>Deployments</h2>
        {data.deployments ? (
          <Link href={"/dept/nzdf/deployments"}>View deployments →</Link>
        ) : (
          <p>No deployments on file.</p>
        )}
      </section>

      <NzdfAiPrompt />
    </main>
  );
}
