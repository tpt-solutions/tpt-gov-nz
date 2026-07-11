export const PORTAL_CONFIG = {
  demoMode: process.env.NEXT_PUBLIC_DEMO_MODE === "true",

  identityServerUrl:
    process.env.NEXT_PUBLIC_IDENTITY_SERVER_URL ?? "http://localhost:8081",

  services: {
    ird: process.env.IRD_SERVICE_URL ?? "http://localhost:8090",
    winz: process.env.WINZ_SERVICE_URL ?? "http://localhost:8091",
    moh: process.env.MOH_SERVICE_URL ?? "http://localhost:8092",
    dia: process.env.DIA_SERVICE_URL ?? "http://localhost:8093",
  },

  sessionCookieName: "tpt_session",
  demoCookieName: "tpt_demo",
  localeCookieName: "tpt_locale",

  sessionTtlSeconds: 60 * 60 * 8,

  get sessionSecret(): string {
    return process.env.TPT__GOV__SESSION_SECRET ?? "dev-insecure-session-secret-change-me";
  },
} as const;

export const DEMO_DID = "did:gov:nz:demo-alex-tane";

export type DeptId = "ird" | "winz" | "moh" | "dia";

export interface DeptMeta {
  id: DeptId;
  name: string;
  shortName: string;
  description: string;
  scopes: string[];
  href: string;
}

export const DEPARTMENTS: DeptMeta[] = [
  {
    id: "ird",
    name: "Inland Revenue",
    shortName: "IRD",
    description: "Income tax, GST, KiwiSaver and Working for Families.",
    scopes: ["ird:income", "ird:tax-summary", "ird:gst", "ird:kiwisaver", "ird:wff"],
    href: "/dept/ird",
  },
  {
    id: "winz",
    name: "Work and Income",
    shortName: "WINZ",
    description: "Benefits, payments and appointments.",
    scopes: ["winz:benefit-status", "winz:payments"],
    href: "/dept/winz",
  },
  {
    id: "moh",
    name: "Ministry of Health",
    shortName: "MOH",
    description: "GP enrolment, prescriptions, appointments and vaccinations.",
    scopes: ["moh:nhi", "moh:prescriptions", "moh:appointments"],
    href: "/dept/moh",
  },
  {
    id: "dia",
    name: "Department of Internal Affairs",
    shortName: "DIA",
    description: "Passports, citizenship and official documents.",
    scopes: ["dia:passport", "dia:birth-certificate"],
    href: "/dept/dia",
  },
];
