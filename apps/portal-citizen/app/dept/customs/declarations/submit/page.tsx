import Link from "next/link";
import DeclarationForm from "./form";

export const metadata = { title: "Submit a Declaration — My Gov NZ" };

export default function CustomsDeclarationSubmitPage() {
  return (
    <main>
      <Link href="/dept/customs/declarations">← Back to declarations</Link>
      <h1>Submit a Traveller Declaration</h1>
      <DeclarationForm />
    </main>
  );
}
