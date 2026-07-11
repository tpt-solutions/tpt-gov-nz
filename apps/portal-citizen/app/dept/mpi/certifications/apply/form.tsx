"use client";

import { useState } from "react";
import Link from "next/link";
import { submitMpiAction } from "../actions";

export default function MpiExportCertificateForm() {
  const [product, setProduct] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMpiAction("apply-export-certificate", { product });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/mpi/certifications">← Back to certifications</Link>
        <h1>Apply for an Export Certificate</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Application submitted."}</p>
            <Link href="/dept/mpi/certifications">View certifications</Link>
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
      <label htmlFor="product" style={{ display: "block", marginBottom: "0.5rem" }}>
        Product
        <input
          id="product"
          type="text"
          value={product}
          onChange={(e) => setProduct(e.target.value)}
          placeholder="e.g. Apples"
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button type="submit" disabled={submitting || product.trim().length === 0}>
        {submitting ? "Submitting…" : "Apply"}
      </button>
    </form>
  );
}
