"use client";

import { useState } from "react";
import Link from "next/link";
import { submitHudAction } from "../actions";

const APPLICATION_TYPES = ["public-housing", "emergency-housing", "home-ownership"] as const;

export default function ApplicationForm() {
  const [applicationType, setApplicationType] = useState<string>("public-housing");
  const [bedroomsNeeded, setBedroomsNeeded] = useState("");
  const [reason, setReason] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitHudAction("submit-housing-application", {
      applicationType,
      bedroomsNeeded: bedroomsNeeded ? Number(bedroomsNeeded) : undefined,
      reason,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/hud/applications">← Back to applications</Link>
        <h1>Submit an Application</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Application submitted."}</p>
            <Link href="/dept/hud/applications">View applications</Link>
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
      <label htmlFor="applicationType" style={{ display: "block", marginBottom: "0.5rem" }}>
        Application type
        <select
          id="applicationType"
          value={applicationType}
          onChange={(e) => setApplicationType(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        >
          {APPLICATION_TYPES.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>
      </label>
      <label htmlFor="bedroomsNeeded" style={{ display: "block", marginBottom: "0.5rem" }}>
        Bedrooms needed (optional)
        <input
          id="bedroomsNeeded"
          type="number"
          value={bedroomsNeeded}
          onChange={(e) => setBedroomsNeeded(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        />
      </label>
      <label htmlFor="reason" style={{ display: "block", marginBottom: "0.5rem" }}>
        Reason
        <textarea
          id="reason"
          value={reason}
          onChange={(e) => setReason(e.target.value)}
          rows={4}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || reason.trim().length === 0}>
        {submitting ? "Submitting…" : "Submit application"}
      </button>
    </form>
  );
}
