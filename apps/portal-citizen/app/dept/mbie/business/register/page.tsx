"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMbieAction } from "../../actions";

const ENTITY_TYPES = ["company", "sole-trader", "partnership", "trust"] as const;

export default function RegisterBusinessPage() {
  const [nzbn, setNzbn] = useState("");
  const [entityName, setEntityName] = useState("");
  const [entityType, setEntityType] = useState<string>("company");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMbieAction("register-business", {
      nzbn,
      entityName,
      entityType,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/mbie/business">← Back to business registrations</Link>
        <h1>Register a Business</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Business registered."}</p>
            <Link href="/dept/mbie/business">View business registrations</Link>
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
      <Link href="/dept/mbie/business">← Back to business registrations</Link>
      <h1>Register a Business</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="nzbn" style={{ display: "block", marginBottom: "0.5rem" }}>
          NZBN
          <input
            id="nzbn"
            type="text"
            value={nzbn}
            onChange={(e) => setNzbn(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem" }}
          />
        </label>
        <label htmlFor="entityName" style={{ display: "block", marginBottom: "0.5rem" }}>
          Entity name
          <input
            id="entityName"
            type="text"
            value={entityName}
            onChange={(e) => setEntityName(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <label htmlFor="entityType" style={{ display: "block", marginBottom: "0.5rem" }}>
          Entity type
          <select
            id="entityType"
            value={entityType}
            onChange={(e) => setEntityType(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem" }}
          >
            {ENTITY_TYPES.map((t) => (
              <option key={t} value={t}>
                {t}
              </option>
            ))}
          </select>
        </label>
        <button type="submit" disabled={submitting || nzbn.trim().length === 0 || entityName.trim().length === 0}>
          {submitting ? "Submitting…" : "Register business"}
        </button>
      </form>
    </main>
  );
}
