"use client";

import { useState } from "react";
import { submitPoliceAction } from "../actions";

export default function InfringementActions({ ticketNumber }: { ticketNumber: string }) {
  const [reason, setReason] = useState("");
  const [showDispute, setShowDispute] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<{ success: boolean; message?: string } | null>(null);

  async function handlePay() {
    setSubmitting(true);
    setResult(null);
    const res = await submitPoliceAction("pay-infringement", { ticketNumber });
    setResult(res);
    setSubmitting(false);
  }

  async function handleDispute(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);
    const res = await submitPoliceAction("dispute-infringement", { ticketNumber, reason });
    setResult(res);
    setSubmitting(false);
  }

  if (result) {
    return (
      <p style={{ color: result.success ? "green" : "red" }}>
        {result.message ?? (result.success ? "Done." : "Something went wrong.")}
      </p>
    );
  }

  return (
    <div style={{ marginTop: "0.5rem" }}>
      <button type="button" onClick={handlePay} disabled={submitting} style={{ marginRight: "0.5rem" }}>
        Pay
      </button>
      <button type="button" onClick={() => setShowDispute((v) => !v)} disabled={submitting}>
        Dispute
      </button>
      {showDispute && (
        <form onSubmit={handleDispute} style={{ marginTop: "0.5rem" }}>
          <label style={{ display: "block", marginBottom: "0.5rem" }}>
            Reason for dispute
            <textarea
              value={reason}
              onChange={(e) => setReason(e.target.value)}
              rows={2}
              style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "24rem" }}
            />
          </label>
          <button type="submit" disabled={submitting || reason.trim().length === 0}>
            Submit dispute
          </button>
        </form>
      )}
    </div>
  );
}
