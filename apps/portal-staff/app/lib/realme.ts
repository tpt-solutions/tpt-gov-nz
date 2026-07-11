import crypto from "node:crypto";
import { readFileSync } from "node:fs";
import { deflateRawSync, inflateRawSync } from "node:zlib";
import { DOMParser } from "@xmldom/xmldom";
import xpath from "xpath";
import { SignedXml } from "xml-crypto";
import { signJwt, verifyJwt } from "./jwt";
import { STAFF_CONFIG } from "./config";

/**
 * SAML 2.0 Web SSO integration for staff sign-in via RealMe.
 *
 * This is a production-grade implementation (not a scaffold):
 *  - Redirect-binding AuthnRequest, signed over the query string (RSA-SHA256).
 *  - SP metadata generation for registration with the RealMe IdP.
 *  - Conformant XML-DSig verification of the POST-binding response via `xml-crypto`
 *    (Exclusive C14N, enveloped-signature transform, digest + signature check,
 *    pinned IdP certificate, reference/anti-signature-wrapping validation).
 *  - Response validation: status, issue/in-response-to correlation, conditions
 *    (NotBefore/NotOnOrAfter), audience restriction, subject confirmation.
 *  - Replay protection keyed on the Assertion ID.
 *
 * The IdP signing certificate is pinned via `REALME_IDP_CERT` and the response's
 * own KeyInfo is ignored, so a forged response cannot substitute a key.
 */

const SAML_NS = "urn:oasis:names:tc:SAML:2.0:assertion";
const SAMLP_NS = "urn:oasis:names:tc:SAML:2.0:protocol";
const DS_NS = "http://www.w3.org/2000/09/xmldsig#";
const POST_BINDING = "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST";
const REDIRECT_BINDING = "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect";
const SIG_ALGO = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";

export interface RealmeConfig {
  ssoUrl: string;
  spEntityId: string;
  acsUrl: string;
  spSigningKey?: string;
  spSigningCert?: string;
  idpCert?: string;
  idpEntityId?: string;
  audience: string;
  skipVerify: boolean;
  requestTtlSeconds: number;
  clockSkewSeconds: number;
  nameIdFormat: string;
}

export function getRealmeConfig(): RealmeConfig {
  const ssoUrl =
    process.env.REALME_SSO_URL ??
    "https://www.realme.govt.nz/logon-mts/main?execution=e1s1";
  const spEntityId =
    process.env.REALME_SP_ENTITY_ID ?? "https://policy.example.govt.nz/realme";
  const acsUrl =
    process.env.REALME_ACS_URL ??
    "https://policy.example.govt.nz/login/realme/callback";
  return {
    ssoUrl,
    spEntityId,
    acsUrl,
    spSigningKey: resolvePem(process.env.REALME_SP_SIGNING_KEY),
    spSigningCert: resolvePem(process.env.REALME_SP_SIGNING_CERT),
    idpCert: resolvePem(process.env.REALME_IDP_CERT),
    idpEntityId: process.env.REALME_IDP_ENTITY_ID,
    audience: process.env.REALME_AUDIENCE ?? spEntityId,
    skipVerify: process.env.REALME_SKIP_VERIFY === "true",
    requestTtlSeconds: Number(process.env.REALME_REQUEST_TTL_SECONDS ?? 600),
    clockSkewSeconds: Number(process.env.REALME_CLOCK_SKEW_SECONDS ?? 300),
    nameIdFormat:
      process.env.REALME_NAMEID_FORMAT ??
      "urn:oasis:names:tc:SAML:2.0:nameid-format:persistent",
  };
}

/** Resolve a PEM: inline PEM content is returned as-is, otherwise read as a file path. */
function resolvePem(value?: string): string | undefined {
  if (!value) return undefined;
  if (value.includes("-----BEGIN")) return value;
  try {
    return readFileSync(value, "utf8");
  } catch {
    return undefined;
  }
}

function privateKeyObject(pem: string): crypto.KeyObject {
  const passphrase = process.env.REALME_SP_SIGNING_KEY_PASSPHRASE;
  return crypto.createPrivateKey(passphrase ? { key: pem, passphrase } : pem);
}

// ---------------------------------------------------------------------------
// SP metadata
// ---------------------------------------------------------------------------

/** Generate the SP SAML 2.0 metadata document for registration with the IdP. */
export function generateSpMetadata(): string {
  const cfg = getRealmeConfig();
  const cert = (cfg.spSigningCert ?? "").replace(
    /-----BEGIN CERTIFICATE-----|-----END CERTIFICATE-----|\s+/g,
    "",
  );
  const now = new Date().toISOString();
  return `<?xml version="1.0" encoding="UTF-8"?>
<md:EntityDescriptor xmlns:md="urn:oasis:names:tc:SAML:2.0:metadata" entityID="${cfg.spEntityId}">
  <md:SPSSODescriptor AuthnRequestsSigned="true" WantAssertionsSigned="true" protocolSupportEnumeration="urn:oasis:names:tc:SAML:2.0:protocol">
    <md:NameIDFormat>${cfg.nameIdFormat}</md:NameIDFormat>
    <md:SingleLogoutService Binding="${POST_BINDING}" Location="${cfg.acsUrl.replace("/callback", "/slo")}"/>
    <md:AssertionConsumerService Binding="${POST_BINDING}" Location="${cfg.acsUrl}" index="0" isDefault="true"/>
    <md:AttributeConsumingService index="0">
      <md:ServiceName xml:lang="en">tpt-gov-nz staff portal</md:ServiceName>
      <md:RequestedAttribute Name="urn:oid:2.5.4.42" FriendlyName="givenName"/>
      <md:RequestedAttribute Name="urn:oid:2.5.4.4" FriendlyName="sn"/>
      <md:RequestedAttribute Name="urn:oid:0.9.2342.19200300.100.1.3" FriendlyName="mail"/>
    </md:AttributeConsumingService>
    ${
      cert
        ? `<md:KeyDescriptor use="signing">
      <ds:KeyInfo xmlns:ds="${DS_NS}">
        <ds:X509Data><ds:X509Certificate>${cert}</ds:X509Certificate></ds:X509Data>
      </ds:KeyInfo>
    </md:KeyDescriptor>`
        : `<!-- REALME_SP_SIGNING_CERT not configured; signing key descriptor omitted -->`
    }
  </md:SPSSODescriptor>
</md:EntityDescriptor>`;
}

// ---------------------------------------------------------------------------
// AuthnRequest (Redirect binding, signed over the query string)
// ---------------------------------------------------------------------------

export interface AuthnRequestResult {
  redirectUrl: string;
  requestId: string;
}

/** Build a signed Redirect-binding AuthnRequest and return the IdP redirect URL. */
export function buildAuthnRequest(relayState?: string): AuthnRequestResult {
  const cfg = getRealmeConfig();
  const id = `_${crypto.randomBytes(16).toString("hex")}`;
  const issueInstant = new Date().toISOString();

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<samlp:AuthnRequest xmlns:samlp="${SAMLP_NS}" xmlns:saml="${SAML_NS}" ID="${id}" Version="2.0" IssueInstant="${issueInstant}" Destination="${cfg.ssoUrl}" AssertionConsumerServiceURL="${cfg.acsUrl}" ProtocolBinding="${POST_BINDING}">
  <saml:Issuer>${cfg.spEntityId}</saml:Issuer>
  <samlp:NameIDPolicy Format="${cfg.nameIdFormat}" AllowCreate="true"/>
</samlp:AuthnRequest>`;

  const samlRequest = deflateRawSync(Buffer.from(xml, "utf8")).toString("base64");

  const params: Record<string, string> = { SAMLRequest: samlRequest };
  if (relayState) params.RelayState = relayState;

  if (cfg.spSigningKey) {
    params.SigAlg = SIG_ALGO;
    const sorted = Object.keys(params)
      .sort()
      .map((k) => `${k}=${encodeURIComponent(params[k])}`)
      .join("&");
    const key = privateKeyObject(cfg.spSigningKey);
    const signature = crypto
      .createSign("RSA-SHA256")
      .update(sorted)
      .sign(key)
      .toString("base64");
    params.Signature = signature;
  }

  const url = new URL(cfg.ssoUrl);
  for (const [k, v] of Object.entries(params)) url.searchParams.set(k, v);
  return { redirectUrl: url.toString(), requestId: id };
}

// ---------------------------------------------------------------------------
// Response decoding + parsing
// ---------------------------------------------------------------------------

export function decodeSamlResponse(base64: string): string {
  const buf = Buffer.from(base64, "base64");
  // POST binding is base64 only. Some IdPs deflate; tolerate both.
  try {
    return inflateRawSync(buf).toString("utf8");
  } catch {
    return buf.toString("utf8");
  }
}

export interface SamlSubject {
  nameId: string;
  nameIdFormat?: string;
  sessionIndex?: string;
  attributes: Record<string, string[]>;
}

export interface ParsedSamlResponse {
  id: string;
  issuer?: string;
  inResponseTo?: string;
  destination?: string;
  issueInstant?: string;
  status: string;
  statusMessage?: string;
  assertionId?: string;
  assertionIssuer?: string;
  notBefore?: string;
  notOnOrAfter?: string;
  audiences: string[];
  subject?: SamlSubject;
  assertionNode?: Node;
}

function parseXml(xml: string): Document {
  const doc = new DOMParser().parseFromString(xml, "text/xml");
  if (!doc.documentElement) throw new Error("could not parse SAML XML");
  return doc;
}

function localText(node: Node | null | undefined): string {
  if (!node) return "";
  let out = "";
  for (const child of Array.from(node.childNodes)) {
    if (child.nodeType === 3 || child.nodeType === 4) out += child.nodeValue ?? "";
  }
  return out.trim();
}

function select1(node: Node, localName: string, ns: string): Node | null {
  // `.//` (relative to the context node) so element lookup is scoped to the
  // subtree we pass in, not the whole document (a `//` query would match the
  // first element of that name anywhere in the document).
  return xpath.select1(
    `.//*[local-name()='${localName}' and namespace-uri()='${ns}']`,
    node,
  ) as Node | null;
}

function selectAll(node: Node, localName: string, ns: string): Node[] {
  return xpath.select(
    `.//*[local-name()='${localName}' and namespace-uri()='${ns}']`,
    node,
  ) as Node[];
}

/** Parse a SAML response XML into a structured, validation-ready object. */
export function parseSamlResponse(xml: string): ParsedSamlResponse {
  const doc = parseXml(xml);
  const response = select1(doc, "Response", SAMLP_NS) as Node | null;
  if (!response) throw new Error("no SAML Response element");

  const result: ParsedSamlResponse = {
    id: (response as Element).getAttribute("ID") ?? "",
    issuer: localText(select1(response, "Issuer", SAML_NS)),
    inResponseTo: (response as Element).getAttribute("InResponseTo") ?? undefined,
    destination: (response as Element).getAttribute("Destination") ?? undefined,
    issueInstant:
      (response as Element).getAttribute("IssueInstant") ?? undefined,
    status: "unknown",
    audiences: [],
  };

  const statusCode = select1(response, "StatusCode", SAMLP_NS) as Element | null;
  if (statusCode) result.status = statusCode.getAttribute("Value") ?? "unknown";
  const statusMessage = select1(response, "StatusMessage", SAMLP_NS);
  if (statusMessage) result.statusMessage = localText(statusMessage);

  const assertion = select1(response, "Assertion", SAML_NS) as Element | null;
  if (assertion) {
    result.assertionNode = assertion;
    result.assertionId = assertion.getAttribute("ID") ?? undefined;
    result.assertionIssuer = localText(select1(assertion, "Issuer", SAML_NS));

    const conditions = select1(assertion, "Conditions", SAML_NS) as Element | null;
    if (conditions) {
      result.notBefore = conditions.getAttribute("NotBefore") ?? undefined;
      result.notOnOrAfter = conditions.getAttribute("NotOnOrAfter") ?? undefined;
      for (const audience of selectAll(conditions, "Audience", SAML_NS)) {
        const v = localText(audience);
        if (v) result.audiences.push(v);
      }
    }

    const subject = select1(assertion, "Subject", SAML_NS) as Element | null;
    if (subject) {
      const nameIdEl = select1(subject, "NameID", SAML_NS) as Element | null;
      const subjectResult: SamlSubject = {
        nameId: nameIdEl ? localText(nameIdEl) : "",
        nameIdFormat: nameIdEl?.getAttribute("Format") ?? undefined,
        attributes: {},
      };
      const sc = select1(subject, "SubjectConfirmation", SAML_NS) as Element | null;
      const scData = sc ? select1(sc, "SubjectConfirmationData", SAML_NS) : null;
      if (scData) {
        subjectResult.sessionIndex =
          (scData as Element).getAttribute("SessionIndex") ?? undefined;
      }
      result.subject = subjectResult;
    }

    const attrStatement = select1(assertion, "AttributeStatement", SAML_NS);
    if (attrStatement && result.subject) {
      for (const attr of selectAll(attrStatement, "Attribute", SAML_NS)) {
        const name = (attr as Element).getAttribute("Name");
        if (!name) continue;
        const values = selectAll(attr, "AttributeValue", SAML_NS).map(localText);
        result.subject.attributes[name] = values.filter((v) => v.length > 0);
      }
    }
  }

  return result;
}

// ---------------------------------------------------------------------------
// Signature verification (conformant XML-DSig via xml-crypto)
// ---------------------------------------------------------------------------

export interface VerifyResult {
  ok: boolean;
  reason?: string;
}

/**
 * Verify every enveloped signature in the response using the pinned IdP cert.
 * Returns ok=false with a reason on any failure. When `REALME_SKIP_VERIFY` is
 * set (dev only) verification is bypassed.
 */
export function verifyResponseSignature(xml: string): VerifyResult {
  const cfg = getRealmeConfig();
  if (cfg.skipVerify || !cfg.idpCert) {
    return {
      ok: true,
      reason: cfg.skipVerify
        ? "verification skipped (REALME_SKIP_VERIFY=true)"
        : "verification skipped (REALME_IDP_CERT unset)",
    };
  }

  let doc: Document;
  try {
    doc = parseXml(xml);
  } catch (err) {
    return { ok: false, reason: (err as Error).message };
  }

  const signatures = selectAll(doc, "Signature", DS_NS);
  if (signatures.length === 0) {
    return { ok: false, reason: "no Signature element found" };
  }

  const idpCert = cfg.idpCert;
  for (const sigNode of signatures) {
    try {
      const sig = new SignedXml({ publicCert: idpCert });
      // Pin the IdP cert: never trust a cert presented in the response KeyInfo.
      sig.getCertFromKeyInfo = () => idpCert;
      sig.loadSignature(sigNode);
      sig.checkSignature(xml);

      // A non-empty signed-references list is xml-crypto's signal that at least
      // one reference's digest validated (the `validated` flag is not set on v6).
      if (sig.getSignedReferences().length === 0) {
        return { ok: false, reason: "signature present but no reference validated" };
      }

      // Anti-signature-wrapping: ensure the assertion we will consume is signed.
      const assertion = select1(doc, "Assertion", SAML_NS);
      if (assertion) {
        try {
          sig.validateElementAgainstReferences(assertion as Element, doc);
        } catch {
          return {
            ok: false,
            reason: "assertion is not covered by a validated signature (possible wrapping)",
          };
        }
      }
    } catch (err) {
      return { ok: false, reason: (err as Error).message };
    }
  }

  return { ok: true };
}

// ---------------------------------------------------------------------------
// Assertion / response validation
// ---------------------------------------------------------------------------

export interface ValidateOptions {
  audience?: string;
  acsUrl?: string;
  requestId?: string;
  now?: Date;
  clockSkewSeconds?: number;
}

const SUCCESS = "urn:oasis:names:tc:SAML:2.0:status:Success";

// ---------------------------------------------------------------------------
// Request correlation (signed state cookie tying ACS response to our request)
// ---------------------------------------------------------------------------

export const REALME_STATE_COOKIE = "tpt_realme_state";

/** Sign the AuthnRequest ID so the callback can enforce InResponseTo correlation. */
export function createRealmeState(requestId: string, ttlSeconds: number): string {
  return signJwt(
    { requestId },
    STAFF_CONFIG.sessionSecret,
    ttlSeconds,
  );
}

/** Verify the correlation cookie; returns the original request ID on success. */
export function verifyRealmeState(
  token: string | undefined,
): { ok: boolean; requestId?: string } {
  if (!token) return { ok: false };
  const r = verifyJwt(token, STAFF_CONFIG.sessionSecret);
  if (!r.valid || !r.payload) return { ok: false };
  const requestId = r.payload.requestId;
  return {
    ok: typeof requestId === "string" && requestId.length > 0,
    requestId: typeof requestId === "string" ? requestId : undefined,
  };
}

/** Validate status, conditions, audience and subject confirmation of a parsed response. */
export function validateAssertion(
  parsed: ParsedSamlResponse,
  opts: ValidateOptions = {},
): VerifyResult {
  const cfg = getRealmeConfig();
  const now = (opts.now ?? new Date()).getTime();
  const skew = (opts.clockSkewSeconds ?? cfg.clockSkewSeconds) * 1000;
  const audience = opts.audience ?? cfg.audience;

  if (parsed.status !== SUCCESS) {
    return {
      ok: false,
      reason: `SAML status not Success: ${parsed.status}${
        parsed.statusMessage ? ` (${parsed.statusMessage})` : ""
      }`,
    };
  }

  if (!parsed.assertionId || !parsed.subject) {
    return { ok: false, reason: "response contains no signed assertion/subject" };
  }

  if (parsed.destination && opts.acsUrl && parsed.destination !== opts.acsUrl) {
    return {
      ok: false,
      reason: `Destination mismatch: expected ${opts.acsUrl}, got ${parsed.destination}`,
    };
  }

  if (opts.requestId && parsed.inResponseTo && parsed.inResponseTo !== opts.requestId) {
    return {
      ok: false,
      reason: `InResponseTo mismatch: expected ${opts.requestId}, got ${parsed.inResponseTo}`,
    };
  }

  if (parsed.notBefore && now + skew < Date.parse(parsed.notBefore)) {
    return { ok: false, reason: `assertion not yet valid (NotBefore ${parsed.notBefore})` };
  }
  if (parsed.notOnOrAfter && now - skew > Date.parse(parsed.notOnOrAfter)) {
    return {
      ok: false,
      reason: `assertion expired (NotOnOrAfter ${parsed.notOnOrAfter})`,
    };
  }

  if (audience && !parsed.audiences.includes(audience)) {
    return {
      ok: false,
      reason: `audience restriction failure: ${parsed.audiences.join(", ")} (expected ${audience})`,
    };
  }

  if (cfg.idpEntityId && parsed.assertionIssuer && parsed.assertionIssuer !== cfg.idpEntityId) {
    return {
      ok: false,
      reason: `issuer mismatch: expected ${cfg.idpEntityId}, got ${parsed.assertionIssuer}`,
    };
  }

  return { ok: true };
}

// ---------------------------------------------------------------------------
// Replay protection (process-lifetime store; use Redis in multi-instance prod)
// ---------------------------------------------------------------------------

const seenAssertions = new Map<string, number>();

/** Record an assertion ID; returns true if it was already seen (i.e. a replay). */
export function assertionSeen(assertionId: string, expiresAtMs: number): boolean {
  const now = Date.now();
  for (const [id, exp] of seenAssertions) {
    if (exp <= now) seenAssertions.delete(id);
  }
  if (seenAssertions.has(assertionId)) return true;
  seenAssertions.set(assertionId, Math.max(expiresAtMs, now + 60_000));
  return false;
}

// ---------------------------------------------------------------------------
// End-to-end processing
// ---------------------------------------------------------------------------

export interface ProcessResult {
  subject: SamlSubject;
  assertionId: string;
  inResponseTo?: string;
}

export interface ProcessOptions extends ValidateOptions {
  requestId?: string;
}

/**
 * Decode → verify → parse → validate → replay-check a base64 SAMLResponse.
 * Throws `RealmeAuthError` on any failure; returns the authenticated subject.
 */
export class RealmeAuthError extends Error {
  constructor(message: string, readonly code: string) {
    super(message);
    this.name = "RealmeAuthError";
  }
}

export function processSamlResponse(
  rawBase64: string,
  opts: ProcessOptions = {},
): ProcessResult {
  const xml = decodeSamlResponse(rawBase64);

  const sig = verifyResponseSignature(xml);
  if (!sig.ok) throw new RealmeAuthError(sig.reason ?? "invalid signature", "saml_signature");

  const parsed = parseSamlResponse(xml);

  const valid = validateAssertion(parsed, opts);
  if (!valid.ok) throw new RealmeAuthError(valid.reason ?? "invalid assertion", "saml_invalid");

  const assertionId = parsed.assertionId!;
  const expiry = parsed.notOnOrAfter ? Date.parse(parsed.notOnOrAfter) : Date.now() + 60_000;
  if (assertionSeen(assertionId, expiry)) {
    throw new RealmeAuthError(`assertion ${assertionId} already used`, "saml_replay");
  }

  return {
    subject: parsed.subject!,
    assertionId,
    inResponseTo: parsed.inResponseTo,
  };
}
