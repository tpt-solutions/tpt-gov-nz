"use client";

import { useState } from "react";
import { useLanguage } from "./LanguageProvider";

interface Msg {
  role: "user" | "bot";
  text: string;
}

export default function AiChatWidget({ enabled }: { enabled: boolean }) {
  const { t } = useLanguage();
  const [open, setOpen] = useState(false);
  const [messages, setMessages] = useState<Msg[]>([]);
  const [input, setInput] = useState("");
  const [busy, setBusy] = useState(false);

  if (!enabled) return null;

  async function send() {
    const q = input.trim();
    if (!q || busy) return;
    setInput("");
    setMessages((m) => [...m, { role: "user", text: q }, { role: "bot", text: "…" }]);
    setBusy(true);
    try {
      const res = await fetch("/api/ai", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ question: q }),
      });
      const data = (await res.json()) as { answer?: string; error?: string };
      setMessages((m) => {
        const copy = [...m];
        copy[copy.length - 1] = {
          role: "bot",
          text: data.answer ?? data.error ?? "No response.",
        };
        return copy;
      });
    } catch {
      setMessages((m) => {
        const copy = [...m];
        copy[copy.length - 1] = { role: "bot", text: "Unable to reach the assistant." };
        return copy;
      });
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      {open ? (
        <div className="ai-panel" role="dialog" aria-label={t("aiAssistant")}>
          <div className="ai-panel__head">{t("aiAssistant")}</div>
          <div className="ai-panel__body">
            {messages.length === 0 && (
              <p style={{ color: "var(--muted)" }}>
                Ask about your services, entitlements or next steps.
              </p>
            )}
            {messages.map((m, i) => (
              <div key={i} className={`ai-panel__msg ai-panel__msg--${m.role}`}>
                <span className="ai-panel__bubble">{m.text}</span>
              </div>
            ))}
          </div>
          <div className="ai-panel__input">
            <input
              aria-label="Ask the assistant"
              value={input}
              disabled={busy}
              placeholder="Type a question…"
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") send();
              }}
            />
            <button type="button" onClick={send} disabled={busy} aria-label="Send">
              ➤
            </button>
          </div>
        </div>
      ) : null}
      <button
        className="ai-fab"
        aria-expanded={open}
        aria-label={t("aiAssistant")}
        onClick={() => setOpen((o) => !o)}
      >
        {open ? "×" : "💬"}
      </button>
    </>
  );
}
