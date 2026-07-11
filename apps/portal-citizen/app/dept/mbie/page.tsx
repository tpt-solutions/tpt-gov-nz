import Link from "next/link";
import { fetchMbieData } from "./actions";
import MbieAiPrompt from "./ai-prompt";

export const metadata = { title: "MBIE — My Gov NZ" };

export default async function MbieOverviewPage() {
  const data = await fetchMbieData(["mbie:business", "mbie:directorships"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Business, Innovation and Employment</h1>
        <p>Unable to load your business information. Please grant access to continue.</p>
        <Link href="/consent?dept=mbie">Grant business access</Link>
      </main>
    );
  }

  const businesses = data.businessRegistrations ?? [];
  const directorships = data.directorships ?? [];

  return (
    <main>
      <h1>Ministry of Business, Innovation and Employment</h1>
      <p>Person id: ••••{data.personId.slice(-4)}</p>

      <section>
        <h2>Business registrations</h2>
        <p>{businesses.length} registered business(es).</p>
        <Link href="/dept/mbie/business">View business registrations →</Link>
      </section>

      <section>
        <h2>Directorships</h2>
        <p>{directorships.length} directorship(s).</p>
        <Link href="/dept/mbie/directorships">View directorships →</Link>
      </section>

      <MbieAiPrompt />
    </main>
  );
}
