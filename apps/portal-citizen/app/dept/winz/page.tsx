import Link from "next/link";
import { fetchWinzData } from "./actions";
import WinzAiPrompt from "./ai-prompt";

export const metadata = { title: "Work and Income — My Gov NZ" };

export default async function WinzOverviewPage() {
  const data = await fetchWinzData(["winz:benefits", "winz:payments", "winz:case-notes"]);

  if (!data) {
    return (
      <main>
        <h1>Work and Income (MSD)</h1>
        <p>Unable to load your Work and Income information. Please grant access to continue.</p>
        <Link href="/consent?dept=winz">Grant Work and Income access</Link>
      </main>
    );
  }

  const total = Number(data.totalWeeklyPayment);
  const active = data.activeBenefits.filter((b) => b.status === "active");
  const lastPayment = data.payments?.[0];

  return (
    <main>
      <h1>Work and Income (MSD)</h1>
      <p>Client reference: {data.clientId}</p>

      <section>
        <h2>Your support</h2>
        <dl>
          <dt>Total weekly payment</dt>
          <dd>
            <strong>${total.toFixed(2)}</strong>
          </dd>
          <dt>Active benefits</dt>
          <dd>{active.length}</dd>
        </dl>
        <Link href="/dept/winz/benefits">View your benefits →</Link>
      </section>

      {lastPayment && (
        <section>
          <h2>Latest payment</h2>
          <dl>
            <dt>Benefit</dt>
            <dd>{lastPayment.benefitType}</dd>
            <dt>Amount</dt>
            <dd>${Number(lastPayment.amount).toFixed(2)}</dd>
            <dt>Paid</dt>
            <dd>{lastPayment.paymentDate}</dd>
          </dl>
          <Link href="/dept/winz/payments">View payment history →</Link>
        </section>
      )}

      <section>
        <h2>Need to talk to us?</h2>
        <Link href="/dept/winz/request-appointment">Request an appointment</Link>
      </section>

      <WinzAiPrompt />
    </main>
  );
}
