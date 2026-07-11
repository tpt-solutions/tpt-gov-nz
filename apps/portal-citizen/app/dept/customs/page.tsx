import Link from "next/link";
import { fetchCustomsData } from "./actions";
import CustomsAiPrompt from "./ai-prompt";

export const metadata = { title: "Customs — My Gov NZ" };

export default async function CustomsOverviewPage() {
  const data = await fetchCustomsData(["customs:travel", "customs:declarations"]);

  if (!data) {
    return (
      <main>
        <h1>New Zealand Customs Service</h1>
        <p>Unable to load your customs information. Please grant access to continue.</p>
        <Link href="/consent?dept=customs">Grant customs access</Link>
      </main>
    );
  }

  const frequentTraveller = data.travel?.frequentTraveller ?? false;
  const openDeclarations =
    data.declarations?.filter((d) => d.status !== "cleared" && d.status !== "referred") ?? [];

  return (
    <main>
      <h1>New Zealand Customs Service</h1>
      <p>Traveller id: ••••{data.travellerId.slice(-4)}</p>

      <section>
        <h2>Travel</h2>
        <p>{data.travel ? "Travel record on file." : "No travel record on file."}</p>
        <Link href="/dept/customs/travel">View travel →</Link>
      </section>

      <section>
        <h2>Declarations</h2>
        <p>{openDeclarations.length} open declaration(s).</p>
        <Link href="/dept/customs/declarations">View declarations →</Link>
        {" · "}
        <Link href="/dept/customs/declarations/submit">Submit a declaration →</Link>
      </section>

      <CustomsAiPrompt />
    </main>
  );
}
