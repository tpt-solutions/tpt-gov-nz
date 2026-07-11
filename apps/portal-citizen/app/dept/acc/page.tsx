import Link from "next/link";
import { fetchAccData } from "./actions";
import AccAiPrompt from "./ai-prompt";

export const metadata = { title: "ACC — My Gov NZ" };

export default async function AccOverviewPage() {
  const data = await fetchAccData([
    "acc:claims",
    "acc:entitlements",
    "acc:rehabilitation",
  ]);

  if (!data) {
    return (
      <main>
        <h1>Accident Compensation Corporation (ACC)</h1>
        <p>Unable to load your ACC information. Please grant access to continue.</p>
        <Link href="/consent?dept=acc">Grant ACC access</Link>
      </main>
    );
  }

  const activeClaims =
    data.claims?.filter((c) => c.status === "open" || c.status === "approved") ?? [];
  const weeklyAmount = data.entitlements?.hasEntitlement
    ? data.entitlements.weeklyAmount
    : undefined;
  const rehabCount = data.rehabilitation?.length ?? 0;

  return (
    <main>
      <h1>Accident Compensation Corporation (ACC)</h1>
      <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

      <section>
        <h2>Claims</h2>
        <p>{activeClaims.length} active claim(s).</p>
        <Link href="/dept/acc/claims">View claims →</Link>
      </section>

      <section>
        <h2>Entitlements</h2>
        {data.entitlements?.hasEntitlement ? (
          <p>Weekly amount: ${weeklyAmount}</p>
        ) : (
          <p>No active entitlement.</p>
        )}
        <Link href="/dept/acc/entitlements">View entitlements →</Link>
      </section>

      <section>
        <h2>Rehabilitation</h2>
        <p>{rehabCount} rehabilitation plan(s).</p>
        <Link href="/dept/acc/rehabilitation">View rehabilitation →</Link>
      </section>

      <AccAiPrompt />
    </main>
  );
}
