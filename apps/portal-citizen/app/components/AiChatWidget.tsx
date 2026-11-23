"use client";

import { useEffect, useRef, useState, type KeyboardEvent } from "react";
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
  const panelRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const fabRef = useRef<HTMLButtonElement>(null);

  // Move focus into the panel when it opens.
  useEffect(() => {
    if (open) inputRef.current?.focus();
  }, [open]);

  if (!enabled) return null;

  function close() {
    setOpen(false);
    // Return focus to the toggle so keyboard users are not left on a removed element.
    fabRef.current?.focus();
  }

  /** Escape closes the panel; Tab and Shift+Tab stay inside it (focus trap). */
  function onPanelKeyDown(e: KeyboardEvent<HTMLDivElement>) {
    if (e.key === "Escape") {
      e.preventDefault();
      close();
      return;
    }
    if (e.key !== "Tab") return;
    const focusable = panelRef.current?.querySelectorAll<HTMLElement>("input, button:not([disabled])");
    if (!focusable || focusable.length === 0) return;
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (!first || !last) return;
    if (e.shiftKey && document.activeElement === first) {
      e.preventDefault();
      last.focus();
    } else if (!e.shiftKey && document.activeElement === last) {
      e.preventDefault();
      first.focus();
    }
  }

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
        <div
          className="ai-panel"
          role="dialog"
          aria-modal="false"
          aria-label={t("aiAssistant")}
          ref={panelRef}
          onKeyDown={onPanelKeyDown}
        >
          <div className="ai-panel__head">{t("aiAssistant")}</div>
          <div className="ai-panel__body" role="log" aria-live="polite" aria-relevant="additions">
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
              ref={inputRef}
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
        ref={fabRef}
        className="ai-fab"
        aria-expanded={open}
        aria-label={t("aiAssistant")}
        onClick={() => (open ? close() : setOpen(true))}
      >
        {open ? "×" : "💬"}
      </button>
    </>
  );
}
