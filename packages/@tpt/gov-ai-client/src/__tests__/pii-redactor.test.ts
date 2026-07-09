import { describe, it, expect } from "vitest";
import { PiiRedactor } from "../pii-redactor.js";

describe("PiiRedactor", () => {
  const redactor = new PiiRedactor();

  describe("NHI numbers", () => {
    it("redacts 3-letter + 4-digit NHI", () => {
      expect(redactor.redact("NHI is ABC1234")).toBe("NHI is [NHI-REDACTED]");
    });

    it("redacts multiple NHIs", () => {
      expect(redactor.redact("ABC1234 and DEF5678")).toBe("[NHI-REDACTED] and [NHI-REDACTED]");
    });

    it("does not redact 2-letter + 4-digit (not NHI format)", () => {
      expect(redactor.redact("AB1234")).toBe("AB1234");
    });
  });

  describe("IRD numbers", () => {
    it("redacts IRD with dashes (XXX-XXX-XXX)", () => {
      expect(redactor.redact("IRD: 123-456-789")).toBe("IRD: [IRD-REDACTED]");
    });

    it("redacts IRD without dashes (XXXXXXXXX)", () => {
      expect(redactor.redact("IRD: 123456789")).toBe("IRD: [IRD-REDACTED]");
    });

    it("redacts 2-digit prefix IRD", () => {
      expect(redactor.redact("12-345-678")).toBe("[IRD-REDACTED]");
    });
  });

  describe("Passport numbers", () => {
    it("redacts 2-letter + 6-digit passport", () => {
      expect(redactor.redact("Passport: AB123456")).toBe("Passport: [PASSPORT-REDACTED]");
    });

    it("does not redact 3-letter + 6-digit", () => {
      expect(redactor.redact("ABC123456")).toBe("ABC123456");
    });
  });

  describe("NZ phone numbers", () => {
    it("redacts 02X mobile numbers", () => {
      expect(redactor.redact("Phone: 0211234567")).toBe("Phone: [PHONE-REDACTED]");
    });

    it("redacts 021 mobile with surrounding text", () => {
      expect(redactor.redact("Call me on 0215551234 today")).toBe("Call me on [PHONE-REDACTED] today");
    });

    it("does not match +64 format (known regex limitation)", () => {
      // The \b word boundary can't match before '+'. This is a known limitation
      // of the current regex. +64 numbers without surrounding text won't be caught.
      expect(redactor.redact("+64 21 555 1234")).toBe("+64 21 555 1234");
    });
  });

  describe("Mixed PII", () => {
    it("redacts all PII types in one string", () => {
      const input = "Citizen ABC1234, IRD 123-456-789, passport AB123456, phone 0211234567";
      const result = redactor.redact(input);
      expect(result).toBe(
        "Citizen [NHI-REDACTED], IRD [IRD-REDACTED], passport [PASSPORT-REDACTED], phone [PHONE-REDACTED]"
      );
    });

    it("preserves non-PII text", () => {
      expect(redactor.redact("Hello world, no PII here")).toBe("Hello world, no PII here");
    });
  });
});
