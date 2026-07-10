import Link from "next/link";
import { fetchWinzData } from "../actions";

export const metadata = { title: "Your Benefits — Work and Income — My Gov NZ" };

export default async function WinzBenefitsPage() {
  const data = await fetchWinzData(["winz:benefits"]);

  if (!data) {
    return (
      <main>
        <Link href="/dept/winz">← Back to Work and Income</Link>
        <h1>Your Benefits</h1>
        <p>Unable to load your benefit information.</p>
      </main>
    );
  }

  return (
    <main>
      <Link href="/dept/winz">← Back to Work and Income</Link>
      <h1>Your Benefits</h1>

      {data.activeBenefits.length === 0 && <p>You have no active benefits.</p>}

      {data.activeBenefits.map((b) => (
        <section key={b.type} style={{ marginBottom: "1.5rem" }}>
          <h2>{b.type}</h2>
          <dl>
            <dt>Weekly amount</dt>
            <dd>
              <strong>${Number(b.weeklyAmount).toFixed(2)}</strong>
            </dd>
            <dt>Status</dt>
            <dd>{b.status}</dd>
            {b.startDate && (
              <>
                <dt>Started</dt>
                <dd>{b.startDate}</dd>
              </>
            )}
            {b.reviewDate && (
              <>
                <dt>Review date</dt>
                <dd>{b.reviewDate}</dd>
              </>
            )}
          </dl>
        </section>
      ))}

      <nav>
        <Link href="/dept/winz/payments">Payment history</Link>
        {" · "}
        <Link href="/dept/winz/request-appointment">Request an appointment</Link>
      </nav>
    </main>
  );
}
