"use server";

import { parseScopes, parseActionRequest, parseQuestion } from "@tpt/gov-schema";
import type { MPIDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMpiAiContext } from "./mpi-ai";
import { PORTAL_CONFIG } from "../../lib/config";

const MPI_SERVICE_URL = process.env.MPI_SERVICE_URL ?? "http://localhost:8106";

export async function fetchMpiData(scopes: string[]): Promise<MPIDataBundle | null> {
  const validScopes = parseScopes(scopes);
  if (!validScopes) return null;
  scopes = validScopes;
  return (await fetchDeptData("mpi", scopes)) as MPIDataBundle | null;
}

export async function submitMpiAction(
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
    const res = await fetch(`${MPI_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what certifications do I hold?" (Stage 6) ─────────────

export async function askMpi(question: string): Promise<{ answer: string; enabled: boolean }> {
  const validQuestion = parseQuestion(question);
  if (!validQuestion) return { answer: "Please enter a question.", enabled: false };
  question = validQuestion;
  const data = await fetchMpiData(["mpi:registrations", "mpi:certifications"]);
  if (!data) {
    return { answer: "Unable to load your MPI information.", enabled: false };
  }
  const context = produceMpiAiContext(data);
  return askWithContext(question, context);
}
