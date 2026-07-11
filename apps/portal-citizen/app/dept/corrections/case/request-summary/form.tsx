"use client";

import { useState } from "react";
import Link from "next/link";
import { submitCorrectionsAction } from "../../actions";

export default function RequestSummaryForm() {
  const [purpose, setPurpose] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitCorrectionsAction("request-sentence-summary", { purpose });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/corrections/case/request-summary">← Back to request</Link>
        <h1>Request a Sentence Summary</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Request submitted."}</p>
            <Link href="/dept/corrections/case">View cases</Link>
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
      <button type="submit" disabled={submitting || purpose.trim().length === 0}>
        {submitting ? "Submitting…" : "Request summary"}
      </button>
    </form>
  );
}
