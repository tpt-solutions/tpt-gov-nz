"use client";

import { useActionState } from "react";
import { simulatePolicy } from "./actions";

interface SimulationResult {
  ok: boolean;
  report?: string;
  error?: string;
}

const initial: SimulationResult = { ok: true };

export default function SimulateForm({
  id,
  title,
  summary,
  departments,
}: {
  id: string;
  title: string;
  summary: string;
  departments: string[];
}) {
  const [state, formAction, pending] = useActionState(simulatePolicy, initial);

  return (
    <div>
      <h1>{title}</h1>
      <div className="card">
        <p>{summary}</p>
        <p>
          {departments.map((d) => (
            <span className="badge" key={d}>
              {d}
            </span>
          ))}
        </p>

        <form action={formAction}>
          <input type="hidden" name="scenario" value={id} />
          <button type="submit" disabled={pending}>
            {pending ? "Simulating…" : "Run simulation"}
          </button>
        </form>
      </div>

      {state.ok === false && state.error && (
        <div className="error">{state.error}</div>
      )}

      {state.ok && state.report && (
        <div className="card report">{state.report}</div>
      )}
    </div>
  );
}
