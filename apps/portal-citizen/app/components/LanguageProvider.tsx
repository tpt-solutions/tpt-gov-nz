"use client";

import { createContext, useContext, useEffect, useState, useCallback } from "react";
import { LOCALES, translate, type Locale } from "@/app/lib/messages";
import { PORTAL_CONFIG } from "@/app/lib/config";

interface LanguageContextValue {
  locale: Locale;
  setLocale: (l: Locale) => void;
  t: (key: string) => string;
}

const LanguageContext = createContext<LanguageContextValue | null>(null);

function readInitialLocale(): Locale {
  if (typeof document !== "undefined") {
    const c = document.cookie
      .split("; ")
      .find((p) => p.startsWith(`${PORTAL_CONFIG.localeCookieName}=`));
    if (c) {
      const v = c.split("=")[1];
      if (v === "mi" || v === "en") return v;
    }
  }
  return "en";
}

export function LanguageProvider({ children }: { children: React.ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>("en");

  useEffect(() => {
    setLocaleState(readInitialLocale());
  }, []);

  useEffect(() => {
    document.documentElement.lang = locale === "mi" ? "mi" : "en-NZ";
  }, [locale]);

  const setLocale = useCallback((l: Locale) => {
    setLocaleState(l);
    document.cookie = `${PORTAL_CONFIG.localeCookieName}=${l}; path=/; max-age=${60 * 60 * 24 * 365}; samesite=lax`;
  }, []);

  const t = useCallback((key: string) => translate(locale, key), [locale]);

  return (
    <LanguageContext.Provider value={{ locale, setLocale, t }}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage(): LanguageContextValue {
  const ctx = useContext(LanguageContext);
  if (!ctx) {
    // Safe fallback if used outside the provider (e.g. in a test).
    return {
      locale: "en",
      setLocale: () => {},
      t: (key: string) => translate("en", key),
    };
  }
  return ctx;
}

export { LOCALES };
