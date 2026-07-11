"use client";

import { useState } from "react";
import { checkWffEligibility } from "../../actions";
import type { WffEligibilityResult } from "../../helpers";

export default function BabyLifeEventForm() {
  const [children, setChildren] = useState(1);
  const [income, setIncome] = useState(60_000);
  const [result, setResult] = useState<WffEligibilityResult | null>(null);
  const [checking, setChecking] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setChecking(true);
    const res = await checkWffEligibility({
      dependantChildren: children,
      familyIncome: income,
    });
    setResult(res);
    setChecking(false);
  }

  return (
    <div>
      <p>
        Tell us about your situation and we will check whether you may qualify for
        Working for Families. Having a new baby usually increases your entitlement.
      </p>

      <form onSubmit={handleSubmit}>
        <fieldset>
          <legend>Your situation</legend>

          <label htmlFor="children" style={{ display: "block", margin: "0.5rem 0" }}>
            Number of dependent children (including the new baby)
            <input
              id="children"
              type="number"
              min={0}
              max={20}
              value={children}
              onChange={(e) => setChildren(Number(e.target.value))}
              style={{ display: "block", marginTop: "0.25rem" }}
            />
          </label>

          <label htmlFor="income" style={{ display: "block", margin: "0.5rem 0" }}>
            Estimated total family income (before tax, per year)
            <input
              id="income"
              type="number"
              min={0}
              step={1000}
              value={income}
              onChange={(e) => setIncome(Number(e.target.value))}
              style={{ display: "block", marginTop: "0.25rem" }}
            />
          </label>
        </fieldset>

        <button type="submit" disabled={checking} style={{ marginTop: "1rem" }}>
          {checking ? "Checking…" : "Check eligibility"}
        </button>
      </form>

      {result && (
        <section style={{ marginTop: "1.5rem" }}>
          <h2>Eligibility result</h2>
          <p>
            <strong>
              {result.eligible ? "You may be eligible" : "You may not be eligible"}
            </strong>
          </p>
          <p>{result.note}</p>
          {result.incomeThreshold != null && (
            <dl>
              <dt>Income threshold</dt>
              <dd>${result.incomeThreshold.toLocaleString()}</dd>
              {result.headroom != null && (
                <>
                  <dt>Headroom</dt>
                  <dd>
                    {result.headroom >= 0
                      ? `$${result.headroom.toLocaleString()} below threshold`
                      : `$${Math.abs(result.headroom).toLocaleString()} above threshold`}
                  </dd>
                </>
              )}
            </dl>
          )}
          <p>
            <small>
              This is an estimate based on your income threshold. Final eligibility is
              confirmed by Inland Revenue.
            </small>
          </p>
        </section>
      )}
    </div>
  );
}
