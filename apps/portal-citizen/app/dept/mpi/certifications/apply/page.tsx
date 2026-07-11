import Link from "next/link";
import MpiExportCertificateForm from "./form";

export const metadata = { title: "Apply for Export Certificate — MPI — My Gov NZ" };

export default function MpiApplyExportCertificatePage() {
  return (
    <main>
      <Link href="/dept/mpi/certifications">← Back to certifications</Link>
      <h1>Apply for an Export Certificate</h1>

      <section>
        <h2>Product details</h2>
        <MpiExportCertificateForm />
      </section>
    </main>
  );
}
