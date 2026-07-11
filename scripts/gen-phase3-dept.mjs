// @ts-nocheck
// Generator for Phase 3 departments in tpt-gov-nz.
//
// Scaffolds a full native department module (Rust dept service + Rust ingester +
// TypeScript schema + citizen/staff portal pages + OPA policy + docker + env +
// registry wiring) following the exact proven pattern of `gov-dept-statsnz`.
//
// Usage:
//   node scripts/gen-phase3-dept.mjs            # generate every listed department
//   node scripts/gen-phase3-dept.mjs treasury   # generate only `treasury`
//
// The generator is deterministic and idempotent: re-running overwrites the
// generated department files. Existing shared files (config.ts, mock-data.ts,
// docker compose, .env.example, schema index) are patched at stable anchors.

import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const W = (rel, content) => {
  const p = join(ROOT, rel);
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(p, content);
};

// ── type mapping ──────────────────────────────────────────────────────────────
const T = {
  string: { rust: "String", pg: "TEXT", zod: "z.string()", json: (f) => `c.${f}` },
  i32: { rust: "i32", pg: "INTEGER", zod: "z.number()", json: (f) => `c.${f}` },
  f64: { rust: "f64", pg: "DOUBLE PRECISION", zod: "z.number()", json: (f) => `c.${f}` },
  bool: { rust: "bool", pg: "BOOLEAN", zod: "z.boolean()", json: (f) => `c.${f}` },
  date: { rust: "chrono::NaiveDate", pg: "DATE", zod: "z.string()", json: (f) => `c.${f}.to_string()` },
};

const pascal = (s) => s.replace(/(^|[-_])(\w)/g, (_, __, c) => c.toUpperCase());
const camel = (s) => s.replace(/[-_ ](\w)/g, (_, c) => c.toUpperCase());
const upper = (s) => s.toUpperCase().replace(/-/g, "_");

function sqlLit(v, type) {
  if (type === "string") return `'${String(v).replace(/'/g, "''")}'`;
  if (type === "bool") return v ? "true" : "false";
  if (type === "date") return `'${String(v)}'`;
  return String(v);
}

// ── department specifications ───────────────────────────────────────────────────
// field: { n: snake_case, j: camelCase json, t: type }
const DEPTS = [
  {
    id: "treasury",
    name: "The Treasury",
    shortName: "Treasury",
    description: "Crown financial management, the Budget and economic forecasts.",
    port: 8120, fedPort: 7040, dbPort: 5460,
    localId: { field: "treasury_id", example: "TRE-100001" },
    aiPrompt: "What Budget and economic data is held about me?",
    entities: [
      {
        key: "budget", scope: "treasury:budget", label: "Budget", cardinality: "many",
        fields: [
          { n: "fiscal_year", j: "fiscalYear", t: "i32" },
          { n: "portfolio", j: "portfolio", t: "string" },
          { n: "appropriation", j: "appropriation", t: "string" },
          { n: "amount", j: "amount", t: "f64" },
        ],
        sample: { fiscalYear: 2026, portfolio: "Health", appropriation: "Vote Health", amount: 1200000000 },
      },
      {
        key: "economic_outlook", scope: "treasury:economic-outlook", label: "Economic outlook", cardinality: "one",
        fields: [
          { n: "forecast_year", j: "forecastYear", t: "i32" },
          { n: "gdp_growth_pct", j: "gdpGrowthPct", t: "f64" },
          { n: "inflation_pct", j: "inflationPct", t: "f64" },
          { n: "net_debt_pct", j: "netDebtPct", t: "f64" },
        ],
        sample: { forecastYear: 2026, gdpGrowthPct: 2.4, inflationPct: 3.1, netDebtPct: 42.0 },
      },
    ],
    actions: [
      { type: "request-economic-brief", params: [{ name: "topic", validate: "nonempty" }], message: "Economic brief request received. We will prepare a brief on" },
    ],
  },
  {
    id: "dpmc",
    name: "Department of the Prime Minister and Cabinet",
    shortName: "DPMC",
    description: "Cabinet support, honours and national security coordination.",
    port: 8121, fedPort: 7041, dbPort: 5461,
    localId: { field: "dpmc_id", example: "DPMC-100001" },
    aiPrompt: "What honours or engagements are recorded for me?",
    entities: [
      {
        key: "honours", scope: "dpmc:honours", label: "Honours", cardinality: "many",
        fields: [
          { n: "award_year", j: "awardYear", t: "i32" },
          { n: "award", j: "award", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { awardYear: 2025, award: "Queen's Service Medal", status: "nominated" },
      },
      {
        key: "engagements", scope: "dpmc:engagements", label: "Engagements", cardinality: "many",
        fields: [
          { n: "event_name", j: "eventName", t: "string" },
          { n: "event_date", j: "eventDate", t: "date" },
          { n: "location", j: "location", t: "string" },
        ],
        sample: { eventName: "Citizens' Honours Reception", eventDate: "2026-05-12", location: "Wellington" },
      },
    ],
    actions: [
      { type: "request-honours-update", params: [{ name: "topic", validate: "nonempty" }], message: "Honours update request received regarding" },
    ],
  },
  {
    id: "publicservice",
    name: "Te Kawa Mataaho Public Service Commission",
    shortName: "Public Service",
    description: "Workforce data and capability ratings across public agencies.",
    port: 8122, fedPort: 7042, dbPort: 5462,
    localId: { field: "psc_id", example: "PSC-100001" },
    aiPrompt: "What public service workforce information is held about me?",
    entities: [
      {
        key: "workforce", scope: "publicservice:workforce", label: "Workforce", cardinality: "many",
        fields: [
          { n: "report_year", j: "reportYear", t: "i32" },
          { n: "agency", j: "agency", t: "string" },
          { n: "headcount", j: "headcount", t: "i32" },
        ],
        sample: { reportYear: 2025, agency: "Department of Internal Affairs", headcount: 4200 },
      },
      {
        key: "agency_ratings", scope: "publicservice:agency-ratings", label: "Agency ratings", cardinality: "many",
        fields: [
          { n: "agency", j: "agency", t: "string" },
          { n: "rating", j: "rating", t: "string" },
          { n: "rating_year", j: "ratingYear", t: "i32" },
        ],
        sample: { agency: "Department of Internal Affairs", rating: "Good", ratingYear: 2025 },
      },
    ],
    actions: [
      { type: "request-workforce-report", params: [{ name: "agency", validate: "nonempty" }], message: "Workforce report request received for" },
    ],
  },
  {
    id: "crownlaw",
    name: "Crown Law Office",
    shortName: "Crown Law",
    description: "Government legal opinions and litigation the Crown is involved in.",
    port: 8123, fedPort: 7043, dbPort: 5463,
    localId: { field: "crownlaw_id", example: "CL-100001" },
    aiPrompt: "What Crown Law matters involve me?",
    entities: [
      {
        key: "legal_opinions", scope: "crownlaw:legal-opinions", label: "Legal opinions", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "topic", j: "topic", t: "string" },
          { n: "issued_date", j: "issuedDate", t: "date" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { reference: "CL-OP-2026-001", topic: "Treaty settlement wording", issuedDate: "2026-02-18", status: "final" },
      },
      {
        key: "litigation", scope: "crownlaw:litigation", label: "Litigation", cardinality: "many",
        fields: [
          { n: "case_name", j: "caseName", t: "string" },
          { n: "crown_role", j: "crownRole", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { caseName: "Re Crown assets", crownRole: "Defendant", status: "ongoing" },
      },
    ],
    actions: [
      { type: "request-opinion-copy", params: [{ name: "reference", validate: "nonempty" }], message: "Opinion copy request received for reference" },
    ],
  },
  {
    id: "sfo",
    name: "Serious Fraud Office",
    shortName: "SFO",
    description: "Investigations into serious or complex fraud.",
    port: 8124, fedPort: 7044, dbPort: 5464,
    localId: { field: "sfo_id", example: "SFO-100001" },
    aiPrompt: "What Serious Fraud Office matters relate to me?",
    entities: [
      {
        key: "investigations", scope: "sfo:investigations", label: "Investigations", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "matter", j: "matter", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "opened_date", j: "openedDate", t: "date" },
        ],
        sample: { reference: "SFO-2026-014", matter: "Complex investment fraud", status: "under-investigation", openedDate: "2026-01-22" },
      },
      {
        key: "outcomes", scope: "sfo:outcomes", label: "Outcomes", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "result", j: "result", t: "string" },
          { n: "result_date", j: "resultDate", t: "date" },
        ],
        sample: { reference: "SFO-2025-009", result: "Prosecution commenced", resultDate: "2025-11-03" },
      },
    ],
    actions: [
      { type: "request-investigation-update", params: [{ name: "reference", validate: "nonempty" }], message: "Investigation update request received for reference" },
    ],
  },
  {
    id: "oranga",
    name: "Oranga Tamariki",
    shortName: "Oranga Tamariki",
    description: "Care, protection and support services for children and young people.",
    port: 8125, fedPort: 7045, dbPort: 5465,
    localId: { field: "oranga_id", example: "OT-100001" },
    aiPrompt: "What Oranga Tamariki support or placements involve me?",
    entities: [
      {
        key: "care_placements", scope: "oranga:care-placements", label: "Care placements", cardinality: "many",
        fields: [
          { n: "placement_type", j: "placementType", t: "string" },
          { n: "start_date", j: "startDate", t: "date" },
          { n: "region", j: "region", t: "string" },
        ],
        sample: { placementType: "Whānau placement", startDate: "2025-08-01", region: "Waikato" },
      },
      {
        key: "support_services", scope: "oranga:support-services", label: "Support services", cardinality: "many",
        fields: [
          { n: "service", j: "service", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "next_review", j: "nextReview", t: "date" },
        ],
        sample: { service: "Intensive support", status: "active", nextReview: "2026-09-01" },
      },
    ],
    actions: [
      { type: "request-support-review", params: [{ name: "service", validate: "nonempty" }], message: "Support review request received for" },
    ],
  },
  {
    id: "women",
    name: "Ministry for Women",
    shortName: "Women",
    description: "Programmes and insights to improve outcomes for women and girls.",
    port: 8126, fedPort: 7046, dbPort: 5466,
    localId: { field: "women_id", example: "WOM-100001" },
    aiPrompt: "What Ministry for Women programmes am I linked to?",
    entities: [
      {
        key: "programmes", scope: "women:programmes", label: "Programmes", cardinality: "many",
        fields: [
          { n: "programme_name", j: "programmeName", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "year", j: "year", t: "i32" },
        ],
        sample: { programmeName: "Women in Governance", status: "enrolled", year: 2026 },
      },
      {
        key: "insights", scope: "women:insights", label: "Insights", cardinality: "many",
        fields: [
          { n: "topic", j: "topic", t: "string" },
          { n: "summary", j: "summary", t: "string" },
          { n: "published", j: "published", t: "date" },
        ],
        sample: { topic: "Pay equity", summary: "Progress on gender pay gap reporting.", published: "2026-03-08" },
      },
    ],
    actions: [
      { type: "request-programme-info", params: [{ name: "programmeName", validate: "nonempty" }], message: "Programme info request received for" },
    ],
  },
  {
    id: "pacific",
    name: "Ministry for Pacific Peoples",
    shortName: "Pacific Peoples",
    description: "Programmes, language services and support for Pacific communities.",
    port: 8127, fedPort: 7047, dbPort: 5467,
    localId: { field: "pacific_id", example: "PAC-100001" },
    aiPrompt: "What Pacific Peoples services am I eligible for?",
    entities: [
      {
        key: "programmes", scope: "pacific:programmes", label: "Programmes", cardinality: "many",
        fields: [
          { n: "programme_name", j: "programmeName", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "year", j: "year", t: "i32" },
        ],
        sample: { programmeName: "Tokelau Language Week", status: "enrolled", year: 2026 },
      },
      {
        key: "language_services", scope: "pacific:language-services", label: "Language services", cardinality: "many",
        fields: [
          { n: "service", j: "service", t: "string" },
          { n: "region", j: "region", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { service: "Gagana Samoa classes", region: "Auckland", status: "available" },
      },
    ],
    actions: [
      { type: "request-language-service", params: [{ name: "service", validate: "nonempty" }], message: "Language service request received for" },
    ],
  },
  {
    id: "ethnic",
    name: "Ministry for Ethnic Communities",
    shortName: "Ethnic Communities",
    description: "Support, grants and engagement for ethnic communities.",
    port: 8128, fedPort: 7048, dbPort: 5468,
    localId: { field: "ethnic_id", example: "ETH-100001" },
    aiPrompt: "What ethnic community support am I registered for?",
    entities: [
      {
        key: "programmes", scope: "ethnic:programmes", label: "Programmes", cardinality: "many",
        fields: [
          { n: "programme_name", j: "programmeName", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "year", j: "year", t: "i32" },
        ],
        sample: { programmeName: "Ethnic Communities Graduate Programme", status: "enrolled", year: 2026 },
      },
      {
        key: "community_grants", scope: "ethnic:community-grants", label: "Community grants", cardinality: "many",
        fields: [
          { n: "grant_name", j: "grantName", t: "string" },
          { n: "amount", j: "amount", t: "f64" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { grantName: "Community-led response fund", amount: 5000, status: "approved" },
      },
    ],
    actions: [
      { type: "request-grant-info", params: [{ name: "grantName", validate: "nonempty" }], message: "Grant info request received for" },
    ],
  },
  {
    id: "tearawhiti",
    name: "Te Arawhiti",
    shortName: "Te Arawhiti",
    description: "Office for Māori Crown Relations — Treaty settlement and engagement.",
    port: 8129, fedPort: 7049, dbPort: 5469,
    localId: { field: "tearawhiti_id", example: "TAW-100001" },
    aiPrompt: "What Treaty settlement or engagement involves me?",
    entities: [
      {
        key: "treaty_settlements", scope: "tearawhiti:treaty-settlements", label: "Treaty settlements", cardinality: "many",
        fields: [
          { n: "iwi", j: "iwi", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "settled_date", j: "settledDate", t: "date" },
        ],
        sample: { iwi: "Ngāti Toa", status: "settled", settledDate: "2024-07-01" },
      },
      {
        key: "engagements", scope: "tearawhiti:engagements", label: "Engagements", cardinality: "many",
        fields: [
          { n: "topic", j: "topic", t: "string" },
          { n: "engagement_date", j: "engagementDate", t: "date" },
          { n: "outcome", j: "outcome", t: "string" },
        ],
        sample: { topic: "Crown engagement hui", engagementDate: "2026-04-15", outcome: "Recommendation agreed" },
      },
    ],
    actions: [
      { type: "request-settlement-info", params: [{ name: "iwi", validate: "nonempty" }], message: "Settlement info request received for" },
    ],
  },
  {
    id: "regulation",
    name: "Ministry for Regulation",
    shortName: "Regulation",
    description: "Regulatory reviews, system stewardship and better regulation.",
    port: 8130, fedPort: 7050, dbPort: 5470,
    localId: { field: "regulation_id", example: "REG-100001" },
    aiPrompt: "What regulatory reviews affect me?",
    entities: [
      {
        key: "regulatory_reviews", scope: "regulation:regulatory-reviews", label: "Regulatory reviews", cardinality: "many",
        fields: [
          { n: "topic", j: "topic", t: "string" },
          { n: "agency", j: "agency", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "review_year", j: "reviewYear", t: "i32" },
        ],
        sample: { topic: "Building consenting", agency: "MBIE", status: "in-progress", reviewYear: 2026 },
      },
      {
        key: "proposals", scope: "regulation:proposals", label: "Proposals", cardinality: "many",
        fields: [
          { n: "title", j: "title", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { title: "Reduce duplicate reporting", status: "consultation" },
      },
    ],
    actions: [
      { type: "request-review-summary", params: [{ name: "topic", validate: "nonempty" }], message: "Review summary request received for" },
    ],
  },
  {
    id: "worksafe",
    name: "WorkSafe New Zealand",
    shortName: "WorkSafe",
    description: "Workplace health, safety and injury insurance.",
    port: 8131, fedPort: 7051, dbPort: 5471,
    localId: { field: "worksafe_id", example: "WS-100001" },
    aiPrompt: "What WorkSafe inspections or investigations involve me?",
    entities: [
      {
        key: "inspections", scope: "worksafe:inspections", label: "Inspections", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "site", j: "site", t: "string" },
          { n: "inspection_date", j: "inspectionDate", t: "date" },
          { n: "outcome", j: "outcome", t: "string" },
        ],
        sample: { reference: "WS-I-2026-003", site: "Tane Construction Ltd", inspectionDate: "2026-02-10", outcome: "Compliance order issued" },
      },
      {
        key: "investigations", scope: "worksafe:investigations", label: "Investigations", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "matter", j: "matter", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "opened_date", j: "openedDate", t: "date" },
        ],
        sample: { reference: "WS-INV-2026-011", matter: "Fatality inquiry", status: "ongoing", openedDate: "2026-01-15" },
      },
    ],
    actions: [
      { type: "request-investigation-update", params: [{ name: "reference", validate: "nonempty" }], message: "Investigation update request received for reference" },
    ],
  },
  {
    id: "retirement",
    name: "Retirement Commission (Te Ara Ahunga Ora)",
    shortName: "Retirement",
    description: "Retirement income policy, planning and guidance.",
    port: 8132, fedPort: 7052, dbPort: 5472,
    localId: { field: "retirement_id", example: "RET-100001" },
    aiPrompt: "What retirement planning information is held about me?",
    entities: [
      {
        key: "retirement_plan", scope: "retirement:retirement-plan", label: "Retirement plan", cardinality: "one",
        fields: [
          { n: "has_plan", j: "hasPlan", t: "bool" },
          { n: "retirement_age", j: "retirementAge", t: "i32" },
          { n: "last_review", j: "lastReview", t: "date" },
        ],
        sample: { hasPlan: true, retirementAge: 65, lastReview: "2025-12-01" },
      },
      {
        key: "guidance", scope: "retirement:guidance", label: "Guidance", cardinality: "many",
        fields: [
          { n: "topic", j: "topic", t: "string" },
          { n: "summary", j: "summary", t: "string" },
          { n: "published", j: "published", t: "date" },
        ],
        sample: { topic: "KiwiSaver contribution rate", summary: "Consider increasing to 6%.", published: "2026-02-20" },
      },
    ],
    actions: [
      { type: "request-guidance", params: [{ name: "topic", validate: "nonempty" }], message: "Guidance request received for" },
    ],
  },
  {
    id: "mfe",
    name: "Ministry for the Environment",
    shortName: "MfE",
    description: "Environmental reporting, emissions and resource management.",
    port: 8133, fedPort: 7053, dbPort: 5473,
    localId: { field: "mfe_id", example: "MFE-100001" },
    aiPrompt: "What environmental data is held about me?",
    entities: [
      {
        key: "emissions", scope: "mfe:emissions", label: "Emissions", cardinality: "many",
        fields: [
          { n: "report_year", j: "reportYear", t: "i32" },
          { n: "sector", j: "sector", t: "string" },
          { n: "tonnes_co2e", j: "tonnesCO2e", t: "f64" },
        ],
        sample: { reportYear: 2025, sector: "Transport", tonnesCO2e: 3200.5 },
      },
      {
        key: "reports", scope: "mfe:reports", label: "Reports", cardinality: "many",
        fields: [
          { n: "title", j: "title", t: "string" },
          { n: "published", j: "published", t: "date" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { title: "Aotearoa New Zealand's Environment 2026", published: "2026-05-01", status: "published" },
      },
    ],
    actions: [
      { type: "request-emissions-report", params: [{ name: "sector", validate: "nonempty" }], message: "Emissions report request received for sector" },
    ],
  },
  {
    id: "eqc",
    name: "Earthquake Commission (Toka Tū Ake)",
    shortName: "EQC",
    description: "Natural disaster insurance for homes and land.",
    port: 8134, fedPort: 7054, dbPort: 5474,
    localId: { field: "eqc_id", example: "EQC-100001" },
    aiPrompt: "What EQC cover or claims do I have?",
    entities: [
      {
        key: "claims", scope: "eqc:claims", label: "Claims", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "property", j: "property", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "lodged_date", j: "lodgedDate", t: "date" },
        ],
        sample: { reference: "EQC-2026-007", property: "12 Totara Street, Porirua", status: "assessed", lodgedDate: "2026-03-02" },
      },
      {
        key: "cover", scope: "eqc:cover", label: "Cover", cardinality: "one",
        fields: [
          { n: "property", j: "property", t: "string" },
          { n: "sum_insured", j: "sumInsured", t: "f64" },
          { n: "valid_to", j: "validTo", t: "date" },
        ],
        sample: { property: "12 Totara Street, Porirua", sumInsured: 350000, validTo: "2027-01-01" },
      },
    ],
    actions: [
      { type: "request-claim-update", params: [{ name: "reference", validate: "nonempty" }], message: "Claim update request received for reference" },
    ],
  },
  {
    id: "mot",
    name: "Ministry of Transport",
    shortName: "Transport",
    description: "Transport policy, strategies and investment.",
    port: 8135, fedPort: 7055, dbPort: 5475,
    localId: { field: "mot_id", example: "MOT-100001" },
    aiPrompt: "What transport programmes am I linked to?",
    entities: [
      {
        key: "strategies", scope: "mot:strategies", label: "Strategies", cardinality: "many",
        fields: [
          { n: "title", j: "title", t: "string" },
          { n: "year", j: "year", t: "i32" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { title: "Te Tangi a Te Manu", year: 2026, status: "active" },
      },
      {
        key: "programmes", scope: "mot:programmes", label: "Programmes", cardinality: "many",
        fields: [
          { n: "name", j: "name", t: "string" },
          { n: "budget", j: "budget", t: "f64" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { name: "Road maintenance boost", budget: 800000000, status: "funded" },
      },
    ],
    actions: [
      { type: "request-programme-info", params: [{ name: "name", validate: "nonempty" }], message: "Programme info request received for" },
    ],
  },
  {
    id: "caa",
    name: "Civil Aviation Authority",
    shortName: "CAA",
    description: "Aviation safety, pilot licences and aircraft registry.",
    port: 8136, fedPort: 7056, dbPort: 5476,
    localId: { field: "caa_id", example: "CAA-100001" },
    aiPrompt: "What Civil Aviation licences or aircraft are registered to me?",
    entities: [
      {
        key: "licences", scope: "caa:licences", label: "Licences", cardinality: "many",
        fields: [
          { n: "licence_no", j: "licenceNo", t: "string" },
          { n: "category", j: "category", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "expires", j: "expires", t: "date" },
        ],
        sample: { licenceNo: "CAA-P-55821", category: "Private Pilot", status: "current", expires: "2027-06-30" },
      },
      {
        key: "aircraft", scope: "caa:aircraft", label: "Aircraft", cardinality: "many",
        fields: [
          { n: "registration", j: "registration", t: "string" },
          { n: "aircraft_type", j: "aircraftType", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { registration: "ZK-TAN", aircraftType: "Cessna 172", status: "registered" },
      },
    ],
    actions: [
      { type: "request-licence-replacement", params: [{ name: "licenceNo", validate: "nonempty" }], message: "Licence replacement request received for" },
    ],
  },
  {
    id: "maritime",
    name: "Maritime New Zealand",
    shortName: "Maritime",
    description: "Maritime safety, response and vessel regulation.",
    port: 8137, fedPort: 7057, dbPort: 5477,
    localId: { field: "maritime_id", example: "MAR-100001" },
    aiPrompt: "What Maritime NZ vessels or incidents involve me?",
    entities: [
      {
        key: "vessels", scope: "maritime:vessels", label: "Vessels", cardinality: "many",
        fields: [
          { n: "vessel_name", j: "vesselName", t: "string" },
          { n: "flag", j: "flag", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { vesselName: "MV Tane Moana", flag: "NZ", status: "registered" },
      },
      {
        key: "incidents", scope: "maritime:incidents", label: "Incidents", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "incident_type", j: "incidentType", t: "string" },
          { n: "incident_date", j: "incidentDate", t: "date" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { reference: "MAR-2026-02", incidentType: "Pollution", incidentDate: "2026-02-14", status: "resolved" },
      },
    ],
    actions: [
      { type: "report-incident", params: [{ name: "incidentType", validate: "nonempty" }], message: "Incident report received for type" },
    ],
  },
  {
    id: "fenz",
    name: "Fire and Emergency New Zealand",
    shortName: "FENZ",
    description: "Fire safety, emergency response and risk information.",
    port: 8138, fedPort: 7058, dbPort: 5478,
    localId: { field: "fenz_id", example: "FENZ-100001" },
    aiPrompt: "What Fire and Emergency safety information is held about me?",
    entities: [
      {
        key: "fire_safety", scope: "fenz:fire-safety", label: "Fire safety", cardinality: "one",
        fields: [
          { n: "property", j: "property", t: "string" },
          { n: "grade", j: "grade", t: "string" },
          { n: "last_inspection", j: "lastInspection", t: "date" },
        ],
        sample: { property: "12 Totara Street, Porirua", grade: "Compliant", lastInspection: "2025-11-12" },
      },
      {
        key: "incidents", scope: "fenz:incidents", label: "Incidents", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "incident_type", j: "incidentType", t: "string" },
          { n: "incident_date", j: "incidentDate", t: "date" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { reference: "FENZ-2026-050", incidentType: "Structure fire", incidentDate: "2026-01-30", status: "closed" },
      },
    ],
    actions: [
      { type: "request-safety-check", params: [{ name: "property", validate: "nonempty" }], message: "Safety check request received for" },
    ],
  },
  {
    id: "moe",
    name: "Ministry of Education",
    shortName: "Education",
    description: "Education policy, enrolment and student support.",
    port: 8139, fedPort: 7059, dbPort: 5479,
    localId: { field: "moe_id", example: "MOE-100001" },
    aiPrompt: "What education enrolment or support involves me?",
    entities: [
      {
        key: "enrolment", scope: "moe:enrolment", label: "Enrolment", cardinality: "one",
        fields: [
          { n: "school", j: "school", t: "string" },
          { n: "year_level", j: "yearLevel", t: "i32" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { school: "Porirua College", yearLevel: 9, status: "enrolled" },
      },
      {
        key: "student_support", scope: "moe:student-support", label: "Student support", cardinality: "many",
        fields: [
          { n: "service", j: "service", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "next_review", j: "nextReview", t: "date" },
        ],
        sample: { service: "Learning support", status: "active", nextReview: "2026-08-01" },
      },
    ],
    actions: [
      { type: "request-support-review", params: [{ name: "service", validate: "nonempty" }], message: "Support review request received for" },
    ],
  },
  {
    id: "ero",
    name: "Education Review Office",
    shortName: "ERO",
    description: "Reviews and reports on early learning and schools.",
    port: 8140, fedPort: 7060, dbPort: 5480,
    localId: { field: "ero_id", example: "ERO-100001" },
    aiPrompt: "What ERO review information is held about my school?",
    entities: [
      {
        key: "reviews", scope: "ero:reviews", label: "Reviews", cardinality: "many",
        fields: [
          { n: "school", j: "school", t: "string" },
          { n: "rating", j: "rating", t: "string" },
          { n: "review_date", j: "reviewDate", t: "date" },
          { n: "next_review", j: "nextReview", t: "date" },
        ],
        sample: { school: "Porirua College", rating: "Developing", reviewDate: "2025-09-01", nextReview: "2027-09-01" },
      },
      {
        key: "reports", scope: "ero:reports", label: "Reports", cardinality: "many",
        fields: [
          { n: "title", j: "title", t: "string" },
          { n: "published", j: "published", t: "date" },
        ],
        sample: { title: "Porirua College annual report", published: "2025-10-15" },
      },
    ],
    actions: [
      { type: "request-report-copy", params: [{ name: "title", validate: "nonempty" }], message: "Report copy request received for" },
    ],
  },
  {
    id: "tec",
    name: "Tertiary Education Commission",
    shortName: "TEC",
    description: "Tertiary funding, courses and provider performance.",
    port: 8141, fedPort: 7061, dbPort: 5481,
    localId: { field: "tec_id", example: "TEC-100001" },
    aiPrompt: "What tertiary funding or courses involve me?",
    entities: [
      {
        key: "funding", scope: "tec:funding", label: "Funding", cardinality: "many",
        fields: [
          { n: "provider", j: "provider", t: "string" },
          { n: "amount", j: "amount", t: "f64" },
          { n: "year", j: "year", t: "i32" },
        ],
        sample: { provider: "Whitireia", amount: 2200000, year: 2026 },
      },
      {
        key: "courses", scope: "tec:courses", label: "Courses", cardinality: "many",
        fields: [
          { n: "course_name", j: "courseName", t: "string" },
          { n: "provider", j: "provider", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { courseName: "New Zealand Certificate in IT", provider: "Whitireia", status: "approved" },
      },
    ],
    actions: [
      { type: "request-course-info", params: [{ name: "courseName", validate: "nonempty" }], message: "Course info request received for" },
    ],
  },
  {
    id: "mch",
    name: "Ministry for Culture and Heritage",
    shortName: "MCH",
    description: "Arts, culture, heritage and broadcasting.",
    port: 8142, fedPort: 7062, dbPort: 5482,
    localId: { field: "mch_id", example: "MCH-100001" },
    aiPrompt: "What heritage or cultural grants involve me?",
    entities: [
      {
        key: "heritage_sites", scope: "mch:heritage-sites", label: "Heritage sites", cardinality: "many",
        fields: [
          { n: "name", j: "name", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "region", j: "region", t: "string" },
        ],
        sample: { name: "Old St Paul's", status: "Category 1 historic place", region: "Wellington" },
      },
      {
        key: "grants", scope: "mch:grants", label: "Grants", cardinality: "many",
        fields: [
          { n: "grant_name", j: "grantName", t: "string" },
          { n: "amount", j: "amount", t: "f64" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { grantName: "Cultural Innovation Fund", amount: 15000, status: "approved" },
      },
    ],
    actions: [
      { type: "request-grant-info", params: [{ name: "grantName", validate: "nonempty" }], message: "Grant info request received for" },
    ],
  },
  {
    id: "mfat",
    name: "Ministry of Foreign Affairs and Trade",
    shortName: "MFAT",
    description: "Foreign affairs, trade and overseas missions.",
    port: 8143, fedPort: 7063, dbPort: 5483,
    localId: { field: "mfat_id", example: "MFAT-100001" },
    aiPrompt: "What MFAT overseas mission or advisory involves me?",
    entities: [
      {
        key: "overseas_missions", scope: "mfat:overseas-missions", label: "Overseas missions", cardinality: "many",
        fields: [
          { n: "country", j: "country", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { country: "Australia", status: "active" },
      },
      {
        key: "travel_advisories", scope: "mfat:travel-advisories", label: "Travel advisories", cardinality: "many",
        fields: [
          { n: "country", j: "country", t: "string" },
          { n: "level", j: "level", t: "string" },
          { n: "updated", j: "updated", t: "date" },
        ],
        sample: { country: "Indonesia", level: "Exercise increased caution", updated: "2026-03-10" },
      },
    ],
    actions: [
      { type: "request-advisory-update", params: [{ name: "country", validate: "nonempty" }], message: "Advisory update request received for" },
    ],
  },
  {
    id: "defence",
    name: "Ministry of Defence",
    shortName: "Defence",
    description: "Defence policy, procurement and basing.",
    port: 8144, fedPort: 7064, dbPort: 5484,
    localId: { field: "defence_id", example: "DEF-100001" },
    aiPrompt: "What Defence procurements or bases involve me?",
    entities: [
      {
        key: "procurements", scope: "defence:procurements", label: "Procurements", cardinality: "many",
        fields: [
          { n: "programme", j: "programme", t: "string" },
          { n: "value", j: "value", t: "f64" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { programme: "Frigate sustainment", value: 450000000, status: "ongoing" },
      },
      {
        key: "bases", scope: "defence:bases", label: "Bases", cardinality: "many",
        fields: [
          { n: "name", j: "name", t: "string" },
          { n: "location", j: "location", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { name: "Trentham Military Camp", location: "Upper Hutt", status: "operational" },
      },
    ],
    actions: [
      { type: "request-procurement-info", params: [{ name: "programme", validate: "nonempty" }], message: "Procurement info request received for" },
    ],
  },
  {
    id: "nzdf",
    name: "New Zealand Defence Force",
    shortName: "NZDF",
    description: "Defence personnel, services and operations.",
    port: 8145, fedPort: 7065, dbPort: 5485,
    localId: { field: "nzdf_id", example: "NZDF-100001" },
    aiPrompt: "What NZDF service record or deployment involves me?",
    entities: [
      {
        key: "service_records", scope: "nzdf:service-records", label: "Service records", cardinality: "many",
        fields: [
          { n: "service_no", j: "serviceNo", t: "string" },
          { n: "branch", j: "branch", t: "string" },
          { n: "status", j: "status", t: "string" },
        ],
        sample: { serviceNo: "NZDF-55821", branch: "Army", status: "active" },
      },
      {
        key: "deployments", scope: "nzdf:deployments", label: "Deployments", cardinality: "many",
        fields: [
          { n: "operation", j: "operation", t: "string" },
          { n: "country", j: "country", t: "string" },
          { n: "year", j: "year", t: "i32" },
        ],
        sample: { operation: "Burnham readiness", country: "NZ", year: 2025 },
      },
    ],
    actions: [
      { type: "request-service-record", params: [{ name: "serviceNo", validate: "nonempty" }], message: "Service record request received for" },
    ],
  },
  {
    id: "gcsb",
    name: "Government Communications Security Bureau",
    shortName: "GCSB",
    description: "Cyber security, signals intelligence and mandates.",
    port: 8146, fedPort: 7066, dbPort: 5486,
    localId: { field: "gcsb_id", example: "GCSB-100001" },
    aiPrompt: "What GCSB mandates or engagements involve me?",
    entities: [
      {
        key: "mandates", scope: "gcsb:mandates", label: "Mandates", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "agency", j: "agency", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "issued_date", j: "issuedDate", t: "date" },
        ],
        sample: { reference: "GCSB-M-2026-001", agency: "NZSIS", status: "active", issuedDate: "2026-01-05" },
      },
      {
        key: "engagements", scope: "gcsb:engagements", label: "Engagements", cardinality: "many",
        fields: [
          { n: "partner", j: "partner", t: "string" },
          { n: "engagement_type", j: "engagementType", t: "string" },
          { n: "engagement_date", j: "engagementDate", t: "date" },
        ],
        sample: { partner: "CERT NZ", engagementType: "Cyber threat briefing", engagementDate: "2026-02-20" },
      },
    ],
    actions: [
      { type: "request-mandate-info", params: [{ name: "reference", validate: "nonempty" }], message: "Mandate info request received for reference" },
    ],
  },
  {
    id: "nzsis",
    name: "New Zealand Security Intelligence Service",
    shortName: "NZSIS",
    description: "National security intelligence and counter-intelligence.",
    port: 8147, fedPort: 7067, dbPort: 5487,
    localId: { field: "nzsis_id", example: "NZSIS-100001" },
    aiPrompt: "What NZSIS mandates or threats involve me?",
    entities: [
      {
        key: "mandates", scope: "nzsis:mandates", label: "Mandates", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "agency", j: "agency", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "issued_date", j: "issuedDate", t: "date" },
        ],
        sample: { reference: "NZSIS-M-2026-002", agency: "GCSB", status: "active", issuedDate: "2026-01-08" },
      },
      {
        key: "threats", scope: "nzsis:threats", label: "Threats", cardinality: "many",
        fields: [
          { n: "reference", j: "reference", t: "string" },
          { n: "category", j: "category", t: "string" },
          { n: "status", j: "status", t: "string" },
          { n: "assessed_date", j: "assessedDate", t: "date" },
        ],
        sample: { reference: "NZSIS-T-2026-014", category: "Foreign interference", status: "monitored", assessedDate: "2026-02-11" },
      },
    ],
    actions: [
      { type: "request-mandate-info", params: [{ name: "reference", validate: "nonempty" }], message: "Mandate info request received for reference" },
    ],
  },
];

export { DEPTS, T, pascal, camel, upper, sqlLit, W, ROOT };
