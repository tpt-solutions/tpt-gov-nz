"use client";

import { useEffect, useState } from "react";

interface Step {
  title: string;
  body: string;
}

const STEPS: Step[] = [
  {
    title: "Welcome to My Gov NZ",
    body: "This is a unified portal for New Zealand government services. Everything you see is built from consented data held by separate department systems — there is no central profile storing your personal information.",
  },
  {
    title: "Your dashboard",
    body: "The dashboard shows the government departments you can access. Each card opens the department's own module with your data, served directly from that department's system.",
  },
  {
    title: "You control your data",
    body: "On the Consent page you can grant or revoke a department's permission to access data you hold with another department. Every grant is signed and appears in your audit trail.",
  },
  {
    title: "Try a scenario",
    body: "This demo runs entirely on fictional data for citizen Alex Tane. Use the banner to switch between Standard, Beneficiary and New-parent scenarios and watch the departments update.",
  },
  {
    title: "Ask the AI assistant",
    body: "The assistant (bottom-right) can answer questions using only your consented data. It is optional and can be turned off — the portal works fully without it.",
  },
];

const TOUR_KEY = "tpt_tour_done";

export default function GuidedTour() {
  const [open, setOpen] = useState(false);
  const [step, setStep] = useState(0);

  useEffect(() => {
    if (typeof window === "undefined") return;
    if (!localStorage.getItem(TOUR_KEY)) {
      const timer = setTimeout(() => setOpen(true), 400);
      return () => clearTimeout(timer);
    }
  }, []);

  function finish() {
    try {
      localStorage.setItem(TOUR_KEY, "1");
    } catch {
      /* ignore */
    }
    setOpen(false);
  }

  if (!open) return null;

  const current = STEPS[step];
  const isLast = step === STEPS.length - 1;

  return (
    <div className="tour-overlay" role="dialog" aria-modal="true" aria-label="Guided tour">
      <div className="tour-card">
        <h2 style={{ marginTop: 0 }}>{current.title}</h2>
        <p>{current.body}</p>
        <div className="tour-step-dots" aria-hidden>
          {STEPS.map((_, i) => (
            <span key={i} className={`tour-dot ${i === step ? "tour-dot--active" : ""}`} />
          ))}
        </div>
        <div className="tour-actions">
          <button type="button" className="btn btn--small btn--ghost" onClick={finish}>
            Skip tour
          </button>
          <div style={{ display: "flex", gap: "0.5rem" }}>
            {step > 0 && (
              <button
                type="button"
                className="btn btn--small btn--ghost"
                onClick={() => setStep((s) => Math.max(0, s - 1))}
              >
                Back
              </button>
            )}
            <button
              type="button"
              className="btn btn--small"
              onClick={() => (isLast ? finish() : setStep((s) => s + 1))}
            >
              {isLast ? "Done" : "Next"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
