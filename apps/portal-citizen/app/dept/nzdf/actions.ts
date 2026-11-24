"use server";

import { parseScopes, parseActionRequest, parseQuestion } from "@tpt/gov-schema";
import type { NzdfDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceNzdfAiContext } from "./nzdf-ai";
import { PORTAL_CONFIG } from "../../lib/config";

const NZDF_SERVICE_URL = process.env.NZDF_SERVICE_URL ?? "http://localhost:8145";

export async function fetchNzdfData(scopes: string[]): Promise<NzdfDataBundle | null> {
  const validScopes = parseScopes(scopes);
  if (!validScopes) return null;
  scopes = validScopes;
  return (await fetchDeptData("nzdf", scopes)) as NzdfDataBundle | null;
}

export async function submitNzdfAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  const request = parseActionRequest(type, parameters);
  if (!request) return { success: false, message: "Invalid request." };
  type = request.type;
  parameters = request.parameters;
  if (PORTAL_CONFIG.demoMode) {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(NZDF_SERVICE_URL + "/citizen/action", {
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

// ── AI ─────────────────────────────────────────────────────────────

export async function askNzdf(question: string): Promise<{ answer: string; enabled: boolean }> {
  const validQuestion = parseQuestion(question);
  if (!validQuestion) return { answer: "Please enter a question.", enabled: false };
  question = validQuestion;
  const data = await fetchNzdfData(["nzdf:service-records", "nzdf:deployments"]);
  if (!data) {
    return { answer: "Unable to load your New Zealand Defence Force information.", enabled: false };
  }
  const context = produceNzdfAiContext(data);
  return askWithContext(question, context);
}
