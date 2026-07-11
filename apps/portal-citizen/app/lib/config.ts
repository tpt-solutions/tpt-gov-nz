export const PORTAL_CONFIG = {
  demoMode: process.env.NEXT_PUBLIC_DEMO_MODE === "true",

  identityServerUrl:
    process.env.NEXT_PUBLIC_IDENTITY_SERVER_URL ?? "http://localhost:8081",

  services: {
    ird: process.env.IRD_SERVICE_URL ?? "http://localhost:8090",
    winz: process.env.WINZ_SERVICE_URL ?? "http://localhost:8091",
    moh: process.env.MOH_SERVICE_URL ?? "http://localhost:8092",
    dia: process.env.DIA_SERVICE_URL ?? "http://localhost:8093",
    nzta: process.env.NZTA_SERVICE_URL ?? "http://localhost:8094",
    acc: process.env.ACC_SERVICE_URL ?? "http://localhost:8095",
    moj: process.env.MOJ_SERVICE_URL ?? "http://localhost:8096",
    police: process.env.POLICE_SERVICE_URL ?? "http://localhost:8097",
    hud: process.env.HUD_SERVICE_URL ?? "http://localhost:8098",
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

export type DeptId = "ird" | "winz" | "moh" | "dia" | "nzta" | "acc" | "moj" | "police" | "hud";

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
  {
    id: "nzta",
    name: "Waka Kotahi NZ Transport Agency",
    shortName: "NZTA",
    description: "Driver licence, vehicles and road user charges.",
    scopes: ["nzta:driver-licence", "nzta:vehicles", "nzta:ruc"],
    href: "/dept/nzta",
  },
  {
    id: "acc",
    name: "Accident Compensation Corporation",
    shortName: "ACC",
    description: "Injury claims, entitlements and rehabilitation.",
    scopes: ["acc:claims", "acc:entitlements", "acc:rehabilitation"],
    href: "/dept/acc",
  },
  {
    id: "moj",
    name: "Ministry of Justice",
    shortName: "MOJ",
    description: "Fines, Disputes Tribunal claims, court records and name changes.",
    scopes: ["moj:fines", "moj:disputes", "moj:court-records"],
    href: "/dept/moj",
  },
  {
    id: "police",
    name: "New Zealand Police",
    shortName: "Police",
    description: "Infringements and reports.",
    scopes: ["police:infringements", "police:reports"],
    href: "/dept/police",
  },
  {
    id: "hud",
    name: "Ministry of Housing and Urban Development / Kāinga Ora",
    shortName: "Housing",
    description: "Housing applications, tenancy and maintenance requests.",
    scopes: ["hud:applications", "hud:tenancy", "hud:maintenance"],
    href: "/dept/hud",
  },
];
