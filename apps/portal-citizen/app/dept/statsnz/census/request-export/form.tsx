"use client";

import { useState } from "react";
import Link from "next/link";
import { submitStatsnzAction } from "../../actions";

export default function RequestExportForm() {
  const [purpose, setPurpose] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitStatsnzAction("request-data-export", { purpose });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/statsnz/census">← Back to census</Link>
        <h1>Request a Data Export</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Export requested."}</p>
            <Link href="/dept/statsnz/census">View census</Link>
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
        {submitting ? "Submitting…" : "Request export"}
      </button>
    </form>
  );
}
