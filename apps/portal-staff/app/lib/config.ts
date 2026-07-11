export const STAFF_CONFIG = {
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
    nzqa: process.env.NZQA_SERVICE_URL ?? "http://localhost:8099",
    msd: process.env.MSD_SERVICE_URL ?? "http://localhost:8100",
    mbie: process.env.MBIE_SERVICE_URL ?? "http://localhost:8101",
    linz: process.env.LINZ_SERVICE_URL ?? "http://localhost:8102",
    statsnz: process.env.STATSNZ_SERVICE_URL ?? "http://localhost:8103",
    corrections: process.env.CORRECTIONS_SERVICE_URL ?? "http://localhost:8104",
    customs: process.env.CUSTOMS_SERVICE_URL ?? "http://localhost:8105",
    mpi: process.env.MPI_SERVICE_URL ?? "http://localhost:8106",
    doc: process.env.DOC_SERVICE_URL ?? "http://localhost:8107",
    tpk: process.env.TPK_SERVICE_URL ?? "http://localhost:8108",
  },

  sessionCookieName: "tpt_staff_session",
  staffIdCookieName: "tpt_staff_id",

  sessionTtlSeconds: 60 * 60 * 8,

  get sessionSecret(): string {
    return process.env.TPT__GOV__SESSION_SECRET ?? "dev-insecure-session-secret-change-me";
  },
} as const;

export type StaffDeptId = "ird" | "winz" | "moh" | "dia" | "nzta" | "acc" | "nzqa" | "msd" | "mbie" | "linz" | "statsnz" | "corrections" | "customs" | "mpi" | "doc" | "tpk";

export interface StaffDeptMeta {
  id: StaffDeptId;
  name: string;
  shortName: string;
  description: string;
  scopes: string[];
  href: string;
}

export const STAFF_DEPARTMENTS: StaffDeptMeta[] = [
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
    scopes: ["nzta:licence", "nzta:vehicles"],
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
    id: "nzqa",
    name: "Ministry of Education / NZQA",
    shortName: "NZQA",
    description: "Qualifications, transcripts and the New Zealand Record of Achievement.",
    scopes: ["nzqa:qualifications", "nzqa:transcripts"],
    href: "/dept/nzqa",
  },
  {
    id: "msd",
    name: "Ministry of Social Development",
    shortName: "MSD",
    description: "StudyLink student loans and allowances, and cross-service case history.",
    scopes: ["msd:studylink", "msd:case-history"],
    href: "/dept/msd",
  },
  {
    id: "mbie",
    name: "Ministry of Business, Innovation and Employment",
    shortName: "MBIE",
    description: "Business registrations and company director lookup.",
    scopes: ["mbie:business", "mbie:directorships"],
    href: "/dept/mbie",
  },
  {
    id: "linz",
    name: "Toitū Te Whenua Land Information New Zealand",
    shortName: "LINZ",
    description: "Property titles and land ownership records.",
    scopes: ["linz:titles", "linz:ownership"],
    href: "/dept/linz",
  },
  {
    id: "statsnz",
    name: "Statistics New Zealand",
    shortName: "Stats NZ",
    description: "Census data, official statistics and your data profile.",
    scopes: ["statsnz:census", "statsnz:profile"],
    href: "/dept/statsnz",
  },
  {
    id: "corrections",
    name: "Department of Corrections",
    shortName: "Corrections",
    description: "Probation status and case information.",
    scopes: ["corrections:probation", "corrections:case"],
    href: "/dept/corrections",
  },
  {
    id: "customs",
    name: "New Zealand Customs Service",
    shortName: "Customs",
    description: "Traveller declarations and arrival information.",
    scopes: ["customs:travel", "customs:declarations"],
    href: "/dept/customs",
  },
  {
    id: "mpi",
    name: "Ministry for Primary Industries",
    shortName: "MPI",
    description: "Food and business registrations and certifications.",
    scopes: ["mpi:registrations", "mpi:certifications"],
    href: "/dept/mpi",
  },
  {
    id: "doc",
    name: "Department of Conservation",
    shortName: "DOC",
    description: "Conservation permits and concessions.",
    scopes: ["doc:permits", "doc:concessions"],
    href: "/dept/doc",
  },
  {
    id: "tpk",
    name: "Te Puni Kōkiri",
    shortName: "TPK",
    description: "Māori development programmes and funding.",
    scopes: ["tpk:programmes", "tpk:funding"],
    href: "/dept/tpk",
  },
];

/** The fictional demo citizen shared with the citizen portal. */
export const DEMO_CITIZEN_DID = "did:gov:nz:demo-alex-tane";
export const DEMO_CITIZEN_NAME = "Alex Tane";
