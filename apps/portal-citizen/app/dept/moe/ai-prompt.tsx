"use client";

import { useState } from "react";
import { askMoe } from "./actions";

export default function MoeAiPrompt() {
  const [question, setQuestion] = useState("What education enrolment or support involves me?");
  const [answer, setAnswer] = useState<string | null>(null);
  const [enabled, setEnabled] = useState(true);
  const [asking, setAsking] = useState(false);

  async function handleAsk(e: React.FormEvent) {
    e.preventDefault();
    setAsking(true);
    setAnswer(null);
    const res = await askMoe(question);
    setEnabled(res.enabled);
    setAnswer(res.answer);
    setAsking(false);
  }

  return (
    <section style={{ marginTop: "1.5rem" }}>
      <h2>Ask about your Education information</h2>
      <form onSubmit={handleAsk}>
        <label htmlFor="moe-q" style={{ display: "block", marginBottom: "0.5rem" }}>
          Your question
          <input
            id="moe-q"
            type="text"
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
            style={{ display: "block", marginTop: "0.25rem", width: "100%", maxWidth: "32rem" }}
          />
        </label>
        <button type="submit" disabled={asking}>
          {asking ? "Asking..." : "Ask AI"}
        </button>
      </form>

      {answer != null && (
        <p style={{ marginTop: "1rem", whiteSpace: "pre-wrap" }}>
          <strong>AI:</strong> {answer}
          {!enabled && <em> (AI is disabled — enable an AI level to get a live answer.)</em>}
        </p>
      )}
    </section>
  );
}
