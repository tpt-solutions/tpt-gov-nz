"use client";

import { useState } from "react";
import Link from "next/link";
import { submitIrdAction, recommendKiwiSaverRateAction, type KiwiSaverRecommendation } from "../../actions";

const RATE_OPTIONS = [3, 4, 6, 8, 10] as const;

export default function UpdateKiwiSaverRateForm({
  aiLevel,
  currentRate,
}: {
  aiLevel: string;
  currentRate: number | null;
}) {
  const initial = currentRate ?? 6;
  const [selectedRate, setSelectedRate] = useState<number>(initial);
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  const [recommendation, setRecommendation] = useState<KiwiSaverRecommendation | null>(null);
  const [loadingRec, setLoadingRec] = useState(false);

  async function loadRecommendation() {
    setLoadingRec(true);
    const rec = await recommendKiwiSaverRateAction();
    setRecommendation(rec);
    setSelectedRate(rec.suggestedRate);
    setLoadingRec(false);
  }

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

  const aiAssisted = aiLevel === "assisted" || aiLevel === "automated";

  return (
    <main>
      <Link href="/dept/ird/kiwisaver">← Back to KiwiSaver</Link>
      <h1>Change KiwiSaver Rate</h1>

      <p>Select your new contribution rate. Changes take effect from your next pay cycle.</p>

      {aiAssisted && (
        <section style={{ margin: "1rem 0", padding: "0.75rem", border: "1px solid #ccc" }}>
          <h2>AI suggestion</h2>
          {!recommendation ? (
            <button type="button" onClick={loadRecommendation} disabled={loadingRec}>
              {loadingRec ? "Calculating…" : "Get a recommended rate"}
            </button>
          ) : (
            <>
              <p>
                Based on your balance, we suggest <strong>{recommendation.suggestedRate}%</strong>.
              </p>
              <p>{recommendation.reason}</p>
              <button type="button" onClick={() => setSelectedRate(recommendation.suggestedRate)}>
                Use {recommendation.suggestedRate}% suggestion
              </button>
            </>
          )}
        </section>
      )}

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
