import Link from "next/link";
import DisputeForm from "./form";
import { fetchMojData } from "../actions";

export const metadata = { title: "Disputes Tribunal — My Gov NZ" };

export default async function MojDisputesPage() {
  const data = await fetchMojData(["moj:disputes"]);
  const disputes = data?.disputes ?? [];

  return (
    <main>
      <Link href="/dept/moj">← Back to Ministry of Justice</Link>
      <h1>Disputes Tribunal</h1>

      <section>
        <h2>Your claims</h2>
        {disputes.length === 0 ? (
          <p>No Disputes Tribunal claims on record.</p>
        ) : (
          <ul>
            {disputes.map((d) => (
              <li key={d.disputeNumber}>
                <strong>{d.disputeNumber}</strong> — {d.claimType} ({d.status}): {d.description}
                {d.amountClaimed != null ? ` — $${d.amountClaimed} claimed` : ""}
                {d.hearingDate ? <><br /><small>Hearing date: {d.hearingDate}</small></> : null}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>File a new claim</h2>
        <DisputeForm />
      </section>
    </main>
  );
}
