"use server";

import type { DIADataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";

const DIA_SERVICE_URL = process.env.DIA_SERVICE_URL ?? "http://localhost:8093";

async function getCitizenDid(): Promise<string | null> {
  return process.env.DEMO_CITIZEN_DID ?? "did:gov:nz:test-citizen-001";
}

export async function fetchDiaData(scopes: string[]): Promise<DIADataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${DIA_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<DIADataBundle>;
  } catch {
    return null;
  }
}

export async function submitDiaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${DIA_SERVICE_URL}/citizen/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, type, parameters, performed_by: "citizen" }),
    });

    if (!res.ok) {
      const err = (await res.json()) as { error?: string };
      return { success: false, message: err.error ?? "Action failed" };
    }

    return res.json() as Promise<{ success: boolean; message?: string }>;
  } catch {
    return { success: false, message: "Service unavailable" };
  }
}

// ── AI: "my passport is expiring, what do I do?" (Stage 6) ──────────────────────

export async function askDia(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchDiaData(["dia:passport", "dia:citizenship"]);
  if (!data) {
    return { answer: "Unable to load your DIA information.", enabled: false };
  }
  const context = produceDiaAiContext(data);
  return askWithContext(question, context);
}

export function produceDiaAiContext(data: DIADataBundle): { kind: string; text: string; pii: boolean }[] {
  const chunks: { kind: string; text: string; pii: boolean }[] = [];

  chunks.push({
    kind: "dia_passport_number",
    text: `Passport number ends in ${data.passportNumber.slice(-4)}.`,
    pii: false,
  });

  if (data.passport) {
    const expiringSoon = new Date(data.passport.expiryDate).getTime() - Date.now() < 1000 * 60 * 60 * 24 * 365;
    chunks.push({
      kind: "dia_passport",
      text: `Passport expires ${data.passport.expiryDate}.${data.passport.renewable ? " Renewable." : ""}${expiringSoon ? " Expiring within a year." : ""}`,
      pii: false,
    });
  }

  if (data.citizenship) {
    chunks.push({
      kind: "dia_citizenship",
      text: `Citizenship status: ${data.citizenship.status}.`,
      pii: false,
    });
  }

  return chunks;
}
