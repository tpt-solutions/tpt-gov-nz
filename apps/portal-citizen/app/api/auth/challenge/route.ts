import { NextResponse } from "next/server";
import { cookies } from "next/headers";
import { randomToken } from "@/app/lib/jwt";
import { PORTAL_CONFIG } from "@/app/lib/config";

export const dynamic = "force-dynamic";

export async function GET() {
  const challenge = randomToken(24);
  const store = await cookies();
  store.set("tpt_auth_challenge", challenge, {
    httpOnly: true,
    sameSite: "lax",
    secure: process.env.NODE_ENV === "production",
    path: "/",
    maxAge: 300,
  });
  return NextResponse.json({
    challenge,
    identityServerUrl: PORTAL_CONFIG.identityServerUrl,
  });
}
