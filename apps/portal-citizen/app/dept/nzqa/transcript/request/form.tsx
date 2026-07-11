"use client";

import { useState } from "react";
import Link from "next/link";
import { submitNzqaAction } from "../../actions";

export default function RequestTranscriptForm() {
  const [purpose, setPurpose] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitNzqaAction("request-transcript", { purpose });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/nzqa/transcript">← Back to transcript</Link>
        <h1>Request a Transcript</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Transcript requested."}</p>
            <Link href="/dept/nzqa/transcript">View transcript</Link>
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
        {submitting ? "Submitting…" : "Request transcript"}
      </button>
    </form>
  );
}
