import Link from "next/link";
import { getSession } from "@/app/lib/session";
import { DEPARTMENTS } from "@/app/lib/config";
import DeptCard from "@/app/components/DeptCard";

export const metadata = { title: "Dashboard" };

function SignInPrompt() {
  return (
    <section className="card">
      <h1 style={{ marginTop: 0 }}>Your dashboard</h1>
      <p>You are not signed in.</p>
      <Link href="/login" className="btn">
        Sign in
      </Link>
    </section>
  );
}

export default async function DashboardPage() {
  const session = await getSession();
  if (!session) return <SignInPrompt />;

  return (
    <div>
      <h1>Your dashboard</h1>
      <p>Consented government services for your digital identity.</p>
      <div className="card-grid">
        {DEPARTMENTS.map((d) => (
          <DeptCard key={d.id} dept={d} />
        ))}
      </div>
      <section className="card">
        <h2 style={{ marginTop: 0 }}>Your data, your control</h2>
        <p>
          Manage which departments can access data you hold with another department, and review
          your full audit trail.
        </p>
        <p>
          <Link href="/consent" className="btn btn--small">
            Manage consent
          </Link>{" "}
          <Link href="/audit" className="btn btn--small btn--ghost">
            View audit trail
          </Link>
        </p>
      </section>
    </div>
  );
}
