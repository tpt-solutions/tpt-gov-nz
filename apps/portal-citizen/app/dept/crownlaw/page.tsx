import Link from "next/link";
import { fetchCrownlawData } from "./actions";
import CrownlawAiPrompt from "./ai-prompt";

export const metadata = { title: "Crown Law Office — My Gov NZ" };

export default async function CrownlawOverviewPage() {
  const data = await fetchCrownlawData(["crownlaw:legal-opinions", "crownlaw:litigation"]);

  if (!data) {
    return (
      <main>
        <h1>Crown Law Office</h1>
        <p>Unable to load your Crown Law information. Please grant access to continue.</p>
        <Link href={"/consent?dept=crownlaw"}>Grant Crown Law access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Crown Law Office</h1>
      <p>Crown Law ID: ••••{data.crownlawId.slice(-4)}</p>

      <section>
        <h2>Legal opinions</h2>
        {data.legal_opinions ? (
          <Link href={"/dept/crownlaw/legal_opinions"}>View legal opinions →</Link>
        ) : (
          <p>No legal opinions on file.</p>
        )}
      </section>

      <section>
        <h2>Litigation</h2>
        {data.litigation ? (
          <Link href={"/dept/crownlaw/litigation"}>View litigation →</Link>
        ) : (
          <p>No litigation on file.</p>
        )}
      </section>

      <CrownlawAiPrompt />
    </main>
  );
}
