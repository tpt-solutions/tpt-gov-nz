"use client";

import { useState } from "react";
import { submitMohAction } from "../actions";

export default function RepeatPrescriptionForm({ prescriptionId, medication }: {
  prescriptionId: string;
  medication: string;
}) {
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitMohAction("request-repeat-prescription", { prescriptionId });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <p style={{ color: result.success ? "green" : "red" }}>
        {result.message ?? (result.success ? "Requested." : "Something went wrong.")}
      </p>
    );
  }

  return (
    <form onSubmit={handleSubmit} style={{ display: "inline" }}>
      <button type="submit" disabled={submitting}>
        {submitting ? "Requesting…" : `Request repeat (${medication})`}
      </button>
    </form>
  );
}
