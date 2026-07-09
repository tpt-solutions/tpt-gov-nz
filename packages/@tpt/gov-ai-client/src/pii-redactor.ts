const NHI_PATTERN = /\b[A-Z]{3}\d{4}\b/g;
const IRD_PATTERN = /\b\d{2,3}-?\d{3}-?\d{3}\b/g;
const PASSPORT_PATTERN = /\b[A-Z]{2}\d{6}\b/g;
const PHONE_NZ_PATTERN = /\b(0[2-9]\d{7,9}|\+64\s?\d[\s\d]{7,11})\b/g;

export class PiiRedactor {
  redact(text: string): string {
    return text
      .replace(NHI_PATTERN, "[NHI-REDACTED]")
      .replace(IRD_PATTERN, "[IRD-REDACTED]")
      .replace(PASSPORT_PATTERN, "[PASSPORT-REDACTED]")
      .replace(PHONE_NZ_PATTERN, "[PHONE-REDACTED]");
  }
}
