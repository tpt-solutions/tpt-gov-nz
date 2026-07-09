"use client";

import { useState } from "react";
import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { submitIrdAction } from "../../actions";

export default function FileGstReturnForm() {
  const searchParams = useSearchParams();
  const periodId = searchParams.get("period") ?? "";

  const [salesIncome, setSalesIncome] = useState("");
  const [gstOnSales, setGstOnSales] = useState("");
  const [gstOnPurchases, setGstOnPurchases] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);

    const res = await submitIrdAction("file-gst-return", {
      periodId,
      salesIncome: Number(salesIncome),
      gstOnSales: Number(gstOnSales),
      gstOnPurchases: Number(gstOnPurchases),
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>GST return for period {periodId} has been filed successfully.</p>
            <Link href="/dept/ird/gst">View GST periods</Link>
          </>
        ) : (
          <>
            <p style={{ color: "red" }}>{result.message ?? "Something went wrong. Please try again."}</p>
            <button type="button" onClick={() => setResult(null)}>Try again</button>
          </>
        )}
      </>
    );
  }

  return (
    <>
      <p>Period: <strong>{periodId}</strong></p>

      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: "1rem" }}>
          <label htmlFor="salesIncome">Sales income (NZD)</label>
          <br />
          <input
            id="salesIncome"
            type="number"
            step="0.01"
            min="0"
            required
            value={salesIncome}
            onChange={(e) => setSalesIncome(e.target.value)}
          />
        </div>

        <div style={{ marginBottom: "1rem" }}>
          <label htmlFor="gstOnSales">GST collected on sales (NZD)</label>
          <br />
          <input
            id="gstOnSales"
            type="number"
            step="0.01"
            min="0"
            required
            value={gstOnSales}
            onChange={(e) => setGstOnSales(e.target.value)}
          />
        </div>

        <div style={{ marginBottom: "1rem" }}>
          <label htmlFor="gstOnPurchases">GST paid on purchases (NZD)</label>
          <br />
          <input
            id="gstOnPurchases"
            type="number"
            step="0.01"
            min="0"
            required
            value={gstOnPurchases}
            onChange={(e) => setGstOnPurchases(e.target.value)}
          />
        </div>

        {gstOnSales && gstOnPurchases && (
          <p>
            Net GST: <strong>
              {Number(gstOnSales) - Number(gstOnPurchases) >= 0
                ? `Owing $${(Number(gstOnSales) - Number(gstOnPurchases)).toFixed(2)}`
                : `Refund $${Math.abs(Number(gstOnSales) - Number(gstOnPurchases)).toFixed(2)}`}
            </strong>
          </p>
        )}

        <button type="submit" disabled={submitting}>
          {submitting ? "Submitting…" : "File return"}
        </button>
      </form>
    </>
  );
}
