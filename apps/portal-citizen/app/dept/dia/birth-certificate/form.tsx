"use client";

import { useState } from "react";
import Link from "next/link";
import { submitDiaAction } from "../actions";

export default function BirthCertificateForm() {
  const [copies, setCopies] = useState(1);
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitDiaAction("request-birth-certificate", { copies });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/dia">← Back to Internal Affairs</Link>
        <h1>Request Birth Certificate</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Requested."}</p>
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
      <h1>Request Birth Certificate</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="copies" style={{ display: "block", marginBottom: "0.5rem" }}>
          Number of copies (1–10)
          <input
            id="copies"
            type="number"
            min={1}
            max={10}
            value={copies}
            onChange={(e) => setCopies(Number(e.target.value))}
            style={{ display: "block", marginTop: "0.25rem" }}
          />
        </label>
        <button type="submit" disabled={submitting}>
          {submitting ? "Submitting…" : "Request certificate"}
        </button>
      </form>
    </main>
  );
}
