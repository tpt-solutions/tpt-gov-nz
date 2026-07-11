import Link from "next/link";
import { getSession, toCitizenIdentityToken } from "@/app/lib/session";
import { listGrants } from "@/app/lib/consent";
import { DEPARTMENTS } from "@/app/lib/config";

export const metadata = { title: "Audit trail" };

function shortDept(id: string): string {
  return DEPARTMENTS.find((d) => d.id === id)?.shortName ?? id;
}

function Prompt() {
  return (
    <section className="card">
      <h1 style={{ marginTop: 0 }}>Your audit trail</h1>
      <p>You are not signed in.</p>
      <Link href="/login" className="btn">
        Sign in
      </Link>
    </section>
  );
}

export default async function AuditPage() {
  const session = await getSession();
  if (!session) return <Prompt />;

  const token = toCitizenIdentityToken(session);
  const grants = await listGrants();

  return (
    <div>
      <h1>Your audit trail</h1>
      <p className="alert alert--info">
        A record of your signed-in session and the data-sharing consents you have granted.
      </p>

      <section className="card">
        <h2 style={{ marginTop: 0 }}>Signed-in session</h2>
        <dl>
          <dt>DID</dt>
          <dd>
            <code>{token.did}</code>
          </dd>
          <dt>Session ID</dt>
          <dd>
            <code>{token.sessionId}</code>
          </dd>
          <dt>Issued</dt>
          <dd>{new Date(token.issuedAt * 1000).toLocaleString("en-NZ")}</dd>
          <dt>Expires</dt>
          <dd>{new Date(token.expiresAt * 1000).toLocaleString("en-NZ")}</dd>
          <dt>Mode</dt>
          <dd>{session.demo ? "Demo" : "Live"}</dd>
        </dl>
      </section>

      <section className="card">
        <h2 style={{ marginTop: 0 }}>Consents granted</h2>
        {grants.length === 0 ? (
          <p>No consents granted yet.</p>
        ) : (
          <ul style={{ paddingLeft: "1.1rem" }}>
            {grants.map((g) => (
              <li key={g.id} style={{ marginBottom: "0.75rem" }}>
                <strong>
                  {shortDept(g.requestingDeptId)} → {shortDept(g.providingDeptId)}
                </strong>{" "}
                <span style={{ color: "var(--muted)" }}>
                  ({g.scopes.length} scope{g.scopes.length === 1 ? "" : "s"})
                </span>
                <br />
                <span style={{ fontSize: "0.85rem", color: "var(--muted)" }}>
                  Issued {new Date(g.issuedAt).toLocaleString("en-NZ")} · expires{" "}
                  {new Date(g.expiresAt).toLocaleString("en-NZ")}
                  {g.demo ? " · demo grant" : ""}
                </span>
              </li>
            ))}
          </ul>
        )}
        <Link href="/consent" className="btn btn--small btn--ghost">
          Manage consent
        </Link>
      </section>
    </div>
  );
}
