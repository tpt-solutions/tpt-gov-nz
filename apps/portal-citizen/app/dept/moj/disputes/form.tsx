"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMojAction } from "../actions";

const CLAIM_TYPES = ["consumer", "tenancy", "debt"] as const;

export default function DisputeForm() {
  const [claimType, setClaimType] = useState<string>("consumer");
  const [amountClaimed, setAmountClaimed] = useState("");
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMojAction("file-dispute-claim", {
      claimType,
      amountClaimed: amountClaimed ? Number(amountClaimed) : undefined,
      description,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/moj/disputes">← Back to disputes</Link>
        <h1>File a Claim</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Claim filed."}</p>
            <Link href="/dept/moj/disputes">View disputes</Link>
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
      <label htmlFor="amountClaimed" style={{ display: "block", marginBottom: "0.5rem" }}>
        Amount claimed (optional)
        <input
          id="amountClaimed"
          type="number"
          value={amountClaimed}
          onChange={(e) => setAmountClaimed(e.target.value)}
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
        {submitting ? "Filing…" : "File claim"}
      </button>
    </form>
  );
}
