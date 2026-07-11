"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import {
  generateWallet,
  getWallet,
  signChallenge,
  type Wallet,
} from "@/app/lib/wallet";
import { loginDemo } from "@/app/lib/auth-actions";
import { SCENARIOS, type ScenarioId } from "@/app/lib/mock-data";
import { PORTAL_CONFIG } from "@/app/lib/config";
import { useLanguage } from "@/app/components/LanguageProvider";

export default function LoginPage() {
  const { t } = useLanguage();
  const router = useRouter();
  const demoMode = PORTAL_CONFIG.demoMode;
  const [wallet, setWallet] = useState<Wallet | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setWallet(getWallet());
  }, []);

  async function handleCreate() {
    setBusy(true);
    setError(null);
    try {
      const w = await generateWallet();
      setWallet(w);
    } catch (e) {
      setError(`Could not create identity: ${String(e)}`);
    } finally {
      setBusy(false);
    }
  }

  async function handleSignIn() {
    if (!wallet) return;
    setBusy(true);
    setError(null);
    try {
      const challengeRes = await fetch("/api/auth/challenge");
      if (!challengeRes.ok) throw new Error("Could not start sign-in.");
      const { challenge } = await challengeRes.json();
      const signature = await signChallenge(challenge);
      const verifyRes = await fetch("/api/auth/verify", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          did: wallet.did,
          publicKey: wallet.publicKeyB64,
          signature,
          challenge,
        }),
      });
      if (!verifyRes.ok) {
        const data = await verifyRes.json().catch(() => ({}));
        throw new Error(data.error ?? "Sign-in failed.");
      }
      router.push("/dashboard");
      router.refresh();
    } catch (e) {
      setError(String(e instanceof Error ? e.message : e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div>
      <h1>{t("login")}</h1>

      {demoMode ? (
        <section className="card">
          <h2>Demo sign-in</h2>
          <p>
            This demo runs on fictional data. Choose a scenario to explore the portal as Alex
            Tane.
          </p>
          <div className="card-grid">
            {SCENARIOS.map((s) => (
              <button
                key={s.id}
                type="button"
                className="card"
                style={{ textAlign: "left", cursor: "pointer" }}
                onClick={() => loginDemo(s.id as ScenarioId)}
              >
                <h3 style={{ margin: "0 0 0.25rem" }}>{s.label}</h3>
                <p style={{ margin: 0, color: "var(--muted)" }}>{s.description}</p>
              </button>
            ))}
          </div>
        </section>
      ) : (
        <section className="card">
          <h2>Your digital identity</h2>
          <p>
            My Gov NZ uses a self-custodied wallet. Your private key is generated in your browser
            and never leaves this device. Your Decentralised Identifier (DID) is derived from your
            public key.
          </p>

          {!wallet ? (
            <button type="button" className="btn" onClick={handleCreate} disabled={busy}>
              {busy ? "Creating…" : "Create your digital identity"}
            </button>
          ) : (
            <>
              <p>
                Identity ready. Your DID:
                <br />
                <code>{wallet.did}</code>
              </p>
              <button type="button" className="btn" onClick={handleSignIn} disabled={busy}>
                {busy ? "Signing in…" : "Sign in"}
              </button>
            </>
          )}

          {error && (
            <p className="alert alert--warn" role="alert">
              {error}
            </p>
          )}
        </section>
      )}
    </div>
  );
}
