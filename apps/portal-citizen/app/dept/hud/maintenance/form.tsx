"use client";

import { useState } from "react";
import Link from "next/link";
import { submitHudAction } from "../actions";

const CATEGORIES = ["plumbing", "electrical", "heating", "structural", "other"] as const;

export default function MaintenanceForm() {
  const [category, setCategory] = useState<string>("plumbing");
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitHudAction("request-maintenance", { category, description });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/hud/maintenance">← Back to maintenance</Link>
        <h1>Log a Maintenance Request</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Request logged."}</p>
            <Link href="/dept/hud/maintenance">View requests</Link>
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
      <label htmlFor="category" style={{ display: "block", marginBottom: "0.5rem" }}>
        Category
        <select
          id="category"
          value={category}
          onChange={(e) => setCategory(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        >
          {CATEGORIES.map((c) => (
            <option key={c} value={c}>
              {c}
            </option>
          ))}
        </select>
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
        {submitting ? "Logging…" : "Log request"}
      </button>
    </form>
  );
}
