import Link from "next/link";
import { getSession } from "@/app/lib/session";
import { DEPARTMENTS } from "@/app/lib/config";
import { listGrants } from "@/app/lib/consent";
import ConsentManager from "@/app/components/ConsentManager";

export const metadata = { title: "Consent" };

function Prompt() {
  return (
    <section className="card">
      <h1 style={{ marginTop: 0 }}>Manage consent</h1>
      <p>You are not signed in.</p>
      <Link href="/login" className="btn">
        Sign in
      </Link>
    </section>
  );
}

export default async function ConsentPage() {
  const session = await getSession();
  if (!session) return <Prompt />;

  const grants = await listGrants();
  const depts = DEPARTMENTS.map((d) => ({ id: d.id, name: d.name, shortName: d.shortName }));

  return (
    <div>
      <h1>Manage data-sharing consent</h1>
      <p className="alert alert--info">
        Grant a government department permission to access data you hold with another department.
        Every grant is signed and recorded in your audit trail.
      </p>
      {grants.length > 0 && (
        <p>
          <strong>{grants.length}</strong> active consent{grants.length === 1 ? "" : "s"}.
        </p>
      )}
      <ConsentManager grants={grants} depts={depts} />
    </div>
  );
}
