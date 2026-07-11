export type Locale = "en" | "mi";

export const LOCALES: { id: Locale; label: string }[] = [
  { id: "en", label: "English" },
  { id: "mi", label: "Te reo Māori" },
];

type Dict = Record<string, string>;

/**
 * UI strings. `en` is the default; `mi` is the opt-in te reo Māori translation.
 * Keys support `{placeholder}` interpolation via `t(key, vars)`.
 */
export const messages: Record<Locale, Dict> = {
  en: {
    appName: "My Gov NZ",
    tagline: "Your unified New Zealand government services portal",
    skipToContent: "Skip to main content",
    home: "Home",
    dashboard: "Dashboard",
    audit: "Audit trail",
    consent: "Consent",
    login: "Sign in",
    logout: "Sign out",
    language: "Language",
    demoBanner:
      "Demo mode — exploring with fictional citizen Alex Tane. No live government systems are contacted.",
    resetDemo: "Reset demo",
    demoScenario: "Scenario",
    aiAssistant: "AI assistant",
    footerNote: "An open-source reference implementation. Not affiliated with any government agency.",
    notAuthenticated: "You are not signed in.",
    signInPrompt: "Sign in to view your dashboard.",
    consentTitle: "Manage data-sharing consent",
    consentIntro:
      "Grant a government department permission to access data you hold with another department. Every grant is signed and recorded in your audit trail.",
    grant: "Grant",
    revoke: "Revoke",
    activeGrants: "Active consents",
    noGrants: "No consents granted yet.",
    auditTitle: "Your audit trail",
    auditIntro: "A record of consents you have granted and your signed-in sessions.",
    tourStart: "Take the tour",
    tourSkip: "Skip tour",
  },
  mi: {
    appName: "Tōku Kawanatanga Aotearoa",
    tagline: "Tō tātau tūmmau ratonga kāwanatanga o Aotearoa",
    skipToContent: "Whakapeka ki ngā kōrero matua",
    home: "Kāinga",
    dashboard: "Papapa whakaaturanga",
    audit: "Kōnae aroturuki",
    consent: "Whakaaetanga",
    login: "Takiuru",
    logout: "Putua te takiuru",
    language: "Reo",
    demoBanner:
      "Aratau whakaaturanga — e tūhura ana mā Alex Tane, he tangata pūtea. Kāhore he pūnaha kāwanatanga ora e whakapāngia ana.",
    resetDemo: "Tārua anō te whakaaturanga",
    demoScenario: "Tauākī āhuatanga",
    aiAssistant: "Kaitohutohu AI",
    footerNote: "He whakatinanatanga tohū tūwhera-puna. Kāhore he hononga ki tētahi umanga kāwanatanga.",
    notAuthenticated: "Kāhore koe kua takiuru.",
    signInPrompt: "Takiuru kia kite i tō papapa whakaaturanga.",
    consentTitle: "Whakahaere i te whakaaetanga tukunga raraunga",
    consentIntro:
      "Tukua tētahi umanga kāwanatanga kia uru ki ngā raraunga kei tētahi atu umanga. Ka hainatia, ka tuhia ngā whakaaetanga katoa ki tō kōnae aroturuki.",
    grant: "Tukua",
    revoke: "Whakakore",
    activeGrants: "Ngā whakaaetanga kaha",
    noGrants: "Kāhore anō kia tukua he whakaaetanga.",
    auditTitle: "Tō kōnae aroturuki",
    auditIntro: "He kōnae o ngā whakaaetanga i tukua e koe me ō wā takiuru kua hainatia.",
    tourStart: "Tīkina te haerenga",
    tourSkip: "Whakarere i te haerenga",
  },
};

export function translate(locale: Locale, key: string, vars?: Record<string, string | number>): string {
  let value = messages[locale]?.[key] ?? messages.en[key] ?? key;
  if (vars) {
    for (const [k, v] of Object.entries(vars)) {
      value = value.replace(new RegExp(`\\{${k}\\}`, "g"), String(v));
    }
  }
  return value;
}
