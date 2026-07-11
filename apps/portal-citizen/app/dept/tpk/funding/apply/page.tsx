import Link from "next/link";
import ApplyForFundingForm from "./form";

export const metadata = { title: "Apply for TPK Funding — My Gov NZ" };

export default function TpkApplyFundingPage() {
  return (
    <main>
      <Link href="/dept/tpk/funding">← Back to funding</Link>
      <h1>Apply for Funding</h1>
      <ApplyForFundingForm />
    </main>
  );
}
