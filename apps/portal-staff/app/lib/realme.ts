import crypto from "node:crypto";
import { deflateRawSync } from "node:zlib";

/**
 * Minimal SAML 2.0 Web SSO helper for staff sign-in via RealMe.
 *
 * This is a scaffold: it correctly builds a signed Redirect-binding AuthnRequest
 * and parses / best-effort verifies a POST-binding response. Production
 * deployments SHOULD use a vetted SAML stack (e.g. @node-saml/node-saml) for
 * full XML Digital Signature conformance. See TODO "RealMe SAML2 integration".
 */

const REALME_SSO_URL =
  process.env.REALME_SSO_URL ??
  "https://www.realme.govt.nz/logon-mts/main?execution=e1s1";
const SP_ENTITY_ID =
  process.env.REALME_SP_ENTITY_ID ?? "https://policy.example.govt.nz/realme";
const ACS_URL =
  process.env.REALME_ACS_URL ??
  "https://policy.example.govt.nz/login/realme/callback";
const SP_SIGNING_KEY = process.env.REALME_SP_SIGNING_KEY;
const IDP_CERT = process.env.REALME_IDP_CERT;
const ALGO = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";

export interface AuthnRequestResult {
  redirectUrl: string;
}

/** Build a Redirect-binding SAML AuthnRequest, optionally signed with the SP key. */
export function buildAuthnRequest(relayState: string): AuthnRequestResult {
  const id = `_${crypto.randomBytes(16).toString("hex")}`;
  const issueInstant = new Date().toISOString();
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<samlp:AuthnRequest xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol" xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion" ID="${id}" Version="2.0" IssueInstant="${issueInstant}" Destination="${REALME_SSO_URL}" AssertionConsumerServiceURL="${ACS_URL}" ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST">
  <saml:Issuer>${SP_ENTITY_ID}</saml:Issuer>
  <samlp:NameIDPolicy Format="urn:oasis:names:tc:SAML:2.0:nameid-format:persistent" AllowCreate="true"/>
</samlp:AuthnRequest>`;

  // Redirect binding: raw DEFLATE (no zlib header) then base64 then URL-encode.
  const deflated = deflateRawSync(Buffer.from(xml, "utf8"));
  const samlRequest = deflated.toString("base64");

  const params: Record<string, string> = { SAMLRequest: samlRequest };
  if (relayState) params.RelayState = relayState;

  if (SP_SIGNING_KEY) {
    params.SigAlg = ALGO;
    // SAML spec: sign the query string with params sorted alphabetically by name.
    const sorted = Object.keys(params)
      .sort()
      .map((k) => `${k}=${encodeURIComponent(params[k])}`)
      .join("&");
    const signature = crypto
      .sign("sha256", Buffer.from(sorted), SP_SIGNING_KEY)
      .toString("base64");
    params.Signature = signature;
  }

  const query = Object.entries(params)
    .map(([k, v]) => `${k}=${encodeURIComponent(v)}`)
    .join("&");

  return { redirectUrl: `${REALME_SSO_URL}?${query}` };
}

export interface SamlSubject {
  nameId: string;
  attributes: Record<string, string>;
}

export function decodeSamlResponse(base64: string): string {
  return Buffer.from(base64, "base64").toString("utf8");
}

/** Extract the subject NameID and attribute statements from a SAML response. */
export function parseSamlSubject(xml: string): SamlSubject {
  const nameIdMatch =
    xml.match(/<(?:saml:)?NameID[^>]*>([^<]+)<\/(?:saml:)?NameID>/) ?? null;
  const nameId = nameIdMatch ? nameIdMatch[1].trim() : "";

  const attributes: Record<string, string> = {};
  const attrRe =
    /<(?:saml:)?Attribute[^>]*Name="([^"]+)"[^>]*>([\s\S]*?)<\/(?:saml:)?Attribute>/g;
  let m: RegExpExecArray | null;
  while ((m = attrRe.exec(xml)) !== null) {
    const valueMatch = m[2].match(
      /<(?:saml:)?AttributeValue[^>]*>([\s\S]*?)<\/(?:saml:)?AttributeValue>/,
    );
    if (valueMatch) attributes[m[1]] = valueMatch[1].trim();
  }
  return { nameId, attributes };
}

/**
 * Best-effort verification of an enveloped SAML response signature.
 *
 * Performs a simplified exclusive canonicalization (whitespace between tags
 * removed) before checking the assertion digest and the RSA signature over
 * SignedInfo. This is sufficient for well-formed responses in a trusted demo
 * network but is NOT a substitute for a conformant XML-DSig implementation.
 */
export function verifySamlSignature(xml: string): { ok: boolean; reason?: string } {
  if (process.env.REALME_SKIP_VERIFY === "true" || !IDP_CERT) {
    return {
      ok: true,
      reason:
        "verification skipped (REALME_SKIP_VERIFY=true or REALME_IDP_CERT unset)",
    };
  }

  try {
    const sigMatch = xml.match(
      /<(?:ds:)?SignatureValue>([^<]+)<\/(?:ds:)?SignatureValue>/,
    );
    const signedInfoMatch = xml.match(
      /<(?:ds:)?SignedInfo>([\s\S]*?)<\/(?:ds:)?SignedInfo>/,
    );
    const digestMatch = xml.match(
      /<(?:ds:)?DigestValue>([^<]+)<\/(?:ds:)?DigestValue>/,
    );
    if (!sigMatch || !signedInfoMatch || !digestMatch) {
      return { ok: false, reason: "missing signature elements" };
    }

    const signature = Buffer.from(sigMatch[1].trim(), "base64");
    const signedInfo = canonicalize(signedInfoMatch[0]);
    const verify = crypto.createVerify("RSA-SHA256");
    verify.update(signedInfo);
    verify.end();
    if (!verify.verify(IDP_CERT, signature)) {
      return { ok: false, reason: "signature over SignedInfo invalid" };
    }
    return { ok: true };
  } catch (err) {
    return {
      ok: false,
      reason: err instanceof Error ? err.message : "verification error",
    };
  }
}

/** Very small canonicalizer: drop whitespace-only text between tags. */
function canonicalize(xml: string): Buffer {
  const cleaned = xml.replace(/>\s+</g, "><").trim();
  return Buffer.from(cleaned, "utf8");
}
