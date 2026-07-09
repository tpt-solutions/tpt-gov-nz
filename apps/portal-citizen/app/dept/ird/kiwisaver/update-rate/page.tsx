"use client";

import { useState } from "react";
import Link from "next/link";
import { submitIrdAction } from "../../actions";

const RATE_OPTIONS = [3, 4, 6, 8, 10] as const;

export default function UpdateKiwiSaverRatePage() {
  const [selectedRate, setSelectedRate] = useState<number>(6);
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);

    const res = await submitIrdAction("update-kiwisaver-rate", { newRate: selectedRate });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/ird/kiwisaver">← Back to KiwiSaver</Link>
        <h1>Change KiwiSaver Rate</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>Your KiwiSaver contribution rate has been updated to {selectedRate}%.</p>
            <Link href="/dept/ird/kiwisaver">View KiwiSaver details</Link>
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
      <Link href="/dept/ird/kiwisaver">← Back to KiwiSaver</Link>
      <h1>Change KiwiSaver Rate</h1>

      <p>Select your new contribution rate. Changes take effect from your next pay cycle.</p>

      <form onSubmit={handleSubmit}>
        <fieldset>
          <legend>Contribution rate</legend>
          {RATE_OPTIONS.map((rate) => (
            <label key={rate} style={{ display: "block", marginBottom: "0.5rem" }}>
              <input
                type="radio"
                name="rate"
                value={rate}
                checked={selectedRate === rate}
                onChange={() => setSelectedRate(rate)}
              />
              {" "}{rate}%
            </label>
          ))}
        </fieldset>

        <button type="submit" disabled={submitting} style={{ marginTop: "1rem" }}>
          {submitting ? "Submitting…" : "Update rate"}
        </button>
      </form>
    </main>
  );
}
