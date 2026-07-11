"use client";

import { useState } from "react";
import Link from "next/link";
import { submitLinzAction } from "../../actions";

export default function RequestTitleCopyForm() {
  const [titleNumber, setTitleNumber] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitLinzAction("request-title-copy", { titleNumber });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/linz/titles">← Back to titles</Link>
        <h1>Request a Title Copy</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Request submitted."}</p>
            <Link href="/dept/linz/titles">View titles</Link>
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
      <label htmlFor="titleNumber" style={{ display: "block", marginBottom: "0.5rem" }}>
        Title number
        <input
          id="titleNumber"
          type="text"
          value={titleNumber}
          onChange={(e) => setTitleNumber(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || titleNumber.trim().length === 0}>
        {submitting ? "Submitting…" : "Request copy"}
      </button>
    </form>
  );
}
