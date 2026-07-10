import Link from "next/link";
import BabyLifeEventForm from "./form";

export const metadata = { title: "Life event — Working for Families — My Gov NZ" };

export default function BabyLifeEventPage() {
  return (
    <main>
      <Link href="/dept/ird/working-for-families">← Back to Working for Families</Link>
      <h1>Life event: I just had a baby</h1>
      <BabyLifeEventForm />
    </main>
  );
}
