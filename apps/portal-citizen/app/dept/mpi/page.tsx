import Link from "next/link";
import { fetchMpiData } from "./actions";
import MpiAiPrompt from "./ai-prompt";

export const metadata = { title: "MPI — My Gov NZ" };

export default async function MpiOverviewPage() {
  const data = await fetchMpiData(["mpi:registrations", "mpi:certifications"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Primary Industries</h1>
        <p>Unable to load your MPI information. Please grant access to continue.</p>
        <Link href="/consent?dept=mpi">Grant MPI access</Link>
      </main>
    );
  }

  const activeRegistrations =
    data.registrations?.filter((r) => r.status === "registered") ?? [];
  const validCertifications =
    data.certifications?.filter((c) => c.expiresDate >= new Date().toISOString().slice(0, 10)) ?? [];

  return (
    <main>
      <h1>Ministry for Primary Industries</h1>
      <p>MPI id: ••••{data.mpiId.slice(-4)}</p>

      <section>
        <h2>Registrations</h2>
        <p>{activeRegistrations.length} active registration(s).</p>
        <Link href="/dept/mpi/registrations">View registrations →</Link>
      </section>

      <section>
        <h2>Certifications</h2>
        <p>{validCertifications.length} valid certification(s).</p>
        <Link href="/dept/mpi/certifications">View certifications →</Link>
      </section>

      <MpiAiPrompt />
    </main>
  );
}
