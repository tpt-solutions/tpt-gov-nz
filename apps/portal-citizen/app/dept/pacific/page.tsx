import Link from "next/link";
import { fetchPacificData } from "./actions";
import PacificAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for Pacific Peoples — My Gov NZ" };

export default async function PacificOverviewPage() {
  const data = await fetchPacificData(["pacific:programmes", "pacific:language-services"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Pacific Peoples</h1>
        <p>Unable to load your Pacific Peoples information. Please grant access to continue.</p>
        <Link href={"/consent?dept=pacific"}>Grant Pacific Peoples access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for Pacific Peoples</h1>
      <p>Pacific Peoples ID: ••••{data.pacificId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data.programmes ? (
          <Link href={"/dept/pacific/programmes"}>View programmes →</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Language services</h2>
        {data.language_services ? (
          <Link href={"/dept/pacific/language_services"}>View language services →</Link>
        ) : (
          <p>No language services on file.</p>
        )}
      </section>

      <PacificAiPrompt />
    </main>
  );
}
