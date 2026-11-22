import { describe, it, expect } from "vitest";
import { translate, messages, LOCALES, type Locale } from "../index.js";

describe("translate", () => {
  it("returns the English string for the en locale", () => {
    expect(translate("en", "appName")).toBe("My Gov NZ");
  });

  it("returns the Māori string for the mi locale", () => {
    expect(translate("mi", "appName")).toBe("Tōku Kawanatanga Aotearoa");
  });

  it("returns the key when the key is unknown", () => {
    expect(translate("en", "doesNotExist")).toBe("doesNotExist");
    expect(translate("mi", "doesNotExist")).toBe("doesNotExist");
  });

  it("leaves the string unchanged when vars are supplied but no placeholders exist", () => {
    expect(translate("en", "appName", { name: "Alex" })).toBe("My Gov NZ");
  });
});

describe("messages", () => {
  it("defines an entry for every declared locale", () => {
    for (const { id } of LOCALES) {
      expect(messages[id as Locale]).toBeDefined();
    }
  });

  it("uses the same key set in en and mi", () => {
    expect(Object.keys(messages.mi).sort()).toEqual(Object.keys(messages.en).sort());
  });

  it("ships a non-empty English and Māori dictionary", () => {
    expect(Object.keys(messages.en).length).toBeGreaterThan(0);
    expect(Object.keys(messages.mi).length).toBeGreaterThan(0);
  });
});