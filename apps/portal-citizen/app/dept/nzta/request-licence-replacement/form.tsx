"use client";

import { useState } from "react";
import Link from "next/link";
import { submitNztaAction } from "../actions";

export default function RequestLicenceReplacementForm() {
  const [reason, setReason] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitNztaAction("request-licence-replacement", { reason });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/nzta">← Back to Transport</Link>
        <h1>Replace Driver Licence Card</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Replacement requested."}</p>
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
      <h1>Replace Driver Licence Card</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="reason" style={{ display: "block", marginBottom: "0.5rem" }}>
          Reason for replacement
          <textarea
            id="reason"
            value={reason}
            onChange={(e) => setReason(e.target.value)}
            rows={4}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <button type="submit" disabled={submitting || reason.trim().length === 0}>
          {submitting ? "Submitting…" : "Request replacement"}
        </button>
      </form>
    </main>
  );
}
