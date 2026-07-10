"use client";

import { useState } from "react";
import Link from "next/link";
import { submitDiaAction } from "../actions";

export default function PassportRenewalForm() {
  const [reason, setReason] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitDiaAction("request-passport-renewal", { reason });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/dia">← Back to Internal Affairs</Link>
        <h1>Renew Passport</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Renewal requested."}</p>
            <Link href="/dept/dia">View Internal Affairs</Link>
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
      <Link href="/dept/dia">← Back to Internal Affairs</Link>
      <h1>Renew Passport</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="reason" style={{ display: "block", marginBottom: "0.5rem" }}>
          Reason for renewal
          <textarea
            id="reason"
            value={reason}
            onChange={(e) => setReason(e.target.value)}
            rows={4}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <button type="submit" disabled={submitting || reason.trim().length === 0}>
          {submitting ? "Submitting…" : "Request renewal"}
        </button>
      </form>
    </main>
  );
}
