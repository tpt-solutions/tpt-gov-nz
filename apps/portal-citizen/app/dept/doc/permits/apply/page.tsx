import Link from "next/link";
import { submitDocAction } from "../actions";

export const metadata = { title: "Apply for a Conservation Permit — My Gov NZ" };

export default async function DocApplyPermitPage() {
  return (
    <main>
      <Link href="/dept/doc/permits">← Back to permits</Link>
      <h1>Apply for a Conservation Permit</h1>
      <p>Complete the form below to apply for a new conservation permit.</p>
    </main>
  );
}
