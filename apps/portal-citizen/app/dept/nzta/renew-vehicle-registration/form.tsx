"use client";

import { useState } from "react";
import Link from "next/link";
import { submitNztaAction } from "../actions";

export default function RenewVehicleRegistrationForm() {
  const [registration, setRegistration] = useState("");
  const [months, setMonths] = useState(12);
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitNztaAction("renew-vehicle-registration", {
      registration,
      months,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/nzta">← Back to Transport</Link>
        <h1>Renew Vehicle Registration</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Registration renewed."}</p>
            <Link href="/dept/nzta">View Transport</Link>
          </>
        ) : (
          <>
            <p style={{ color: "red" }}>{result.message ?? "Something went wrong."}</p>
            <button type="button" onClick={() => setResult(null)}>Try again</button>
          </>
        )}
      </main>
    );
  }

  return (
    <main>
      <Link href="/dept/nzta">← Back to Transport</Link>
      <h1>Renew Vehicle Registration</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="registration" style={{ display: "block", marginBottom: "0.5rem" }}>
          Registration
          <input
            id="registration"
            type="text"
            value={registration}
            onChange={(e) => setRegistration(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <label htmlFor="months" style={{ display: "block", marginBottom: "0.5rem" }}>
          Months to renew (1–24)
          <input
            id="months"
            type="number"
            min={1}
            max={24}
            value={months}
            onChange={(e) => setMonths(Number(e.target.value))}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "12rem" }}
          />
        </label>
        <button type="submit" disabled={submitting || registration.trim().length === 0}>
          {submitting ? "Submitting…" : "Renew registration"}
        </button>
      </form>
    </main>
  );
}
