import Link from "next/link";
import { fetchStaffCase } from "../../lib/staff-data";
import { listCaseNotes } from "../../lib/case-notes";
import { listReferralsForCitizen } from "../../lib/referrals";
import { submitCaseNote, submitReferral } from "../../lib/case-actions";
import { getStaffSession } from "../../lib/session";
import { STAFF_DEPARTMENTS, type StaffDeptId } from "../../lib/config";
import type {
  IRDDataBundle,
  WINZDataBundle,
  MOHDataBundle,
  DIADataBundle,
  NZTADataBundle,
  ACCDataBundle,
} from "@tpt/gov-schema";

export const metadata = { title: "Case File — Staff — My Gov NZ" };

type Params = { did: string };

function money(n: number | string | undefined): string {
  if (n === undefined) return "—";
  return `$${Number(n).toLocaleString()}`;
}

function DeptSummary({ dept, data }: { dept: StaffDeptId; data: unknown }) {
  if (dept === "ird") {
    const d = data as IRDDataBundle;
    return (
      <dl>
        <dt>IRD number</dt>
        <dd>…{d.irdNumber?.slice(-3)}</dd>
        {d.currentTaxYear && (
          <>
            <dt>Tax {d.currentTaxYear.assessmentYear}</dt>
            <dd>
              Income {money(d.currentTaxYear.totalIncome)} · Refund {money(d.currentTaxYear.taxRefundDue)} · Owing{" "}
              {money(d.currentTaxYear.taxOwing)}
            </dd>
          </>
        )}
        <dt>KiwiSaver</dt>
        <dd>
          {d.kiwiSaver?.membershipStatus ?? "—"}
          {d.kiwiSaver?.contributionRate ? ` (${d.kiwiSaver.contributionRate}%)` : ""}
        </dd>
        <dt>Working for Families</dt>
        <dd>{d.workingForFamilies?.eligible ? "Receiving" : "Not eligible"}</dd>
      </dl>
    );
  }
  if (dept === "winz") {
    const d = data as WINZDataBundle;
    return (
      <dl>
        <dt>Active benefits</dt>
        <dd>{d.activeBenefits?.length ?? 0}</dd>
        <dt>Weekly total</dt>
        <dd>{money(d.totalWeeklyPayment)}</dd>
        {d.caseManagerName && (
          <>
            <dt>Case manager</dt>
            <dd>{d.caseManagerName}</dd>
          </>
        )}
      </dl>
    );
  }
  if (dept === "moh") {
    const d = data as MOHDataBundle;
    return (
      <dl>
        <dt>NHI</dt>
        <dd>{d.nhiNumber ?? "—"}</dd>
        <dt>Enrolled GP</dt>
        <dd>{d.enrolledGP?.practiceName ?? "—"}</dd>
        <dt>Prescriptions</dt>
        <dd>{d.activePrescriptions?.length ?? 0}</dd>
        <dt>Upcoming appointments</dt>
        <dd>{d.upcomingAppointments?.length ?? 0}</dd>
      </dl>
    );
  }
  if (dept === "dia") {
    const d = data as DIADataBundle;
    return (
      <dl>
        <dt>Passport</dt>
        <dd>{d.passport?.passportNumber ?? "—"} (expires {d.passport?.expiryDate ?? "—"})</dd>
        <dt>Citizenship</dt>
        <dd>{d.citizenship?.status ?? "—"}</dd>
      </dl>
    );
  }
  if (dept === "nzta") {
    const d = data as NZTADataBundle;
    return (
      <dl>
        <dt>Driver licence</dt>
        <dd>
          {d.driverLicence?.licenceNumber ?? "—"} ({d.driverLicence?.licenceClass ?? "—"}, expires{" "}
          {d.driverLicence?.expiryDate ?? "—"})
        </dd>
        <dt>Vehicles</dt>
        <dd>{d.vehicles?.length ?? 0}</dd>
      </dl>
    );
  }
  if (dept === "acc") {
    const d = data as ACCDataBundle;
    return (
      <dl>
        <dt>Client number</dt>
        <dd>{d.clientNumber ?? "—"}</dd>
        <dt>Open claims</dt>
        <dd>{d.claims?.filter((c) => c.status === "open").length ?? 0}</dd>
        <dt>Entitlements</dt>
        <dd>{d.entitlements?.hasEntitlement ? d.entitlements.type : "None"}</dd>
      </dl>
    );
  }
  return null;
}

export default async function CaseFilePage({ params }: { params: Promise<Params> }) {
  const { did } = await params;
  const session = await getStaffSession();
  const [results, notes, referrals] = await Promise.all([
    fetchStaffCase(did),
    listCaseNotes(did),
    listReferralsForCitizen(did),
  ]);

  const consented = results.filter((r) => r.consentGranted);
  const locked = results.filter((r) => !r.consentGranted);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/citizens">← Back to citizen search</Link>
      <h1>Cross-Department Case File</h1>
      <p>
        <strong>Citizen:</strong> <code>{did}</code>
      </p>
      <p>
        <em>Read-only view. No actions can be taken against citizen records from this screen.</em>
      </p>

      <section>
        <h2>Consented departments ({consented.length})</h2>
        {consented.length === 0 && (
          <p>
            No consent grants are active for this citizen. Request a consent grant before any
            records can be displayed.
          </p>
        )}
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(auto-fill, minmax(18rem, 1fr))",
            gap: "1rem",
          }}
        >
          {consented.map((r) => {
            const meta = STAFF_DEPARTMENTS.find((d) => d.id === r.dept)!;
            return (
              <article key={r.dept} style={{ border: "1px solid #ccc", padding: "1rem" }}>
                <h3>
                  {meta.shortName} — {meta.name}
                </h3>
                {r.data ? (
                  <DeptSummary dept={r.dept} data={r.data} />
                ) : (
                  <p>Unable to load {meta.shortName} information for this citizen.</p>
                )}
                <Link href={`/dept/${r.dept}?did=${encodeURIComponent(did)}`}>
                  Open full {meta.shortName} record →
                </Link>
              </article>
            );
          })}
        </div>
      </section>

      {locked.length > 0 && (
        <section>
          <h2>Consent required ({locked.length})</h2>
          <p>The citizen has not granted case-worker access to the following departments:</p>
          <ul>
            {locked.map((r) => (
              <li key={r.dept}>{STAFF_DEPARTMENTS.find((d) => d.id === r.dept)!.name}</li>
            ))}
          </ul>
        </section>
      )}

      <section style={{ borderTop: "2px solid #ccc", marginTop: "1.5rem", paddingTop: "1rem" }}>
        <h2>Case notes</h2>
        {notes.length === 0 && <p>No case notes yet.</p>}
        <ul>
          {notes.map((n) => (
            <li key={n.id} style={{ marginBottom: "0.5rem" }}>
              <strong>{n.author}</strong> · {new Date(n.createdAt).toLocaleString()}
              <br />
              {n.note}
            </li>
          ))}
        </ul>
        <form action={submitCaseNote.bind(null, did)} method="post" style={{ maxWidth: "32rem" }}>
          <label htmlFor="note">Add a case note</label>
          <textarea
            id="note"
            name="note"
            rows={3}
            required
            style={{ display: "block", width: "100%", margin: "0.5rem 0" }}
          />
          <button type="submit">Save note</button>
        </form>
      </section>

      <section style={{ borderTop: "2px solid #ccc", marginTop: "1.5rem", paddingTop: "1rem" }}>
        <h2>Referrals</h2>
        {referrals.length === 0 && <p>No referrals for this citizen.</p>}
        <ul>
          {referrals.map((r) => {
            const to = STAFF_DEPARTMENTS.find((d) => d.id === r.toDept)!;
            const from = STAFF_DEPARTMENTS.find((d) => d.id === r.fromDept)!;
            return (
              <li key={r.id} style={{ marginBottom: "0.5rem" }}>
                <strong>
                  {from.shortName} → {to.shortName}
                </strong>{" "}
                ({r.status}) · {new Date(r.createdAt).toLocaleString()}
                <br />
                {r.reason}
              </li>
            );
          })}
        </ul>
        <form action={submitReferral.bind(null, did)} method="post" style={{ maxWidth: "32rem" }}>
          <label htmlFor="fromDept">Referring department</label>
          <select id="fromDept" name="fromDept" defaultValue="ird" style={{ display: "block", margin: "0.25rem 0" }}>
            {STAFF_DEPARTMENTS.map((d) => (
              <option key={d.id} value={d.id}>
                {d.name}
              </option>
            ))}
          </select>
          <label htmlFor="toDept">Referral to</label>
          <select id="toDept" name="toDept" defaultValue="winz" style={{ display: "block", margin: "0.25rem 0" }}>
            {STAFF_DEPARTMENTS.map((d) => (
              <option key={d.id} value={d.id}>
                {d.name}
              </option>
            ))}
          </select>
          <label htmlFor="reason">Reason</label>
          <textarea
            id="reason"
            name="reason"
            rows={3}
            required
            style={{ display: "block", width: "100%", margin: "0.5rem 0" }}
          />
          <button type="submit">Create referral</button>
        </form>
      </section>

      {session?.displayName && (
        <p style={{ color: "#555", fontSize: "0.8rem", marginTop: "1rem" }}>
          Acting as {session.displayName}
          {session.demo ? " (demo)" : ""}.
        </p>
      )}
    </main>
  );
}
