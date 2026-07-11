"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMojAction } from "../actions";

export default function PayFineForm() {
  const [fineNumber, setFineNumber] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMojAction("pay-fine", { fineNumber });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/moj/fines">← Back to fines</Link>
        <h1>Pay a Fine</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Fine paid."}</p>
            <Link href="/dept/moj/fines">View fines</Link>
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
      <label htmlFor="fineNumber" style={{ display: "block", marginBottom: "0.5rem" }}>
        Fine number
        <input
          id="fineNumber"
          type="text"
          value={fineNumber}
          onChange={(e) => setFineNumber(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || fineNumber.trim().length === 0}>
        {submitting ? "Paying…" : "Pay fine"}
      </button>
    </form>
  );
}
