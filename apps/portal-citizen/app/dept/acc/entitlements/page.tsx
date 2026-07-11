import Link from "next/link";
import { fetchAccData } from "../actions";

export const metadata = { title: "ACC Entitlements — My Gov NZ" };

export default async function AccEntitlementsPage() {
  const data = await fetchAccData(["acc:entitlements"]);
  const e = data?.entitlements;

  return (
    <main>
      <Link href="/dept/acc">← Back to ACC</Link>
      <h1>ACC Entitlements</h1>

      {!e ? (
        <p>No entitlement information on record.</p>
      ) : !e.hasEntitlement ? (
        <p>You do not currently have an active ACC entitlement.</p>
      ) : (
        <dl>
          <dt>Type</dt>
          <dd>{e.type ?? "Active entitlement"}</dd>
          <dt>Weekly amount</dt>
          <dd>{e.weeklyAmount != null ? `$${e.weeklyAmount}` : "n/a"}</dd>
          <dt>Remaining weeks</dt>
          <dd>{e.remainingWeeks != null ? e.remainingWeeks : "n/a"}</dd>
        </dl>
      )}
    </main>
  );
}
