"use client";

import { Suspense, useState } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import { loginStaffDemo, loginStaff } from "../lib/auth-actions";
import { STAFF_CONFIG } from "../lib/config";

function StaffLoginForm() {
  const params = useSearchParams();
  const error = params.get("error");
  const [staffId, setStaffId] = useState("");
  const [password, setPassword] = useState("");

  return (
    <main style={{ padding: "1rem", maxWidth: "40rem" }}>
      <h1>Staff sign in</h1>
      <p>Case-worker access to the cross-government case management portal.</p>

      {STAFF_CONFIG.demoMode ? (
        <section style={{ border: "1px solid #ccc", padding: "1rem", margin: "1rem 0" }}>
          <h2>Demo mode</h2>
          <p>
            This demo runs on fictional data. Sign in as a case worker to explore the consent-gated
            citizen search and cross-department case view.
          </p>
          <button
            type="button"
            onClick={() => loginStaffDemo()}
            style={{ padding: "0.5rem 1rem", cursor: "pointer" }}
          >
            Sign in as Demo Case Worker
          </button>
        </section>
      ) : (
        <section style={{ border: "1px solid #ccc", padding: "1rem", margin: "1rem 0" }}>
          <h2>Case worker credentials</h2>
          {error === "invalid" && (
            <p style={{ color: "#b00020" }} role="alert">
              Invalid staff ID or password.
            </p>
          )}
          <form action={loginStaff} style={{ display: "grid", gap: "0.5rem" }}>
            <label htmlFor="staffId">Staff ID</label>
            <input
              id="staffId"
              name="staffId"
              value={staffId}
              onChange={(e) => setStaffId(e.target.value)}
              autoComplete="username"
              style={{ maxWidth: "24rem" }}
            />
            <label htmlFor="password">Password</label>
            <input
              id="password"
              name="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoComplete="current-password"
              style={{ maxWidth: "24rem" }}
            />
            <button type="submit" style={{ maxWidth: "12rem", padding: "0.5rem", cursor: "pointer" }}>
              Sign in
            </button>
          </form>
          <p style={{ color: "#555", fontSize: "0.85rem" }}>
            Real sign-in uses a shared passphrase (<code>STAFF_PASSWORD</code>) standing in for a
            RealMe SAML2 lookup (see Production Hardening).
          </p>
        </section>
      )}

      <p>
        <Link href="/">Return to portal home</Link>
      </p>
    </main>
  );
}

export default function StaffLoginPage() {
  return (
    <Suspense fallback={<main style={{ padding: "1rem" }}>Loading…</main>}>
      <StaffLoginForm />
    </Suspense>
  );
}
