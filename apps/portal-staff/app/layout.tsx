import type { Metadata } from "next";
import Link from "next/link";
import { getStaffSession } from "./lib/session";
import { logoutStaff } from "./lib/auth-actions";
import { STAFF_CONFIG } from "./lib/config";

export const metadata: Metadata = {
  title: "My Gov NZ — Staff",
  description: "Case-worker portal for New Zealand government services",
};

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  const session = await getStaffSession();

  return (
    <html lang="en-NZ">
      <body>
        <header
          style={{
            borderBottom: "2px solid #ccc",
            padding: "0.75rem 1rem",
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            flexWrap: "wrap",
            gap: "0.5rem",
          }}
        >
          <div>
            <strong>My Gov NZ — Staff Portal</strong>
            <span style={{ marginLeft: "1rem", color: "#555", fontSize: "0.85rem" }}>
              Read-only case worker view
            </span>
          </div>
          <nav style={{ display: "flex", gap: "1rem", alignItems: "center", fontSize: "0.9rem" }}>
            <Link href="/citizens">Citizen search</Link>
            <Link href="/referrals">Referrals</Link>
            {session ? (
              <>
                <span style={{ color: "#555" }}>
                  {session.displayName}
                  {session.demo ? " (demo)" : ""}
                </span>
                <form action={logoutStaff} style={{ display: "inline" }}>
                  <button type="submit" style={{ cursor: "pointer" }}>
                    Sign out
                  </button>
                </form>
              </>
            ) : (
              <Link href="/login">Sign in</Link>
            )}
          </nav>
        </header>
        {STAFF_CONFIG.demoMode && (
          <div style={{ background: "#fff7d6", padding: "0.4rem 1rem", fontSize: "0.85rem" }}>
            Demo mode — data is fictional. Consent is pre-granted for the demo citizen.
          </div>
        )}
        {children}
      </body>
    </html>
  );
}
