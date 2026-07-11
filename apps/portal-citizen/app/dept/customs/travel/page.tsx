import Link from "next/link";
import { fetchCustomsData } from "../actions";

export const metadata = { title: "Travel — My Gov NZ" };

export default async function CustomsTravelPage() {
  const data = await fetchCustomsData(["customs:travel"]);
  const travel = data?.travel;

  return (
    <main>
      <Link href="/dept/customs">← Back to customs</Link>
      <h1>Travel</h1>

      {!travel ? (
        <p>No travel record on file.</p>
      ) : (
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
