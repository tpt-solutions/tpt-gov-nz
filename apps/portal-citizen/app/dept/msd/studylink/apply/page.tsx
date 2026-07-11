"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMsdAction } from "../actions";

export default function StudyLinkForm() {
  const [courseOfStudy, setCourseOfStudy] = useState("");
  const [provider, setProvider] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMsdAction("apply-student-allowance", {
      courseOfStudy,
      provider,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/msd/studylink">← Back to StudyLink</Link>
        <h1>Apply for a Student Allowance</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Application submitted."}</p>
            <Link href="/dept/msd/studylink">View StudyLink</Link>
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
      <label htmlFor="courseOfStudy" style={{ display: "block", marginBottom: "0.5rem" }}>
        Course of study
        <input
          id="courseOfStudy"
          type="text"
          value={courseOfStudy}
          onChange={(e) => setCourseOfStudy(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <label htmlFor="provider" style={{ display: "block", marginBottom: "0.5rem" }}>
        Education provider
        <input
          id="provider"
          type="text"
          value={provider}
          onChange={(e) => setProvider(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || courseOfStudy.trim().length === 0 || provider.trim().length === 0}>
        {submitting ? "Submitting…" : "Submit application"}
      </button>
    </form>
  );
}
