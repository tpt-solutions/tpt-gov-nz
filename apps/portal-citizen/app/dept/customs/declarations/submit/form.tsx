"use client";

import { useState } from "react";
import Link from "next/link";
import { submitCustomsAction } from "../actions";

export default function DeclarationForm() {
  const [countryFrom, setCountryFrom] = useState("");
  const [goodsDeclared, setGoodsDeclared] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitCustomsAction("submit-traveller-declaration", {
      countryFrom,
      goodsDeclared,
    });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <main>
        <Link href="/dept/customs/declarations">← Back to declarations</Link>
        <h1>Submit a Traveller Declaration</h1>
        {result.success ? (
          <>
            <p style={{ color: "green" }}>{result.message ?? "Declaration submitted."}</p>
            <Link href="/dept/customs/declarations">View declarations</Link>
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
      <label htmlFor="countryFrom" style={{ display: "block", marginBottom: "0.5rem" }}>
        Country travelled from
        <input
          id="countryFrom"
          type="text"
          value={countryFrom}
          onChange={(e) => setCountryFrom(e.target.value)}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <label htmlFor="goodsDeclared" style={{ display: "block", marginBottom: "0.5rem" }}>
        Goods declared
        <textarea
          id="goodsDeclared"
          value={goodsDeclared}
          onChange={(e) => setGoodsDeclared(e.target.value)}
          rows={4}
          style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
        />
      </label>
      <button
        type="submit"
        disabled={submitting || countryFrom.trim().length === 0 || goodsDeclared.trim().length === 0}
      >
        {submitting ? "Submitting…" : "Submit declaration"}
      </button>
    </form>
  );
}
