import { Suspense } from "react";
import Link from "next/link";
import FileGstReturnForm from "./form";

export const metadata = { title: "File GST Return — IRD — My Gov NZ" };

export default function FileGstReturnPage() {
  return (
    <main>
      <Link href="/dept/ird/gst">← Back to GST</Link>
      <h1>File GST Return</h1>
      <Suspense fallback={<p>Loading…</p>}>
        <FileGstReturnForm />
      </Suspense>
    </main>
  );
}
