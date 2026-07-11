import Link from "next/link";
import { fetchHudData } from "./actions";
import HudAiPrompt from "./ai-prompt";

export const metadata = { title: "Housing — My Gov NZ" };

export default async function HudOverviewPage() {
  const data = await fetchHudData(["hud:applications", "hud:tenancy", "hud:maintenance"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Housing and Urban Development / Kāinga Ora</h1>
        <p>Unable to load your housing information. Please grant access to continue.</p>
        <Link href="/consent?dept=hud">Grant housing access</Link>
      </main>
    );
  }

  const activeApplications =
    data.applications?.filter((a) => a.status === "submitted" || a.status === "waitlisted" || a.status === "assessed") ?? [];
  const activeTenancies = data.tenancies?.filter((t) => t.status === "active") ?? [];
  const openMaintenance = data.maintenanceRequests?.filter((m) => m.status !== "completed") ?? [];

  return (
    <main>
      <h1>Ministry of Housing and Urban Development / Kāinga Ora</h1>
      <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

      <section>
        <h2>Applications</h2>
        <p>{activeApplications.length} active application(s).</p>
        <Link href="/dept/hud/applications">View applications →</Link>
      </section>

      <section>
        <h2>Tenancy</h2>
        <p>{activeTenancies.length} active tenancy/tenancies.</p>
        <Link href="/dept/hud/tenancy">View tenancy →</Link>
      </section>

      <section>
        <h2>Maintenance</h2>
        <p>{openMaintenance.length} open maintenance request(s).</p>
        <Link href="/dept/hud/maintenance">View maintenance →</Link>
      </section>

      <HudAiPrompt />
    </main>
  );
}
