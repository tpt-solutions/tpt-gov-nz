import Link from "next/link";
import RequestTranscriptForm from "./form";

export const metadata = { title: "Request NZQA Transcript — My Gov NZ" };

export default function NzqaTranscriptRequestPage() {
  return (
    <main>
      <Link href="/dept/nzqa/transcript">← Back to transcript</Link>
      <h1>Request a Transcript</h1>
      <p>Order an official copy of your Record of Achievement.</p>
      <RequestTranscriptForm />
    </main>
  );
}
