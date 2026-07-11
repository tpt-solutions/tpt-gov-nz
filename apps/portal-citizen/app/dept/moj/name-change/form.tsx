"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMojAction } from "../actions";

export default function NameChangeForm() {
  const [newName, setNewName] = useState("");
  const [reason, setReason] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMojAction("request-name-change", { newName, reason });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Name change requested."}</p>
            <Link href="/dept/moj">Back to Ministry of Justice</Link>
          </>
        ) : (
          <>
            <p style={{ color: "red" }}>{result.message ?? "Something went wrong."}</p>
            <button type="button" onClick={() => setResult(null)}>Try again</button>
          </>
        )}
      </>
    );
  }

  return (
    <form onSubmit={handleSubmit}>
      <label htmlFor="newName" style={{ display: "block", marginBottom: "0.5rem" }}>
        New name
        <input
          id="newName"
          type="text"
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        />
      </label>
      <label htmlFor="reason" style={{ display: "block", marginBottom: "0.5rem" }}>
        Reason
        <textarea
          id="reason"
          value={reason}
          onChange={(e) => setReason(e.target.value)}
          rows={3}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button
        type="submit"
        disabled={submitting || newName.trim().length === 0 || reason.trim().length === 0}
      >
        {submitting ? "Submitting…" : "Submit request"}
      </button>
    </form>
  );
}
