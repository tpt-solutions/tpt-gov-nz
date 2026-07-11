"use client";

import Link from "next/link";
import { useTransition } from "react";
import LanguageToggle from "./LanguageToggle";
import { useLanguage } from "./LanguageProvider";
import { logout } from "@/app/lib/auth-actions";
import type { ScenarioId } from "@/app/lib/mock-data";

export default function Header({
  did,
  demo,
  scenario,
}: {
  did: string | null;
  demo: boolean;
  scenario: ScenarioId;
}) {
  const { t } = useLanguage();
  const [pending, startTransition] = useTransition();

  const shortDid = did ? `${did.slice(0, 18)}…` : null;

  return (
    <header className="site-header">
      <div className="site-header__inner">
        <Link href="/" className="site-header__brand">
          {t("appName")}
          <small>{t("tagline")}</small>
        </Link>
        <nav className="site-nav" aria-label="Primary">
          <Link href="/">{t("home")}</Link>
          <Link href="/dashboard">{t("dashboard")}</Link>
          <Link href="/consent">{t("consent")}</Link>
          <Link href="/audit">{t("audit")}</Link>
        </nav>
        <div className="header-actions">
          <LanguageToggle />
          {did ? (
            <>
              {shortDid && (
                <span className="did-chip" title={did}>
                  {shortDid}
                  {demo ? " · demo" : ""}
                </span>
              )}
              <button
                type="button"
                className="btn btn--small btn--ghost"
                disabled={pending}
                onClick={() => startTransition(() => logout())}
              >
                {t("logout")}
              </button>
            </>
          ) : (
            <Link href="/login" className="btn btn--small">
              {t("login")}
            </Link>
          )}
        </div>
      </div>
    </header>
  );
}
