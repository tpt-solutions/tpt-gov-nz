"use client";

import { useState } from "react";
import Link from "next/link";
import { submitPoliceAction } from "../actions";

const REPORT_TYPES = ["theft", "incident", "lost-property"] as const;

export default function ReportForm() {
  const [reportType, setReportType] = useState<string>("theft");
  const [description, setDescription] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitPoliceAction("file-report", { reportType, description });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/police/reports">← Back to reports</Link>
        <h1>File a Report</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Report filed."}</p>
            <Link href="/dept/police/reports">View reports</Link>
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
      <label htmlFor="reportType" style={{ display: "block", marginBottom: "0.5rem" }}>
        Report type
        <select
          id="reportType"
          value={reportType}
          onChange={(e) => setReportType(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem" }}
        >
          {REPORT_TYPES.map((t) => (
            <option key={t} value={t}>
              {t}
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
        {submitting ? "Filing…" : "File report"}
      </button>
    </form>
  );
}
