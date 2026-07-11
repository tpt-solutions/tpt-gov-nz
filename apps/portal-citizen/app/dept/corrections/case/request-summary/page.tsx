import Link from "next/link";
import RequestSummaryForm from "./form";

export const metadata = { title: "Request Sentence Summary — My Gov NZ" };

export default function RequestSummaryPage() {
  return (
    <main>
      <Link href="/dept/corrections/case">← Back to cases</Link>
      <h1>Request a Sentence Summary</h1>
      <p>Request a written summary of your sentence for a specific purpose.</p>
      <RequestSummaryForm />
    </main>
  );
}
