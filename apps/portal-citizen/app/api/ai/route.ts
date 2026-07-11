import { NextResponse } from "next/server";
import { getSession } from "@/app/lib/session";
import { fetchDeptData } from "@/app/lib/data-access";
import { DEPARTMENTS } from "@/app/lib/config";
import { buildAiContext } from "@/app/lib/ai-context";
import { buildAiClient } from "@/app/ai/client";

export const dynamic = "force-dynamic";

const SYSTEM_PROMPT = `You are an assistant for the New Zealand government portal, helping citizens
understand their government services. Use only the consented data provided. Be concise,
plain-language, and never invent figures. If information is missing, say so. Do not reveal raw
identifiers such as IRD numbers, NHI or passport numbers.`;

export async function POST(req: Request) {
  const session = await getSession();
  if (!session) {
    return NextResponse.json({ error: "Not authenticated" }, { status: 401 });
  }

  const { question } = (await req.json()) as { question?: string };
  if (!question || !question.trim()) {
    return NextResponse.json({ error: "Empty question" }, { status: 400 });
  }

  const bundles: Record<string, unknown> = {};
  await Promise.all(
    DEPARTMENTS.map(async (d) => {
      bundles[d.id] = await fetchDeptData(d.id, d.scopes);
    }),
  );

  const context = buildAiContext({
    ird: bundles.ird as never,
    winz: bundles.winz as never,
    moh: bundles.moh as never,
    dia: bundles.dia as never,
  });

  const client = buildAiClient();
  if (!client.isEnabled) {
    return NextResponse.json({
      answer: "AI assistance is currently disabled for this portal.",
      enabled: false,
    });
  }

  try {
    const res = await client.chat(SYSTEM_PROMPT, question, context);
    return NextResponse.json({ answer: res.content, enabled: true });
  } catch (e) {
    return NextResponse.json(
      { answer: "Sorry, I could not reach the AI service just now.", enabled: true, error: String(e) },
      { status: 502 },
    );
  }
}
