"use client";

import { useTransition } from "react";
import { setScenario } from "@/app/lib/auth-actions";
import { resetDemoAction } from "@/app/lib/consent-actions";
import { SCENARIOS, type ScenarioId } from "@/app/lib/mock-data";
import { useLanguage } from "@/app/components/LanguageProvider";

export default function DemoBanner({ scenario }: { scenario: ScenarioId }) {
  const { t } = useLanguage();
  const [pending, startTransition] = useTransition();

  return (
    <div className="demo-banner" role="status">
      <div className="demo-banner__inner">
        <span>🧭 {t("demoBanner")}</span>
        <span className="demo-banner__spacer" />
        <span className="demo-scenario">
          <label htmlFor="demo-scenario" style={{ fontSize: "0.85rem" }}>
            {t("demoScenario")}:
          </label>
          <select
            id="demo-scenario"
            value={scenario}
            disabled={pending}
            onChange={(e) =>
              startTransition(() => setScenario(e.target.value as ScenarioId))
            }
            style={{ padding: "0.25rem 0.4rem", borderRadius: "8px", border: "1px solid #f0d9a8" }}
          >
            {SCENARIOS.map((s) => (
              <option key={s.id} value={s.id}>
                {s.label}
              </option>
            ))}
          </select>
          <button
            type="button"
            className="btn btn--small btn--ghost"
            disabled={pending}
            onClick={() => startTransition(() => resetDemoAction())}
          >
            {t("resetDemo")}
          </button>
        </span>
      </div>
    </div>
  );
}
