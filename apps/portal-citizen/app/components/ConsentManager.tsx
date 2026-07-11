"use client";

import { useTransition } from "react";
import { grantConsentAction, revokeConsentAction } from "@/app/lib/consent-actions";
import type { GrantView } from "@/app/lib/consent";
import { useLanguage } from "@/app/components/LanguageProvider";

interface DeptLite {
  id: string;
  name: string;
  shortName: string;
}

export default function ConsentManager({
  grants,
  depts,
}: {
  grants: GrantView[];
  depts: DeptLite[];
}) {
  const { t } = useLanguage();
  const [pending, startTransition] = useTransition();

  const findByPair = (req: string, prov: string) =>
    grants.find((g) => g.requestingDeptId === req && g.providingDeptId === prov);

  return (
    <div className="card-grid">
      {depts.flatMap((req) =>
        depts
          .filter((prov) => prov.id !== req.id)
          .map((prov) => {
            const grant = findByPair(req.id, prov.id);
            return (
              <div key={`${req.id}-${prov.id}`} className="card">
                <h3 style={{ marginTop: 0 }}>
                  {req.shortName} → {prov.shortName}
                </h3>
                <p style={{ color: "var(--muted)", minHeight: "2.5rem" }}>
                  Allow {req.name} to access your {prov.name} data.
                </p>
                {grant ? (
                  <button
                    type="button"
                    className="btn btn--small btn--danger"
                    disabled={pending}
                    onClick={() => startTransition(() => { void revokeConsentAction(grant.id); })}
                  >
                    {t("revoke")}
                  </button>
                ) : (
                  <button
                    type="button"
                    className="btn btn--small"
                    disabled={pending}
                    onClick={() =>
                      startTransition(() => {
                        void grantConsentAction(req.id as never, prov.id as never);
                      })
                    }
                  >
                    {t("grant")}
                  </button>
                )}
              </div>
            );
          }),
      )}
    </div>
  );
}
