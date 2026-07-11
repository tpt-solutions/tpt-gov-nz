import Link from "next/link";
import RequestExportForm from "./form";

export const metadata = { title: "Request Stats NZ Data Export — My Gov NZ" };

export default function StatsnzRequestExportPage() {
  return (
    <main>
      <Link href="/dept/statsnz/census">← Back to census</Link>
      <h1>Request a Data Export</h1>
      <p>Request an export of your Statistics New Zealand data for a stated purpose.</p>
      <RequestExportForm />
    </main>
  );
}
