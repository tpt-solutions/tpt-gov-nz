"use client";

import { useState } from "react";
import Link from "next/link";
import { submitWinzAction } from "./actions";

export default function ActionForm({
  actionType,
  title,
  description,
  inputLabel,
  paramKey,
  backHref,
}: {
  actionType: string;
  title: string;
  description: string;
  inputLabel: string;
  paramKey: "reason" | "notes";
  backHref: string;
}) {
  const [value, setValue] = useState("");
  const [channel, setChannel] = useState("phone");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitWinzAction(actionType, { [paramKey]: value, channel });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href={backHref}>← Back</Link>
        <h1>{title}</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Done."}</p>
            <Link href="/dept/winz">View Work and Income</Link>
          </>
        ) : (
          <>
            <p style={{ color: "red" }}>{result.message ?? "Something went wrong. Please try again."}</p>
            <button type="button" onClick={() => setResult(null)}>Try again</button>
          </>
        )}
      </main>
    );
  }

  return (
    <main>
      <Link href={backHref}>← Back</Link>
      <h1>{title}</h1>
      <p>{description}</p>

      <form onSubmit={handleSubmit}>
        <label htmlFor="winz-action-input" style={{ display: "block", marginBottom: "0.5rem" }}>
          {inputLabel}
          <textarea
            id="winz-action-input"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            rows={4}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>

        <label htmlFor="winz-action-channel" style={{ display: "block", marginBottom: "1rem" }}>
          Preferred contact method
          <select
            id="winz-action-channel"
            value={channel}
            onChange={(e) => setChannel(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem" }}
          >
            <option value="phone">Phone</option>
            <option value="email">Email</option>
            <option value="in-person">In person</option>
          </select>
        </label>

        <button type="submit" disabled={submitting || value.trim().length === 0}>
          {submitting ? "Submitting…" : "Submit"}
        </button>
      </form>
    </main>
  );
}
