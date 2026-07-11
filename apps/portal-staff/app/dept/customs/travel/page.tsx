import Link from "next/link";
import { fetchCustomsDataForCitizen } from "../actions";

export const metadata = { title: "Travel — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CustomsStaffTravelPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Travel — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchCustomsDataForCitizen(did, ["customs:travel"]);
  const travel = data?.travel;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/customs?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Travel — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load travel record.</p>}
      {data && !travel && <p>No travel record on file.</p>}

      {travel && (
        <ul>
          <li>
            <strong>Passport {travel.passportNumber}</strong> — last arrival {travel.lastArrival} at {travel.arrivalPort}
            <br />
            <small>
              {travel.frequentTraveller ? "Frequent traveller" : "Not a frequent traveller"}
            </small>
          </li>
        </ul>
      )}
    </main>
  );
}
