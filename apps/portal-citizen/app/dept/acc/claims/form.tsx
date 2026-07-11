"use client";

import { useState } from "react";
import Link from "next/link";
import { submitAccAction } from "../actions";

const CLAIM_TYPES = ["work", "non-work", "treatment"] as const;

export default function ClaimsForm() {
  const [claimType, setClaimType] = useState<string>("work");
  const [injuryDate, setInjuryDate] = useState("");
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitAccAction("lodge-claim", { claimType, injuryDate, description });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/acc/claims">← Back to claims</Link>
        <h1>Lodge a Claim</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Claim lodged."}</p>
            <Link href="/dept/acc/claims">View claims</Link>
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
      <label htmlFor="claimType" style={{ display: "block", marginBottom: "0.5rem" }}>
        Claim type
        <select
          id="claimType"
          value={claimType}
          onChange={(e) => setClaimType(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        >
          {CLAIM_TYPES.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>
      </label>
      <label htmlFor="injuryDate" style={{ display: "block", marginBottom: "0.5rem" }}>
        Injury date
        <input
          id="injuryDate"
          type="date"
          value={injuryDate}
          onChange={(e) => setInjuryDate(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        />
      </label>
      <label htmlFor="description" style={{ display: "block", marginBottom: "0.5rem" }}>
        Description
        <textarea
          id="description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          rows={4}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || description.trim().length === 0}>
        {submitting ? "Submitting…" : "Lodge claim"}
      </button>
    </form>
  );
}
