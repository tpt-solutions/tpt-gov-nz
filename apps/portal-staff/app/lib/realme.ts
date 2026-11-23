import crypto from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
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
const MAIL_OID = "urn:oid:0.9.2342.19200300.100.1.3";
const SIG_ALGO = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";

export interface RealmeConfig {
  ssoUrl: string;
  spEntityId: string;
  acsUrl: string;
  spSigningKey?: string | undefined;
  spSigningCert?: string | undefined;
  idpCert?: string | undefined;
  idpEntityId?: string | undefined;
  audience: string;
  skipVerify: boolean;
  requestTtlSeconds: number;
  clockSkewSeconds: number;
  nameIdFormat: string;
  /** IdP Single Logout endpoint (HTTP-Redirect). Logout stays local when unset. */
  sloUrl?: string | undefined;
  /**
   * Case workers allowed to sign in via RealMe, matched case-insensitively against
   * the NameID or the mail attribute. Empty means nobody is allowed.
   */
  staffAllowlist: string[];
}

export function getRealmeConfig(): RealmeConfig {
  // Unverified SAML must never be accepted in production, whatever the environment says.
  if (process.env.REALME_SKIP_VERIFY === "true" && process.env.NODE_ENV === "production") {
    throw new Error("REALME_SKIP_VERIFY is not allowed in production");
  }
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
    sloUrl: process.env.REALME_SLO_URL || undefined,
    staffAllowlist: (process.env.REALME_STAFF_ALLOWLIST ?? "")
      .split(",")
      .map((s) => s.trim().toLowerCase())
      .filter((s) => s.length > 0),
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
  return `<?xml version="1.0" encoding="UTF-8"?>
<md:EntityDescriptor xmlns:md="urn:oasis:names:tc:SAML:2.0:metadata" entityID="${cfg.spEntityId}">
  <md:SPSSODescriptor AuthnRequestsSigned="true" WantAssertionsSigned="true" protocolSupportEnumeration="urn:oasis:names:tc:SAML:2.0:protocol">
    <md:NameIDFormat>${cfg.nameIdFormat}</md:NameIDFormat>
    <md:SingleLogoutService Binding="${REDIRECT_BINDING}" Location="${cfg.acsUrl.replace("/callback", "/slo")}"/>
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
/**
 * Keep RelayState only if it is a same-origin relative path (for example `/citizens`).
 * Anything else, including absolute URLs and protocol-relative `//host` paths, is dropped.
 */
export function safeRelayState(relay: string | null | undefined): string {
  if (!relay || relay.length > 512) return "";
  // Must start with a single "/", contain no whitespace or backslash.
  if (!/^\/(?![/\\])[^\s\\]*$/.test(relay)) return "";
  return relay;
}

export function buildAuthnRequest(relayState?: string): AuthnRequestResult {
  const cfg = getRealmeConfig();
  const id = `_${crypto.randomBytes(16).toString("hex")}`;
  const issueInstant = new Date().toISOString();

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<samlp:AuthnRequest xmlns:samlp="${SAMLP_NS}" xmlns:saml="${SAML_NS}" ID="${id}" Version="2.0" IssueInstant="${issueInstant}" Destination="${cfg.ssoUrl}" AssertionConsumerServiceURL="${cfg.acsUrl}" ProtocolBinding="${POST_BINDING}">
  <saml:Issuer>${cfg.spEntityId}</saml:Issuer>
  <samlp:NameIDPolicy Format="${cfg.nameIdFormat}" AllowCreate="true"/>
</samlp:AuthnRequest>`;

  const url = new URL(cfg.ssoUrl);
  for (const [k, v] of redirectBindingParams("SAMLRequest", xml, relayState)) {
    url.searchParams.set(k, v);
  }
  return { redirectUrl: url.toString(), requestId: id };
}

/**
 * Redirect-binding parameters (SAML 2.0 bindings §3.4.4.1), signed when an SP key
 * is configured. The signature covers `SAMLRequest|SAMLResponse`, `RelayState`,
 * `SigAlg` in that order, each URL-encoded, so the order here is part of the contract.
 */
function redirectBindingParams(
  messageParam: "SAMLRequest" | "SAMLResponse",
  xml: string,
  relayState?: string,
): Array<[string, string]> {
  const cfg = getRealmeConfig();
  const params: Array<[string, string]> = [
    [messageParam, deflateRawSync(Buffer.from(xml, "utf8")).toString("base64")],
  ];
  if (relayState) params.push(["RelayState", relayState]);
  if (cfg.spSigningKey) {
    params.push(["SigAlg", SIG_ALGO]);
    const signedString = params.map(([k, v]) => `${k}=${encodeURIComponent(v)}`).join("&");
    const signature = crypto
      .createSign("RSA-SHA256")
      .update(signedString)
      .sign(privateKeyObject(cfg.spSigningKey))
      .toString("base64");
    params.push(["Signature", signature]);
  }
  return params;
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
  nameIdFormat?: string | undefined;
  /** From AuthnStatement (SAML 2.0 core §4.1.4.2), needed for single logout. */
  sessionIndex?: string | undefined;
  /** SubjectConfirmation method, Recipient, InResponseTo and NotOnOrAfter. */
  confirmationMethod?: string | undefined;
  confirmationRecipient?: string | undefined;
  confirmationInResponseTo?: string | undefined;
  confirmationNotOnOrAfter?: string | undefined;
  attributes: Record<string, string[]>;
}

export interface ParsedSamlResponse {
  id: string;
  issuer?: string | undefined;
  inResponseTo?: string | undefined;
  destination?: string | undefined;
  issueInstant?: string | undefined;
  status: string;
  statusMessage?: string | undefined;
  assertionId?: string | undefined;
  assertionIssuer?: string | undefined;
  notBefore?: string | undefined;
  notOnOrAfter?: string | undefined;
  audiences: string[];
  subject?: SamlSubject | undefined;
  assertionNode?: Node | undefined;
}

function parseXml(xml: string): Document {
  // Entity declarations enable XML bombs and external entity tricks. SAML never needs them.
  if (/<!DOCTYPE/i.test(xml)) throw new Error("DOCTYPE declarations are not allowed in SAML responses");
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
      if (sc) {
        subjectResult.confirmationMethod = sc.getAttribute("Method") ?? undefined;
        const scData = select1(sc, "SubjectConfirmationData", SAML_NS) as Element | null;
        if (scData) {
          subjectResult.confirmationRecipient = scData.getAttribute("Recipient") ?? undefined;
          subjectResult.confirmationInResponseTo = scData.getAttribute("InResponseTo") ?? undefined;
          subjectResult.confirmationNotOnOrAfter = scData.getAttribute("NotOnOrAfter") ?? undefined;
        }
      }
      const authn = select1(assertion, "AuthnStatement", SAML_NS) as Element | null;
      if (authn) {
        subjectResult.sessionIndex = authn.getAttribute("SessionIndex") ?? undefined;
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
  if (cfg.skipVerify) {
    // Development only: getRealmeConfig refuses this in production.
    return { ok: true, reason: "verification skipped (REALME_SKIP_VERIFY=true)" };
  }
  if (!cfg.idpCert) {
    // Fail closed: an unpinned IdP certificate means the response cannot be trusted.
    return { ok: false, reason: "REALME_IDP_CERT is not configured; refusing unverified SAML response" };
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
  audience?: string | undefined;
  acsUrl?: string | undefined;
  requestId?: string | undefined;
  now?: Date | undefined;
  clockSkewSeconds?: number | undefined;
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
): { ok: boolean; requestId?: string | undefined } {
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

  if (opts.acsUrl && parsed.destination !== opts.acsUrl) {
    return {
      ok: false,
      reason: `Destination mismatch: expected ${opts.acsUrl}, got ${parsed.destination}`,
    };
  }

  if (opts.requestId && parsed.inResponseTo !== opts.requestId) {
    return {
      ok: false,
      reason: `InResponseTo mismatch: expected ${opts.requestId}, got ${parsed.inResponseTo}`,
    };
  }

  // Bearer subject confirmation: the only method a browser SSO response can use,
  // and the Recipient/InResponseTo/NotOnOrAfter checks below only bind the
  // assertion to this ACS and this request when they are present.
  const subj = parsed.subject;
  if (subj?.confirmationMethod !== "urn:oasis:names:tc:SAML:2.0:cm:bearer") {
    return { ok: false, reason: `subject confirmation method is not bearer: ${subj?.confirmationMethod}` };
  }
  if (!subj.confirmationNotOnOrAfter) {
    return { ok: false, reason: "SubjectConfirmationData has no NotOnOrAfter" };
  }
  if (now - skew > Date.parse(subj.confirmationNotOnOrAfter)) {
    return { ok: false, reason: `subject confirmation expired (NotOnOrAfter ${subj.confirmationNotOnOrAfter})` };
  }
  if (opts.acsUrl && subj.confirmationRecipient !== opts.acsUrl) {
    return {
      ok: false,
      reason: `SubjectConfirmation Recipient mismatch: expected ${opts.acsUrl}, got ${subj.confirmationRecipient}`,
    };
  }
  if (opts.requestId && subj.confirmationInResponseTo !== opts.requestId) {
    return {
      ok: false,
      reason: `SubjectConfirmation InResponseTo mismatch: expected ${opts.requestId}, got ${subj.confirmationInResponseTo}`,
    };
  }

  if (!parsed.notOnOrAfter) {
    return { ok: false, reason: "assertion has no NotOnOrAfter condition" };
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

  if (!cfg.idpEntityId) {
    return { ok: false, reason: "REALME_IDP_ENTITY_ID is not configured" };
  }
  if (parsed.assertionIssuer !== cfg.idpEntityId) {
    return {
      ok: false,
      reason: `issuer mismatch: expected ${cfg.idpEntityId}, got ${parsed.assertionIssuer}`,
    };
  }

  return { ok: true };
}

// ---------------------------------------------------------------------------
// Replay protection: a file-backed log, so a restart cannot forget an accepted assertion.
// One line per accepted assertion: "<expiresAtMs> <assertionId>". Expired lines are
// dropped when the log is loaded. Multi-instance deployments still need a shared store.
// ---------------------------------------------------------------------------

/** Assertion IDs are NCName-like; anything else is refused before it reaches the log. */
const ASSERTION_ID_PATTERN = /^[A-Za-z0-9_.:-]{1,256}$/;

interface ReplayStore {
  path: string;
  seen: Map<string, number>;
}

let replayStore: ReplayStore | null = null;

function replayStorePath(): string {
  return process.env.REALME_REPLAY_STORE || join(process.cwd(), ".data", "realme-replay.log");
}

function openReplayStore(): ReplayStore {
  const file = replayStorePath();
  if (replayStore && replayStore.path === file) return replayStore;

  const now = Date.now();
  const seen = new Map<string, number>();
  let expired = 0;
  if (existsSync(file)) {
    for (const line of readFileSync(file, "utf8").split("\n")) {
      const sp = line.indexOf(" ");
      if (sp < 0) continue;
      const exp = Number(line.slice(0, sp));
      const id = line.slice(sp + 1);
      if (!Number.isFinite(exp) || !ASSERTION_ID_PATTERN.test(id)) continue;
      if (exp <= now) {
        expired += 1;
        continue;
      }
      seen.set(id, exp);
    }
    if (expired > 0) {
      // Compact the log so expired entries do not accumulate across restarts.
      const body = [...seen].map(([id, exp]) => `${exp} ${id}\n`).join("");
      writeFileSync(file, body, { mode: 0o600 });
    }
  }
  replayStore = { path: file, seen };
  return replayStore;
}

/**
 * Record an assertion ID. Returns true if it was already seen (a replay). Throws when the
 * log cannot be written: the caller must then refuse the assertion (fail closed).
 */
export function assertionSeen(assertionId: string, expiresAtMs: number): boolean {
  const store = openReplayStore();
  const now = Date.now();
  for (const [id, exp] of store.seen) {
    if (exp <= now) store.seen.delete(id);
  }
  if (store.seen.has(assertionId)) return true;

  const exp = Math.max(expiresAtMs, now + 60_000);
  mkdirSync(dirname(store.path), { recursive: true, mode: 0o700 });
  // Persist before accepting, so the assertion is remembered even if the process dies now.
  appendFileSync(store.path, `${exp} ${assertionId}\n`, { mode: 0o600 });
  store.seen.set(assertionId, exp);
  return false;
}

// ---------------------------------------------------------------------------
// End-to-end processing
// ---------------------------------------------------------------------------

export interface ProcessResult {
  subject: SamlSubject;
  assertionId: string;
  inResponseTo?: string | undefined;
}

export interface ProcessOptions extends ValidateOptions {
  requestId?: string | undefined;
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
  if (!ASSERTION_ID_PATTERN.test(assertionId)) {
    throw new RealmeAuthError("assertion ID has an unexpected format", "saml_invalid");
  }
  const expiry = parsed.notOnOrAfter ? Date.parse(parsed.notOnOrAfter) : Date.now() + 60_000;
  let replayed: boolean;
  try {
    replayed = assertionSeen(assertionId, expiry);
  } catch {
    throw new RealmeAuthError("replay store unavailable; refusing assertion", "saml_replay_store");
  }
  if (replayed) {
    throw new RealmeAuthError(`assertion ${assertionId} already used`, "saml_replay");
  }

  return {
    subject: parsed.subject!,
    assertionId,
    inResponseTo: parsed.inResponseTo,
  };
}

// ---------------------------------------------------------------------------
// Staff allow-list: a valid RealMe login is not enough, the person must be a case worker
// ---------------------------------------------------------------------------

/** True only when the authenticated subject is on the configured case-worker allow-list. */
export function isRealmeStaffAllowed(
  subject: SamlSubject,
  allowlist: string[] = getRealmeConfig().staffAllowlist,
): boolean {
  if (allowlist.length === 0) return false;
  const candidates = [subject.nameId, ...(subject.attributes[MAIL_OID] ?? [])]
    .map((v) => v.trim().toLowerCase())
    .filter((v) => v.length > 0);
  return candidates.some((c) => allowlist.includes(c));
}

// ---------------------------------------------------------------------------
// Single Logout (HTTP-Redirect binding, signed both ways)
// ---------------------------------------------------------------------------

/** Holds the ID of our LogoutRequest so the IdP's LogoutResponse can be correlated. */
export const REALME_SLO_COOKIE = "tpt_realme_slo";

function escapeXml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

/** Split a raw query string into [name, still-encoded value] pairs, preserving order. */
function rawQueryPairs(rawQuery: string): Array<[string, string]> {
  return rawQuery
    .replace(/^\?/, "")
    .split("&")
    .filter((p) => p.length > 0)
    .map((p) => {
      const i = p.indexOf("=");
      return i < 0 ? [p, ""] : [p.slice(0, i), p.slice(i + 1)];
    });
}

function rawParam(pairs: Array<[string, string]>, name: string): string | undefined {
  return pairs.find(([k]) => k === name)?.[1];
}

/**
 * Verify the Redirect-binding signature of an IdP message against the pinned IdP
 * certificate. The signed string is rebuilt from the raw, still-encoded values in the
 * order the IdP sent them, which is what the SAML bindings spec requires.
 */
export function verifyRedirectSignature(rawQuery: string): VerifyResult {
  const cfg = getRealmeConfig();
  if (cfg.skipVerify) {
    return { ok: true, reason: "verification skipped (REALME_SKIP_VERIFY=true)" };
  }
  if (!cfg.idpCert) {
    return { ok: false, reason: "REALME_IDP_CERT is not configured; refusing unverified SLO message" };
  }
  const pairs = rawQueryPairs(rawQuery);
  const signature = rawParam(pairs, "Signature");
  const sigAlg = rawParam(pairs, "SigAlg");
  if (signature === undefined || sigAlg === undefined) {
    return { ok: false, reason: "SLO message is not signed" };
  }
  if (decodeURIComponent(sigAlg) !== SIG_ALGO) {
    return { ok: false, reason: "unsupported SigAlg" };
  }
  const messageParam = rawParam(pairs, "SAMLRequest") !== undefined ? "SAMLRequest" : "SAMLResponse";
  const message = rawParam(pairs, messageParam);
  if (message === undefined) return { ok: false, reason: "SLO message has no SAML payload" };

  const signedParts = [`${messageParam}=${message}`];
  const relay = rawParam(pairs, "RelayState");
  if (relay !== undefined) signedParts.push(`RelayState=${relay}`);
  signedParts.push(`SigAlg=${sigAlg}`);

  let valid = false;
  try {
    valid = crypto
      .createVerify("RSA-SHA256")
      .update(signedParts.join("&"))
      .verify(cfg.idpCert, Buffer.from(decodeURIComponent(signature), "base64"));
  } catch (err) {
    return { ok: false, reason: (err as Error).message };
  }
  return valid
    ? { ok: true }
    : { ok: false, reason: "SLO signature does not verify against the pinned IdP certificate" };
}

/** Inflate the Redirect-binding payload. The output cap guards against decompression bombs. */
function decodeRedirectMessage(raw: string): string {
  const compressed = Buffer.from(decodeURIComponent(raw), "base64");
  return inflateRawSync(compressed, { maxOutputLength: 64 * 1024 }).toString("utf8");
}

export interface LogoutRequestMessage {
  id: string;
  issuer?: string | undefined;
  nameId?: string | undefined;
  sessionIndex?: string | undefined;
}

function parseLogoutRequestXml(xml: string): LogoutRequestMessage {
  const doc = parseXml(xml);
  const req = select1(doc, "LogoutRequest", SAMLP_NS) as Element | null;
  if (!req) throw new RealmeAuthError("no LogoutRequest element", "slo_invalid");
  const nameId = select1(req, "NameID", SAML_NS);
  const sessionIndex = select1(req, "SessionIndex", SAMLP_NS);
  return {
    id: req.getAttribute("ID") ?? "",
    issuer: localText(select1(req, "Issuer", SAML_NS)) || undefined,
    nameId: nameId ? localText(nameId) : undefined,
    sessionIndex: sessionIndex ? localText(sessionIndex) : undefined,
  };
}

function parseLogoutResponseXml(xml: string): {
  id: string;
  issuer?: string | undefined;
  inResponseTo?: string | undefined;
  status: string;
} {
  const doc = parseXml(xml);
  const res = select1(doc, "LogoutResponse", SAMLP_NS) as Element | null;
  if (!res) throw new RealmeAuthError("no LogoutResponse element", "slo_invalid");
  const status = select1(res, "StatusCode", SAMLP_NS) as Element | null;
  return {
    id: res.getAttribute("ID") ?? "",
    issuer: localText(select1(res, "Issuer", SAML_NS)) || undefined,
    inResponseTo: res.getAttribute("InResponseTo") ?? undefined,
    status: status?.getAttribute("Value") ?? "unknown",
  };
}

/** Wrap an SLO message in a parse-and-verify step that turns every failure into RealmeAuthError. */
function safeParse<T>(fn: () => T): T {
  try {
    return fn();
  } catch (err) {
    if (err instanceof RealmeAuthError) throw err;
    throw new RealmeAuthError((err as Error).message, "slo_invalid");
  }
}

/** Verify and parse an IdP LogoutRequest (front-channel, IdP-initiated logout). */
export function processLogoutRequest(rawQuery: string): LogoutRequestMessage {
  const cfg = getRealmeConfig();
  const sig = verifyRedirectSignature(rawQuery);
  if (!sig.ok) throw new RealmeAuthError(sig.reason ?? "invalid signature", "slo_signature");
  const pairs = rawQueryPairs(rawQuery);
  const samlRequest = rawParam(pairs, "SAMLRequest");
  if (samlRequest === undefined) throw new RealmeAuthError("no SAMLRequest", "slo_invalid");
  const msg = safeParse(() => parseLogoutRequestXml(decodeRedirectMessage(samlRequest)));
  if (!cfg.idpEntityId) throw new RealmeAuthError("REALME_IDP_ENTITY_ID is not configured", "slo_invalid");
  if (msg.issuer !== cfg.idpEntityId) {
    throw new RealmeAuthError(`LogoutRequest issuer mismatch: got ${msg.issuer}`, "slo_invalid");
  }
  if (!ASSERTION_ID_PATTERN.test(msg.id)) {
    throw new RealmeAuthError("LogoutRequest ID has an unexpected format", "slo_invalid");
  }
  return msg;
}

/** Verify an IdP LogoutResponse and check it answers our own LogoutRequest. */
export function processLogoutResponse(rawQuery: string, expectedRequestId: string): void {
  const cfg = getRealmeConfig();
  const sig = verifyRedirectSignature(rawQuery);
  if (!sig.ok) throw new RealmeAuthError(sig.reason ?? "invalid signature", "slo_signature");
  const pairs = rawQueryPairs(rawQuery);
  const samlResponse = rawParam(pairs, "SAMLResponse");
  if (samlResponse === undefined) throw new RealmeAuthError("no SAMLResponse", "slo_invalid");
  const msg = safeParse(() => parseLogoutResponseXml(decodeRedirectMessage(samlResponse)));
  if (!cfg.idpEntityId || msg.issuer !== cfg.idpEntityId) {
    throw new RealmeAuthError("LogoutResponse issuer mismatch", "slo_invalid");
  }
  if (!expectedRequestId || msg.inResponseTo !== expectedRequestId) {
    throw new RealmeAuthError("LogoutResponse InResponseTo does not match our request", "slo_invalid");
  }
  if (msg.status !== SUCCESS) {
    throw new RealmeAuthError(`IdP logout status: ${msg.status}`, "slo_status");
  }
}

function requireSloUrl(): string {
  const cfg = getRealmeConfig();
  if (!cfg.sloUrl) throw new Error("REALME_SLO_URL is not configured");
  return cfg.sloUrl;
}

/** Build a signed LogoutRequest for the IdP. The caller stores `requestId` in a cookie. */
export function buildLogoutRequest(session: {
  nameId: string;
  sessionIndex?: string | undefined;
}): { redirectUrl: string; requestId: string } {
  const cfg = getRealmeConfig();
  const sloUrl = requireSloUrl();
  const id = `_${crypto.randomBytes(16).toString("hex")}`;
  const sessionIndexXml = session.sessionIndex
    ? `\n  <samlp:SessionIndex>${escapeXml(session.sessionIndex)}</samlp:SessionIndex>`
    : "";
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<samlp:LogoutRequest xmlns:samlp="${SAMLP_NS}" xmlns:saml="${SAML_NS}" ID="${id}" Version="2.0" IssueInstant="${new Date().toISOString()}" Destination="${sloUrl}">
  <saml:Issuer>${escapeXml(cfg.spEntityId)}</saml:Issuer>
  <saml:NameID Format="${escapeXml(cfg.nameIdFormat)}">${escapeXml(session.nameId)}</saml:NameID>${sessionIndexXml}
</samlp:LogoutRequest>`;
  const url = new URL(sloUrl);
  for (const [k, v] of redirectBindingParams("SAMLRequest", xml)) url.searchParams.set(k, v);
  return { redirectUrl: url.toString(), requestId: id };
}

/** Build a signed LogoutResponse (Success) answering the IdP's LogoutRequest. */
export function buildLogoutResponse(inResponseTo: string): string {
  const cfg = getRealmeConfig();
  const sloUrl = requireSloUrl();
  const id = `_${crypto.randomBytes(16).toString("hex")}`;
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<samlp:LogoutResponse xmlns:samlp="${SAMLP_NS}" xmlns:saml="${SAML_NS}" ID="${id}" Version="2.0" IssueInstant="${new Date().toISOString()}" Destination="${sloUrl}" InResponseTo="${escapeXml(inResponseTo)}">
  <saml:Issuer>${escapeXml(cfg.spEntityId)}</saml:Issuer>
  <samlp:Status><samlp:StatusCode Value="${SUCCESS}"/></samlp:Status>
</samlp:LogoutResponse>`;
  const url = new URL(sloUrl);
  for (const [k, v] of redirectBindingParams("SAMLResponse", xml)) url.searchParams.set(k, v);
  return url.toString();
}
