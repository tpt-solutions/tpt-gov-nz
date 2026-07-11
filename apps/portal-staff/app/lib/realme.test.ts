import crypto from "node:crypto";
import { inflateRawSync, deflateRawSync } from "node:zlib";
import { describe, it, expect, beforeAll } from "vitest";
import { SignedXml } from "xml-crypto";
import {
  buildAuthnRequest,
  generateSpMetadata,
  processSamlResponse,
  RealmeAuthError,
  createRealmeState,
  verifyRealmeState,
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

beforeAll(() => {
  idp = rsaKeyPair();
  sp = rsaKeyPair();
  process.env.REALME_IDP_CERT = idp.cert;
  process.env.REALME_SP_ENTITY_ID = SP_ENTITY;
  process.env.REALME_AUDIENCE = SP_ENTITY;
  process.env.REALME_ACS_URL = ACS;
  process.env.REALME_IDP_ENTITY_ID = IDP_ENTITY;
  process.env.TPT__GOV__SESSION_SECRET = "test-session-secret";
  process.env.REALME_SKIP_VERIFY = "";
});

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

describe("RealMe correlation state", () => {
  it("round-trips the signed request state", () => {
    const state = createRealmeState("req-123", 600);
    const v = verifyRealmeState(state);
    expect(v.ok).toBe(true);
    expect(v.requestId).toBe("req-123");
    expect(verifyRealmeState(undefined).ok).toBe(false);
  });
});
