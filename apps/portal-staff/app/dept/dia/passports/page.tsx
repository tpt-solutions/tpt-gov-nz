import Link from "next/link";
import { fetchDiaDataForCitizen } from "../actions";

export const metadata = { title: "Passport — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DiaStaffPassportsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Passport — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchDiaDataForCitizen(did, ["dia:passport"]);
  const passport = data?.passport;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/dia?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Passport — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load passport.</p>}
      {data && !passport && <p>No passport on record.</p>}

      {passport && (
        <dl>
          <dt>Passport number</dt>
          <dd>••••{passport.passportNumber.slice(-4)}</dd>
          <dt>Expiry</dt>
          <dd>{passport.expiryDate}</dd>
          <dt>Renewable</dt>
          <dd>{passport.renewable ? "Yes" : "No"}</dd>
        </dl>
      )}
    </main>
  );
}
