"use server";

import type { IRDDataBundle } from "@tpt/gov-schema";

const IRD_SERVICE_URL = process.env.IRD_SERVICE_URL ?? "http://localhost:8090";

async function getCitizenDid(): Promise<string | null> {
  // In Phase 1 this reads from the session cookie / JWT
  // Stubbed for now — full implementation when identity server is wired up
  return process.env.DEMO_CITIZEN_DID ?? "did:gov:nz:test-citizen-001";
}

export async function fetchIrdData(scopes: string[]): Promise<IRDDataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${IRD_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<IRDDataBundle>;
  } catch {
    return null;
  }
}

export async function submitIrdAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${IRD_SERVICE_URL}/citizen/action`, {
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
