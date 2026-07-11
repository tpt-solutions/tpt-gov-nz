import Link from "next/link";

export const metadata = { title: "Case Worker Home — Staff — My Gov NZ" };

export default function StaffHomePage() {
  return (
    <main style={{ padding: "1rem" }}>
      <h1>Case Worker Home</h1>
      <p>Search for a citizen by their government DID to open their cross-department case file.</p>

      <form action="/dept/ird" method="get" style={{ margin: "1rem 0" }}>
        <label htmlFor="did">Citizen DID</label>
        <input
          id="did"
          name="did"
          placeholder="did:gov:nz:test-citizen-001"
          style={{ display: "block", margin: "0.5rem 0", width: "100%", maxWidth: "32rem" }}
        />
        <button type="submit">Open IRD case file</button>
      </form>

      <nav>
        <Link href="/dept/ird">Inland Revenue (IRD)</Link>
        {" · "}
        <Link href="/dept/nzta">Waka Kotahi NZTA</Link>
        {" · "}
        <Link href="/dept/acc">ACC</Link>
      </nav>
    </main>
  );
}
