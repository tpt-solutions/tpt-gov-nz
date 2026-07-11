"use client";

import { useLanguage, LOCALES } from "@/app/components/LanguageProvider";

export default function LanguageToggle() {
  const { locale, setLocale, t } = useLanguage();
  return (
    <label className="field" style={{ margin: 0, display: "flex", alignItems: "center", gap: "0.4rem" }}>
      <span className="sr-only">{t("language")}</span>
      <span aria-hidden style={{ fontSize: "0.85rem", color: "var(--muted)" }}>🌐</span>
      <select
        value={locale}
        aria-label={t("language")}
        onChange={(e) => setLocale(e.target.value as "en" | "mi")}
        style={{ padding: "0.3rem 0.4rem", borderRadius: "8px", border: "1px solid var(--border)" }}
      >
        {LOCALES.map((l) => (
          <option key={l.id} value={l.id}>
            {l.label}
          </option>
        ))}
      </select>
    </label>
  );
}
