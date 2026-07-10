"use server";

import type { MOHDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";

const MOH_SERVICE_URL = process.env.MOH_SERVICE_URL ?? "http://localhost:8092";

async function getCitizenDid(): Promise<string | null> {
  return process.env.DEMO_CITIZEN_DID ?? "did:gov:nz:test-citizen-001";
}

export async function fetchMohData(scopes: string[]): Promise<MOHDataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${MOH_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MOHDataBundle>;
  } catch {
    return null;
  }
}

export async function submitMohAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${MOH_SERVICE_URL}/citizen/action`, {
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

// ── AI health navigation (Stage 6) ────────────────────────────────────────────

export async function askMohHealth(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMohData(["moh:gp", "moh:prescriptions", "moh:vaccinations"]);
  if (!data) {
    return { answer: "Unable to load your health information.", enabled: false };
  }
  const context = produceMohAiContext(data);
  return askWithContext(question, context);
}

/** Build the AI context chunks from a MOH data bundle. */
export function produceMohAiContext(data: MOHDataBundle): { kind: string; text: string; pii: boolean }[] {
  const chunks: { kind: string; text: string; pii: boolean }[] = [];

  chunks.push({
    kind: "moh_nhi",
    text: `National Health Index number ends in ${data.nhiNumber.slice(-4)}.`,
    pii: false,
  });

  if (data.enrolledGP) {
    chunks.push({
      kind: "moh_gp",
      text: `Enrolled with ${data.enrolledGP.practiceName} (${data.enrolledGP.address}).`,
      pii: false,
    });
  }

  for (const p of data.activePrescriptions ?? []) {
    chunks.push({
      kind: "moh_prescription",
      text: `Prescription ${p.medication} ${p.dose}, ${p.repeatsRemaining} repeat(s) remaining.`,
      pii: false,
    });
  }

  for (const v of data.vaccinations ?? []) {
    const booster = v.dueForBooster ? " — booster due" : "";
    chunks.push({
      kind: "moh_vaccination",
      text: `Vaccination ${v.vaccine} on ${v.date}${booster}.`,
      pii: false,
    });
  }

  return chunks;
}
