"use client";

import { useState } from "react";
import Link from "next/link";
import { submitTpkAction } from "../../actions";

export default function ApplyForFundingForm() {
  const [programme, setProgramme] = useState("");
  const [purpose, setPurpose] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitTpkAction("apply-funding", { programme, purpose });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/tpk/funding">← Back to funding</Link>
        <h1>Apply for Funding</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Application submitted."}</p>
            <Link href="/dept/tpk/funding">View funding</Link>
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
    <form onSubmit={handleSubmit}>
      <label htmlFor="programme" style={{ display: "block", marginBottom: "0.5rem" }}>
        Programme
        <input
          id="programme"
          type="text"
          value={programme}
          onChange={(e) => setProgramme(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <label htmlFor="purpose" style={{ display: "block", marginBottom: "0.5rem" }}>
        Purpose
        <textarea
          id="purpose"
          value={purpose}
          onChange={(e) => setPurpose(e.target.value)}
          rows={4}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || programme.trim().length === 0 || purpose.trim().length === 0}>
        {submitting ? "Submitting…" : "Apply for funding"}
      </button>
    </form>
  );
}
