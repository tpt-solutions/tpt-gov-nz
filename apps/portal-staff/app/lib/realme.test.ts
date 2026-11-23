import crypto from "node:crypto";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { deflateRawSync, inflateRawSync } from "node:zlib";
import { describe, it, expect, beforeAll, afterAll, vi } from "vitest";
import { SignedXml } from "xml-crypto";
import {
  buildAuthnRequest,
  buildLogoutRequest,
  buildLogoutResponse,
  generateSpMetadata,
  getRealmeConfig,
  safeRelayState,
  processSamlResponse,
  processLogoutRequest,
  processLogoutResponse,
  isRealmeStaffAllowed,
  RealmeAuthError,
  createRealmeState,
  verifyRealmeState,
  type SamlSubject,
} from "./realme";

const SP_ENTITY = "https://policy.example.govt.nz/realme";
const IDP_ENTITY = "https://www.realme.govt.nz";
const ACS = "https://policy.example.govt.nz/login/realme/callback";

/** Generate an RSA key pair (PKCS#8 private / SPKI public) for signing + verification. */
function rsaKeyPair(): { key: string; cert: string } {
  const { privateKey, publicKey } = crypto.generateKeyPairSync("rsa", {
    modulusLength: 2048,
  });
  const key = privateKey.export({ type: "pkcs8", format: "pem" }) as string;
  const cert = publicKey.export({ type: "spki", format: "pem" }) as string;
  return { key, cert };
}

/** A syntactically valid (but not real) cert PEM, used only to exercise metadata generation. */
const DUMMY_CERT = "-----BEGIN CERTIFICATE-----\nMIIBFAKEexamplecert\n-----END CERTIFICATE-----";

function uid(): string {
  return `_${crypto.randomBytes(12).toString("hex")}`;
}

function buildResponse(opts: {
  requestId: string;
  assertionId?: string;
  notBefore?: Date;
  notOnOrAfter?: Date;
  audience?: string;
  status?: string;
  nameId?: string;
}): string {
  const aid = opts.assertionId ?? uid();
  const nb = (opts.notBefore ?? new Date(Date.now() - 60_000)).toISOString();
  const noa = (opts.notOnOrAfter ?? new Date(Date.now() + 600_000)).toISOString();
  const aud = opts.audience ?? SP_ENTITY;
  const status = opts.status ?? "urn:oasis:names:tc:SAML:2.0:status:Success";
  const nameId = opts.nameId ?? "case.worker@realme";
  return `<?xml version="1.0" encoding="UTF-8"?>
<samlp:Response xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol" xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion" ID="${uid()}" Version="2.0" IssueInstant="${new Date().toISOString()}" Destination="${ACS}" InResponseTo="${opts.requestId}">
  <saml:Issuer>${SP_ENTITY}</saml:Issuer>
  <samlp:Status><samlp:StatusCode Value="${status}"/></samlp:Status>
  <saml:Assertion ID="${aid}" Version="2.0" IssueInstant="${new Date().toISOString()}">
    <saml:Issuer>${IDP_ENTITY}</saml:Issuer>
    <saml:Subject>
      <saml:NameID Format="urn:oasis:names:tc:SAML:2.0:nameid-format:persistent">${nameId}</saml:NameID>
      <saml:SubjectConfirmation Method="urn:oasis:names:tc:SAML:2.0:cm:bearer">
        <saml:SubjectConfirmationData NotOnOrAfter="${noa}" Recipient="${ACS}" InResponseTo="${opts.requestId}"/>
      </saml:SubjectConfirmation>
    </saml:Subject>
    <saml:Conditions NotBefore="${nb}" NotOnOrAfter="${noa}">
      <saml:AudienceRestriction><saml:Audience>${aud}</saml:Audience></saml:AudienceRestriction>
    </saml:Conditions>
    <saml:AuthnStatement AuthnInstant="${new Date().toISOString()}" SessionIndex="session-1"/>
    <saml:AttributeStatement>
      <saml:Attribute Name="urn:oid:2.5.4.42"><saml:AttributeValue>Case</saml:AttributeValue></saml:Attribute>
      <saml:Attribute Name="urn:oid:0.9.2342.19200300.100.1.3"><saml:AttributeValue>case@example.govt.nz</saml:AttributeValue></saml:Attribute>
    </saml:AttributeStatement>
  </saml:Assertion>
</samlp:Response>`;
}

function signAssertion(xml: string, idpKey: string, idpCert: string): string {
  const sig = new SignedXml({ privateKey: idpKey, publicCert: idpCert });
  sig.addReference({
    xpath: "//*[local-name()='Assertion']",
    transforms: [
      "http://www.w3.org/2000/09/xmldsig#enveloped-signature",
      "http://www.w3.org/2001/10/xml-exc-c14n#",
    ],
    digestAlgorithm: "http://www.w3.org/2001/04/xmlenc#sha256",
  });
  sig.signatureAlgorithm = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";
  sig.canonicalizationAlgorithm = "http://www.w3.org/2001/10/xml-exc-c14n#";
  sig.computeSignature(xml, {
    location: { reference: "//*[local-name()='Assertion']", action: "append" },
  });
  return sig.getSignedXml();
}

function b64(xml: string): string {
  return Buffer.from(xml, "utf8").toString("base64");
}

let idp: { key: string; cert: string };
let sp: { key: string; cert: string };

const SIG_ALG = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";
const SLO_URL = "https://www.realme.govt.nz/logout";
const MAIL_OID = "urn:oid:0.9.2342.19200300.100.1.3";
let replayDir: string;

beforeAll(() => {
  idp = rsaKeyPair();
  sp = rsaKeyPair();
  replayDir = mkdtempSync(join(tmpdir(), "realme-replay-"));
  process.env.REALME_IDP_CERT = idp.cert;
  process.env.REALME_SP_ENTITY_ID = SP_ENTITY;
  process.env.REALME_AUDIENCE = SP_ENTITY;
  process.env.REALME_ACS_URL = ACS;
  process.env.REALME_IDP_ENTITY_ID = IDP_ENTITY;
  process.env.TPT__GOV__SESSION_SECRET = "test-session-secret";
  process.env.REALME_SKIP_VERIFY = "";
  process.env.REALME_REPLAY_STORE = join(replayDir, "replay.log");
});

afterAll(() => {
  delete process.env.REALME_SLO_URL;
  delete process.env.REALME_SP_SIGNING_KEY;
});

/** Build a Redirect-binding message signed by `key`, the way an IdP would send it. */
function signedRedirectQuery(
  kind: "SAMLRequest" | "SAMLResponse",
  xml: string,
  key: string,
): string {
  const parts = [
    `${kind}=${encodeURIComponent(deflateRawSync(Buffer.from(xml, "utf8")).toString("base64"))}`,
    `SigAlg=${encodeURIComponent(SIG_ALG)}`,
  ];
  const signed = parts.join("&");
  const sig = crypto.createSign("RSA-SHA256").update(signed).sign(key, "base64");
  return `${signed}&Signature=${encodeURIComponent(sig)}`;
}

function idpLogoutRequestXml(opts: { id?: string; issuer?: string; nameId?: string }): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
<samlp:LogoutRequest xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol" xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion" ID="${opts.id ?? "_idp-req-1"}" Version="2.0" IssueInstant="${new Date().toISOString()}" Destination="${ACS.replace("/callback", "/slo")}">
  <saml:Issuer>${opts.issuer ?? IDP_ENTITY}</saml:Issuer>
  <saml:NameID Format="urn:oasis:names:tc:SAML:2.0:nameid-format:persistent">${opts.nameId ?? "case.worker@realme"}</saml:NameID>
  <samlp:SessionIndex>session-1</samlp:SessionIndex>
</samlp:LogoutRequest>`;
}

function idpLogoutResponseXml(opts: { inResponseTo: string; status?: string }): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
<samlp:LogoutResponse xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol" xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion" ID="_idp-res-1" Version="2.0" IssueInstant="${new Date().toISOString()}" InResponseTo="${opts.inResponseTo}">
  <saml:Issuer>${IDP_ENTITY}</saml:Issuer>
  <samlp:Status><samlp:StatusCode Value="${opts.status ?? "urn:oasis:names:tc:SAML:2.0:status:Success"}"/></samlp:Status>
</samlp:LogoutResponse>`;
}

/**
 * Verify a Redirect-binding query the way the spec describes: the signed string starts at the
 * SAML message parameter and covers only the raw encoded values, so any other parameter in the
 * SSO URL (such as RealMe's `execution`) is outside the signature.
 */
function verifiesWithCert(query: string, cert: string): boolean {
  const parts = query.split("&");
  const sigIndex = parts.findIndex((p) => p.startsWith("Signature="));
  const start = parts.findIndex((p) => p.startsWith("SAMLRequest=") || p.startsWith("SAMLResponse="));
  const signed = parts.slice(start, sigIndex).join("&");
  const sig = decodeURIComponent(parts[sigIndex]!.slice("Signature=".length));
  return crypto
    .createVerify("RSA-SHA256")
    .update(signed)
    .verify(cert, Buffer.from(sig, "base64"));
}

describe("RealMe AuthnRequest", () => {
  it("builds a deflated, signed Redirect-binding request", () => {
    process.env.REALME_SP_SIGNING_KEY = sp.key;
    const { redirectUrl, requestId } = buildAuthnRequest("");
    expect(requestId).toMatch(/^_/);
    const url = new URL(redirectUrl);
    const saml = url.searchParams.get("SAMLRequest");
    expect(saml).toBeTruthy();
    expect(url.searchParams.get("SigAlg")).toContain("rsa-sha256");
    expect(url.searchParams.get("Signature")).toBeTruthy();

    const decoded = inflateRawSync(Buffer.from(saml!, "base64")).toString("utf8");
    expect(decoded).toContain(`ID="${requestId}"`);
    delete process.env.REALME_SP_SIGNING_KEY;
  });

  it("builds an unsigned request when no SP key is configured", () => {
    delete process.env.REALME_SP_SIGNING_KEY;
    const { redirectUrl } = buildAuthnRequest("");
    const url = new URL(redirectUrl);
    expect(url.searchParams.get("Signature")).toBeNull();
  });
});

describe("RealMe SP metadata", () => {
  it("includes the entityID and signing certificate", () => {
    process.env.REALME_SP_SIGNING_CERT = DUMMY_CERT;
    const md = generateSpMetadata();
    expect(md).toContain(`entityID="${SP_ENTITY}"`);
    expect(md).toContain('AuthnRequestsSigned="true"');
    expect(md).toContain("AssertionConsumerService");
    const certB64 = DUMMY_CERT.replace(
      /-----BEGIN CERTIFICATE-----|-----END CERTIFICATE-----|\s+/g,
      "",
    ).trim();
    expect(md).toContain(certB64);
    delete process.env.REALME_SP_SIGNING_CERT;
  });
});

describe("RealMe response processing", () => {
  it("verifies, parses and validates a correctly signed response", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const res = processSamlResponse(b64(xml), { requestId, acsUrl: ACS });
    expect(res.subject.nameId).toBe("case.worker@realme");
    expect(res.subject.attributes["urn:oid:2.5.4.42"]).toEqual(["Case"]);
    expect(res.inResponseTo).toBe(requestId);
  });

  it("rejects an expired assertion (NotOnOrAfter in the past)", () => {
    const requestId = uid();
    const xml = signAssertion(
      buildResponse({
        requestId,
        notBefore: new Date(Date.now() - 1_200_000),
        notOnOrAfter: new Date(Date.now() - 600_000),
      }),
      idp.key,
      idp.cert,
    );
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      RealmeAuthError,
    );
  });

  it("rejects an audience restriction mismatch", () => {
    const requestId = uid();
    const xml = signAssertion(
      buildResponse({ requestId, audience: "https://evil.example/" }),
      idp.key,
      idp.cert,
    );
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /audience/i,
    );
  });

  it("rejects an InResponseTo mismatch", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    expect(() =>
      processSamlResponse(b64(xml), { requestId: "wrong-request", acsUrl: ACS }),
    ).toThrow(/InResponseTo/);
  });

  it("rejects a non-Success status", () => {
    const requestId = uid();
    const xml = signAssertion(
      buildResponse({
        requestId,
        status: "urn:oasis:names:tc:SAML:2.0:status:AuthnFailed",
      }),
      idp.key,
      idp.cert,
    );
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /status/i,
    );
  });

  it("rejects a tampered response (signature failure)", () => {
    const requestId = uid();
    let xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    xml = xml.replace("case.worker@realme", "attacker@realme");
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      RealmeAuthError,
    );
  });

  it("rejects replay of the same assertion ID", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    processSamlResponse(b64(xml), { requestId, acsUrl: ACS });
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /already used/,
    );
  });

  it("fails verification when the IdP cert is wrong (key confusion)", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const other = rsaKeyPair();
    process.env.REALME_IDP_CERT = other.cert;
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      RealmeAuthError,
    );
    process.env.REALME_IDP_CERT = idp.cert;
  });
});

describe("RealMe subject confirmation", () => {
  it("reads SessionIndex from AuthnStatement", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const res = processSamlResponse(b64(xml), { requestId, acsUrl: ACS });
    expect(res.subject.sessionIndex).toBe("session-1");
  });

  it("rejects a SubjectConfirmation Recipient for another ACS", () => {
    const requestId = uid();
    const unsigned = buildResponse({ requestId }).replace(
      `Recipient="${ACS}"`,
      'Recipient="https://evil.example/callback"',
    );
    const xml = signAssertion(unsigned, idp.key, idp.cert);
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(/Recipient/);
  });

  it("rejects a SubjectConfirmation InResponseTo that does not match the request", () => {
    const requestId = uid();
    const unsigned = buildResponse({ requestId }).replace(
      `InResponseTo="${requestId}"/>`,
      'InResponseTo="_other"/>',
    );
    const xml = signAssertion(unsigned, idp.key, idp.cert);
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /SubjectConfirmation InResponseTo/,
    );
  });

  it("rejects a non-bearer subject confirmation method", () => {
    const requestId = uid();
    const unsigned = buildResponse({ requestId }).replace(
      "cm:bearer",
      "cm:holder-of-key",
    );
    const xml = signAssertion(unsigned, idp.key, idp.cert);
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(/bearer/);
  });

  it("rejects a SubjectConfirmationData without NotOnOrAfter", () => {
    const requestId = uid();
    const unsigned = buildResponse({ requestId }).replace(
      /(<saml:SubjectConfirmationData NotOnOrAfter="[^"]*")/,
      "<saml:SubjectConfirmationData",
    );
    const xml = signAssertion(unsigned, idp.key, idp.cert);
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /NotOnOrAfter/,
    );
  });
});

describe("RealMe correlation state", () => {
  it("round-trips the signed request state", () => {
    const state = createRealmeState("req-123", 600);
    const v = verifyRealmeState(state);
    expect(v.ok).toBe(true);
    expect(v.requestId).toBe("req-123");
    expect(verifyRealmeState(undefined).ok).toBe(false);
  });
});

describe("RealMe fail-closed behaviour", () => {
  it("refuses to accept a response when no IdP certificate is pinned", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const saved = process.env.REALME_IDP_CERT;
    delete process.env.REALME_IDP_CERT;
    try {
      expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
        /REALME_IDP_CERT/,
      );
    } finally {
      process.env.REALME_IDP_CERT = saved;
    }
  });

  it("refuses REALME_SKIP_VERIFY in production", () => {
    const savedSkip = process.env.REALME_SKIP_VERIFY;
    const savedEnv = process.env.NODE_ENV;
    process.env.REALME_SKIP_VERIFY = "true";
    (process.env as Record<string, string>).NODE_ENV = "production";
    try {
      expect(() => getRealmeConfig()).toThrow(/not allowed in production/);
    } finally {
      process.env.REALME_SKIP_VERIFY = savedSkip;
      (process.env as Record<string, string>).NODE_ENV = savedEnv;
    }
  });

  it("allows REALME_SKIP_VERIFY outside production (development only)", () => {
    const savedSkip = process.env.REALME_SKIP_VERIFY;
    process.env.REALME_SKIP_VERIFY = "true";
    try {
      expect(getRealmeConfig().skipVerify).toBe(true);
    } finally {
      process.env.REALME_SKIP_VERIFY = savedSkip;
    }
  });

  it("refuses to validate when the IdP entity ID is not configured", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const saved = process.env.REALME_IDP_ENTITY_ID;
    delete process.env.REALME_IDP_ENTITY_ID;
    try {
      expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
        /REALME_IDP_ENTITY_ID/,
      );
    } finally {
      process.env.REALME_IDP_ENTITY_ID = saved;
    }
  });
});

describe("RealMe hardening: relayState and DOCTYPE", () => {
  it("keeps same-origin relative paths", () => {
    expect(safeRelayState("/citizens")).toBe("/citizens");
    expect(safeRelayState("/dept/ird?x=1")).toBe("/dept/ird?x=1");
  });

  it("drops absolute, protocol-relative and backslash targets", () => {
    expect(safeRelayState("https://evil.example/")).toBe("");
    expect(safeRelayState("//evil.example/")).toBe("");
    expect(safeRelayState("/\\evil.example")).toBe("");
    expect(safeRelayState("javascript:alert(1)")).toBe("");
    expect(safeRelayState("/with space")).toBe("");
    expect(safeRelayState(null)).toBe("");
  });

  it("rejects SAML responses that declare a DOCTYPE", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    const withDoctype = xml.replace(
      "<samlp:Response",
      '<!DOCTYPE x [<!ENTITY e "boom">]><samlp:Response',
    );
    expect(() => processSamlResponse(b64(withDoctype), { requestId, acsUrl: ACS })).toThrow(
      /DOCTYPE/,
    );
  });
});

describe("RealMe staff allow-list", () => {
  const subject = (nameId: string, mail?: string): SamlSubject => ({
    nameId,
    attributes: mail ? { [MAIL_OID]: [mail] } : {},
  });

  it("denies everyone when the allow-list is empty", () => {
    expect(isRealmeStaffAllowed(subject("case.worker@realme"), [])).toBe(false);
  });

  it("matches the NameID case-insensitively", () => {
    expect(isRealmeStaffAllowed(subject("Case.Worker@RealMe"), ["case.worker@realme"])).toBe(true);
  });

  it("matches the mail attribute", () => {
    expect(
      isRealmeStaffAllowed(subject("opaque-id", "Case@Example.govt.nz"), ["case@example.govt.nz"]),
    ).toBe(true);
  });

  it("denies a valid identity that is not on the list", () => {
    expect(
      isRealmeStaffAllowed(subject("someone.else@realme", "else@example.govt.nz"), [
        "case.worker@realme",
      ]),
    ).toBe(false);
  });

  it("reads the allow-list from REALME_STAFF_ALLOWLIST", () => {
    process.env.REALME_STAFF_ALLOWLIST = " Case.Worker@realme, ,other@example.govt.nz ";
    try {
      expect(getRealmeConfig().staffAllowlist).toEqual([
        "case.worker@realme",
        "other@example.govt.nz",
      ]);
    } finally {
      delete process.env.REALME_STAFF_ALLOWLIST;
    }
  });
});

describe("RealMe replay store", () => {
  it("remembers an accepted assertion across a restart", async () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
    processSamlResponse(b64(xml), { requestId, acsUrl: ACS });

    // A fresh module instance has no in-memory state: only the log can remember the assertion.
    vi.resetModules();
    const restarted = await import("./realme");
    expect(() => restarted.processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /already used/,
    );
  });

  it("refuses the assertion when the replay log cannot be written (fail closed)", () => {
    const blocker = join(replayDir, "not-a-directory");
    writeFileSync(blocker, "x");
    const saved = process.env.REALME_REPLAY_STORE;
    process.env.REALME_REPLAY_STORE = join(blocker, "replay.log");
    try {
      const requestId = uid();
      const xml = signAssertion(buildResponse({ requestId }), idp.key, idp.cert);
      expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
        /replay store unavailable/,
      );
    } finally {
      process.env.REALME_REPLAY_STORE = saved;
    }
  });

  it("rejects assertion IDs that could corrupt the log", () => {
    const requestId = uid();
    const xml = signAssertion(buildResponse({ requestId, assertionId: "bad id" }), idp.key, idp.cert);
    expect(() => processSamlResponse(b64(xml), { requestId, acsUrl: ACS })).toThrow(
      /unexpected format/,
    );
  });
});

describe("RealMe signed Redirect-binding messages", () => {
  it("signs AuthnRequests over SAMLRequest, RelayState and SigAlg in that order", () => {
    process.env.REALME_SP_SIGNING_KEY = sp.key;
    try {
      const { redirectUrl } = buildAuthnRequest("/citizens");
      const query = new URL(redirectUrl).search.slice(1);
      expect(verifiesWithCert(query, sp.cert)).toBe(true);
    } finally {
      delete process.env.REALME_SP_SIGNING_KEY;
    }
  });

  it("builds a signed LogoutRequest carrying NameID and SessionIndex", () => {
    process.env.REALME_SP_SIGNING_KEY = sp.key;
    process.env.REALME_SLO_URL = SLO_URL;
    try {
      const { redirectUrl, requestId } = buildLogoutRequest({
        nameId: "a&b@realme",
        sessionIndex: "session-9",
      });
      const url = new URL(redirectUrl);
      expect(url.origin + url.pathname).toBe(SLO_URL);
      expect(verifiesWithCert(url.search.slice(1), sp.cert)).toBe(true);
      const xml = inflateRawSync(
        Buffer.from(url.searchParams.get("SAMLRequest")!, "base64"),
      ).toString("utf8");
      expect(xml).toContain(`ID="${requestId}"`);
      expect(xml).toContain("a&amp;b@realme");
      expect(xml).toContain("<samlp:SessionIndex>session-9</samlp:SessionIndex>");
    } finally {
      delete process.env.REALME_SP_SIGNING_KEY;
      delete process.env.REALME_SLO_URL;
    }
  });

  it("builds a signed LogoutResponse for the IdP", () => {
    process.env.REALME_SP_SIGNING_KEY = sp.key;
    process.env.REALME_SLO_URL = SLO_URL;
    try {
      const url = new URL(buildLogoutResponse("_idp-req-1"));
      expect(verifiesWithCert(url.search.slice(1), sp.cert)).toBe(true);
      const xml = inflateRawSync(
        Buffer.from(url.searchParams.get("SAMLResponse")!, "base64"),
      ).toString("utf8");
      expect(xml).toContain('InResponseTo="_idp-req-1"');
    } finally {
      delete process.env.REALME_SP_SIGNING_KEY;
      delete process.env.REALME_SLO_URL;
    }
  });

  it("refuses to build a LogoutRequest when REALME_SLO_URL is not configured", () => {
    delete process.env.REALME_SLO_URL;
    expect(() => buildLogoutRequest({ nameId: "x" })).toThrow(/REALME_SLO_URL/);
  });
});

describe("RealMe Single Logout: inbound messages from the IdP", () => {
  it("accepts an IdP LogoutRequest signed by the pinned IdP certificate", () => {
    const query = signedRedirectQuery(
      "SAMLRequest",
      idpLogoutRequestXml({ id: "_idp-req-7" }),
      idp.key,
    );
    const msg = processLogoutRequest(`?${query}`);
    expect(msg.id).toBe("_idp-req-7");
    expect(msg.nameId).toBe("case.worker@realme");
    expect(msg.sessionIndex).toBe("session-1");
  });

  it("rejects an unsigned LogoutRequest", () => {
    const samlRequest = encodeURIComponent(
      deflateRawSync(Buffer.from(idpLogoutRequestXml({}), "utf8")).toString("base64"),
    );
    expect(() => processLogoutRequest(`SAMLRequest=${samlRequest}`)).toThrow(RealmeAuthError);
  });

  it("rejects a LogoutRequest signed by a key other than the pinned IdP key", () => {
    const attacker = rsaKeyPair();
    const query = signedRedirectQuery("SAMLRequest", idpLogoutRequestXml({}), attacker.key);
    expect(() => processLogoutRequest(`?${query}`)).toThrow(/does not verify/);
  });

  it("rejects a LogoutRequest whose payload was altered after signing", () => {
    const query = signedRedirectQuery("SAMLRequest", idpLogoutRequestXml({}), idp.key);
    const tampered = query.replace(
      /SAMLRequest=([^&]+)/,
      (_m, value: string) => `SAMLRequest=${value.slice(0, -2)}AA`,
    );
    expect(() => processLogoutRequest(`?${tampered}`)).toThrow(RealmeAuthError);
  });

  it("rejects a LogoutRequest from an issuer other than the IdP", () => {
    const query = signedRedirectQuery(
      "SAMLRequest",
      idpLogoutRequestXml({ issuer: "https://evil.example/idp" }),
      idp.key,
    );
    expect(() => processLogoutRequest(`?${query}`)).toThrow(/issuer/);
  });

  it("accepts a LogoutResponse that answers our request", () => {
    const query = signedRedirectQuery(
      "SAMLResponse",
      idpLogoutResponseXml({ inResponseTo: "_ours" }),
      idp.key,
    );
    expect(() => processLogoutResponse(`?${query}`, "_ours")).not.toThrow();
  });

  it("rejects a LogoutResponse that answers a different request", () => {
    const query = signedRedirectQuery(
      "SAMLResponse",
      idpLogoutResponseXml({ inResponseTo: "_someone-else" }),
      idp.key,
    );
    expect(() => processLogoutResponse(`?${query}`, "_ours")).toThrow(/InResponseTo/);
  });

  it("rejects a LogoutResponse that reports a failed logout", () => {
    const query = signedRedirectQuery(
      "SAMLResponse",
      idpLogoutResponseXml({
        inResponseTo: "_ours",
        status: "urn:oasis:names:tc:SAML:2.0:status:Requester",
      }),
      idp.key,
    );
    expect(() => processLogoutResponse(`?${query}`, "_ours")).toThrow(/status/);
  });

  it("refuses inbound logout messages when no IdP certificate is pinned", () => {
    const query = signedRedirectQuery("SAMLRequest", idpLogoutRequestXml({}), idp.key);
    const saved = process.env.REALME_IDP_CERT;
    delete process.env.REALME_IDP_CERT;
    try {
      expect(() => processLogoutRequest(`?${query}`)).toThrow(/REALME_IDP_CERT/);
    } finally {
      process.env.REALME_IDP_CERT = saved;
    }
  });
});
