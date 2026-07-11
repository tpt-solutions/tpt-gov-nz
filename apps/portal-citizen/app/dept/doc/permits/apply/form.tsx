"use client";

import { useState } from "react";
import Link from "next/link";
import { submitDocAction } from "../../actions";

export default function ApplyPermitForm() {
  const [activity, setActivity] = useState("");
  const [location, setLocation] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitDocAction("apply-conservation-permit", {
      activity,
      location,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/doc/permits">← Back to permits</Link>
        <h1>Apply for a Conservation Permit</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Application submitted."}</p>
            <Link href="/dept/doc/permits">View permits</Link>
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
      <label htmlFor="activity" style={{ display: "block", marginBottom: "0.5rem" }}>
        Activity
        <input
          id="activity"
          type="text"
          value={activity}
          onChange={(e) => setActivity(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <label htmlFor="location" style={{ display: "block", marginBottom: "0.5rem" }}>
        Location
        <input
          id="location"
          type="text"
          value={location}
          onChange={(e) => setLocation(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || activity.trim().length === 0 || location.trim().length === 0}>
        {submitting ? "Submitting…" : "Apply for permit"}
      </button>
    </form>
  );
}
