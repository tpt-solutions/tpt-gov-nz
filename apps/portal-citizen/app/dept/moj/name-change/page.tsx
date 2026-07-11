import Link from "next/link";
import NameChangeForm from "./form";

export const metadata = { title: "Name Change Request — My Gov NZ" };

export default function MojNameChangePage() {
  return (
    <main>
      <Link href="/dept/moj">← Back to Ministry of Justice</Link>
      <h1>Request a Name Change</h1>
      <NameChangeForm />
    </main>
  );
}
