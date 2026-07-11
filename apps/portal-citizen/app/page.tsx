import Link from "next/link";
import { getSession } from "@/app/lib/session";

export const metadata = { title: "Home" };

export default async function HomePage() {
  const session = await getSession();
  return (
    <div>
      <section className="card">
        <h1 style={{ marginTop: 0 }}>My Gov NZ</h1>
        <p>Your unified New Zealand government services portal.</p>
        {session ? (
          <Link href="/dashboard" className="btn">
            Go to your dashboard
          </Link>
        ) : (
          <Link href="/login" className="btn">
            Sign in
          </Link>
        )}
      </section>
      <section className="card">
        <h2 style={{ marginTop: 0 }}>How it works</h2>
        <ul>
          <li>You hold a self-custodied digital identity (a DID) in your browser.</li>
          <li>Each department serves its own data directly — there is no central profile.</li>
          <li>You grant and revoke consent for any cross-department sharing.</li>
          <li>An optional AI assistant answers questions using only your consented data.</li>
        </ul>
      </section>
    </div>
  );
}
