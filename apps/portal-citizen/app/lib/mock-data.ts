import type {
  IRDDataBundle,
  WINZDataBundle,
  MOHDataBundle,
  DIADataBundle,
  NZTADataBundle,
  ACCDataBundle,
  MOJDataBundle,
  PoliceDataBundle,
  HUDDataBundle,
  NZQADataBundle,
  MSDDataBundle,
  MBIEDataBundle,
  LINZDataBundle,
  STATSNZDataBundle,
  CORRECTIONSDataBundle,
  CUSTOMSDataBundle,
  MPIDataBundle,
  DOCDataBundle,
  TPKDataBundle,
} from "@tpt/gov-schema";

export type ScenarioId = "standard" | "beneficiary" | "new-parent";

export interface ScenarioMeta {
  id: ScenarioId;
  label: string;
  teReoLabel: string;
  description: string;
}

export const SCENARIOS: ScenarioMeta[] = [
  {
    id: "standard",
    label: "Standard",
    teReoLabel: "Whānui",
    description: "Employed, filing a tax return with a refund, KiwiSaver member.",
  },
  {
    id: "beneficiary",
    label: "Beneficiary",
    teReoLabel: "Kaiwhiwhi pūtea",
    description: "Receiving a jobseeker benefit and accommodation support, with two tamariki.",
  },
  {
    id: "new-parent",
    label: "New parent",
    teReoLabel: "Mātua hou",
    description: "Recently had a baby — checking Working for Families and parental leave.",
  },
];

export const CITIZEN_NAME = "Alex Tane";

export interface DemoData {
  ird: IRDDataBundle;
  winz: WINZDataBundle;
  moh: MOHDataBundle;
  dia: DIADataBundle;
  nzta: NZTADataBundle;
  acc: ACCDataBundle;
  moj: MOJDataBundle;
  police: PoliceDataBundle;
  hud: HUDDataBundle;
  nzqa: NZQADataBundle;
  msd: MSDDataBundle;
  mbie: MBIEDataBundle;
  linz: LINZDataBundle;
  statsnz: STATSNZDataBundle;
  corrections: CORRECTIONSDataBundle;
  customs: CUSTOMSDataBundle;
  mpi: MPIDataBundle;
  doc: DOCDataBundle;
  tpk: TPKDataBundle;
}

const irdStandard: IRDDataBundle = {
  irdNumber: "123-456-789",
  currentTaxYear: {
    assessmentYear: 2025,
    taxCode: "M",
    employmentIncome: 84000,
    totalIncome: 84000,
    taxableIncome: 84000,
    taxLiability: 17600,
    taxPaid: 18900,
    taxRefundDue: 1300,
    taxOwing: 0,
    assessmentStatus: "final",
  },
  taxHistory: [
    {
      assessmentYear: 2024,
      taxCode: "M",
      totalIncome: 81000,
      taxableIncome: 81000,
      taxLiability: 16800,
      taxPaid: 17220,
      taxRefundDue: 420,
      taxOwing: 0,
      assessmentStatus: "final",
    },
    {
      assessmentYear: 2023,
      taxCode: "M",
      totalIncome: 78000,
      taxableIncome: 78000,
      taxLiability: 16000,
      taxPaid: 16110,
      taxRefundDue: 0,
      taxOwing: 110,
      assessmentStatus: "final",
    },
  ],
  gstRegistered: false,
  kiwiSaver: {
    membershipStatus: "active",
    contributionRate: 3,
    employerContributionRate: 3,
    scheme: "Simplicity Growth",
    totalBalance: 58200,
    lastContributionDate: "2025-05-15",
    governmentContributionEligible: true,
    firstHomeBuyerEligible: true,
  },
  workingForFamilies: {
    eligible: false,
    incomeThreshold: 0,
    currentIncome: 84000,
    numberOfDependantChildren: 0,
    currentEntitlement: undefined,
  },
};

const irdBeneficiary: IRDDataBundle = {
  irdNumber: "123-456-789",
  currentTaxYear: {
    assessmentYear: 2025,
    taxCode: "M",
    employmentIncome: 12000,
    totalIncome: 12000,
    taxableIncome: 12000,
    taxLiability: 0,
    taxPaid: 0,
    taxRefundDue: 0,
    taxOwing: 0,
    assessmentStatus: "estimated",
  },
  taxHistory: [],
  gstRegistered: false,
  kiwiSaver: {
    membershipStatus: "active",
    contributionRate: 3,
    employerContributionRate: 3,
    scheme: "Kiwi Wealth Balanced",
    totalBalance: 9400,
    lastContributionDate: "2025-01-20",
    governmentContributionEligible: true,
    firstHomeBuyerEligible: true,
  },
  workingForFamilies: {
    eligible: true,
    incomeThreshold: 76848,
    currentIncome: 12000,
    numberOfDependantChildren: 2,
    currentEntitlement: {
      familyTaxCredit: 224.0,
      inWorkTaxCredit: 72.0,
      bestStartPayment: 0,
      minimumFamilyTaxCredit: 0,
      totalWeeklyEntitlement: 296.0,
    },
    paymentFrequency: "weekly",
    nextReviewDate: "2025-11-01",
  },
};

const irdNewParent: IRDDataBundle = {
  irdNumber: "123-456-789",
  currentTaxYear: {
    assessmentYear: 2025,
    taxCode: "M",
    employmentIncome: 48000,
    totalIncome: 48000,
    taxableIncome: 48000,
    taxLiability: 6600,
    taxPaid: 6900,
    taxRefundDue: 300,
    taxOwing: 0,
    assessmentStatus: "provisional",
  },
  taxHistory: [
    {
      assessmentYear: 2024,
      taxCode: "M",
      totalIncome: 72000,
      taxableIncome: 72000,
      taxLiability: 14700,
      taxPaid: 14940,
      taxRefundDue: 240,
      taxOwing: 0,
      assessmentStatus: "final",
    },
  ],
  gstRegistered: false,
  kiwiSaver: {
    membershipStatus: "active",
    contributionRate: 6,
    employerContributionRate: 3,
    scheme: "ANZ SmartSteps Growth",
    totalBalance: 21500,
    lastContributionDate: "2025-04-02",
    governmentContributionEligible: true,
    firstHomeBuyerEligible: true,
  },
  workingForFamilies: {
    eligible: true,
    incomeThreshold: 53376,
    currentIncome: 48000,
    numberOfDependantChildren: 1,
    currentEntitlement: {
      familyTaxCredit: 128.0,
      inWorkTaxCredit: 72.0,
      bestStartPayment: 65.0,
      minimumFamilyTaxCredit: 0,
      totalWeeklyEntitlement: 265.0,
    },
    paymentFrequency: "weekly",
    nextReviewDate: "2026-02-15",
  },
};

const winzStandard: WINZDataBundle = {
  clientId: "WINZ-000000",
  activeBenefits: [],
  totalWeeklyPayment: "0.00",
  payments: [],
  caseNotes: [],
};

const winzBeneficiary: WINZDataBundle = {
  clientId: "WINZ-000001",
  activeBenefits: [
    {
      type: "jobseeker",
      weeklyAmount: "276.40",
      startDate: "2024-03-10",
      reviewDate: "2025-11-01",
      status: "active",
    },
    {
      type: "accommodation-supplement",
      weeklyAmount: "145.00",
      startDate: "2024-03-10",
      status: "active",
    },
  ],
  totalWeeklyPayment: "421.40",
  payments: [
    {
      paymentId: "P-9001",
      benefitType: "jobseeker",
      paymentDate: "2025-06-09",
      amount: "276.40",
      method: "bank-deposit",
    },
    {
      paymentId: "P-9002",
      benefitType: "accommodation-supplement",
      paymentDate: "2025-06-09",
      amount: "145.00",
      method: "bank-deposit",
    },
  ],
  caseNotes: [
    {
      noteId: "N-1",
      noteDate: "2025-05-20",
      author: "Case worker",
      note: "Client advised of upcoming 6-month review.",
    },
  ],
  caseManagerName: "R. Walker",
  nextAppointment: "2025-11-01",
};

const winzNewParent: WINZDataBundle = {
  clientId: "WINZ-000002",
  activeBenefits: [
    {
      type: "sole-parent",
      weeklyAmount: "329.57",
      startDate: "2025-03-01",
      reviewDate: "2026-02-15",
      status: "active",
    },
  ],
  totalWeeklyPayment: "329.57",
  payments: [
    {
      paymentId: "P-9101",
      benefitType: "sole-parent",
      paymentDate: "2025-06-10",
      amount: "329.57",
      method: "bank-deposit",
    },
  ],
  caseNotes: [],
  caseManagerName: "H. Tamati",
  nextAppointment: "2026-02-15",
};

const mohStandard: MOHDataBundle = {
  nhiNumber: "BHN1234",
  enrolledGP: {
    practiceName: "Ponsonby Road Medical",
    address: "12 Ponsonby Road, Auckland 1011",
    phone: "09 376 1234",
  },
  activePrescriptions: [
    {
      medication: "Rivaroxaban",
      dose: "20mg",
      repeatsRemaining: 2,
      issuedAt: "2025-05-15",
    },
  ],
  upcomingAppointments: [
    {
      provider: "Dr L. Chen",
      date: "2025-07-22T10:30:00+12:00",
      type: "General check-up",
      status: "booked",
    },
  ],
  vaccinations: [
    {
      vaccine: "Influenza 2025",
      date: "2025-04-01",
      dueForBooster: false,
    },
  ],
};

const mohBeneficiary: MOHDataBundle = {
  nhiNumber: "BHN1234",
  enrolledGP: {
    practiceName: "Otara Family Health",
    address: "30 East Tamaki Road, Auckland 2023",
    phone: "09 274 0022",
  },
  activePrescriptions: [
    {
      medication: "Salbutamol inhaler",
      dose: "100mcg",
      repeatsRemaining: 4,
      issuedAt: "2025-04-02",
    },
    {
      medication: "Fluticasone",
      dose: "50mcg",
      repeatsRemaining: 1,
      issuedAt: "2025-03-10",
    },
  ],
  upcomingAppointments: [
    {
      provider: "Dr P. Singh",
      date: "2025-07-15T14:00:00+12:00",
      type: "Asthma review",
      status: "booked",
    },
  ],
  vaccinations: [
    {
      vaccine: "Influenza 2025",
      date: "2025-04-01",
      dueForBooster: false,
    },
    {
      vaccine: "COVID-19 booster",
      date: "2024-02-10",
      dueForBooster: true,
    },
  ],
};

const mohNewParent: MOHDataBundle = {
  nhiNumber: "BHN1234",
  enrolledGP: {
    practiceName: "Ponsonby Road Medical",
    address: "12 Ponsonby Road, Auckland 1011",
    phone: "09 376 1234",
  },
  activePrescriptions: [
    {
      medication: "Iron supplement",
      dose: "325mg",
      repeatsRemaining: 3,
      issuedAt: "2025-06-01",
    },
  ],
  upcomingAppointments: [
    {
      provider: "Plunket nurse",
      date: "2025-07-08T11:00:00+12:00",
      type: "Wellchild tamariki check",
      status: "booked",
    },
  ],
  vaccinations: [
    {
      vaccine: "Infant 6-week (baby)",
      date: "2025-05-01",
      dueForBooster: false,
    },
    {
      vaccine: "Influenza 2025 (parent)",
      date: "2025-03-15",
      dueForBooster: false,
    },
  ],
};

const diaStandard: DIADataBundle = {
  passportNumber: "RE1234567",
  passport: {
    passportNumber: "RE1234567",
    expiryDate: "2028-09-30",
    renewable: true,
  },
  birthCertificate: {
    certificateNumber: "BC-1989-0001",
    dateOfBirth: "1989-09-12",
    placeOfBirth: "Auckland, New Zealand",
  },
  citizenship: {
    status: "citizen-by-birth",
  },
};

const diaBeneficiary: DIADataBundle = {
  passportNumber: "RE1234567",
  passport: {
    passportNumber: "RE1234567",
    expiryDate: "2026-03-15",
    renewable: true,
  },
  birthCertificate: {
    certificateNumber: "BC-1991-0042",
    dateOfBirth: "1991-03-02",
    placeOfBirth: "Hamilton, New Zealand",
  },
  citizenship: {
    status: "citizen-by-birth",
  },
};

const diaNewParent: DIADataBundle = {
  passportNumber: "RE1234567",
  passport: {
    passportNumber: "RE1234567",
    expiryDate: "2028-09-30",
    renewable: true,
  },
  birthCertificate: {
    certificateNumber: "BC-1992-0077",
    dateOfBirth: "1992-07-21",
    placeOfBirth: "Wellington, New Zealand",
  },
  citizenship: {
    status: "citizen-by-birth",
  },
};

const nztaStandard: NZTADataBundle = {
  driverLicenceNumber: "NZ1234567",
  driverLicence: {
    licenceNumber: "NZ1234567",
    fullName: "Alex Tane",
    licenceClass: "1 (car)",
    expiryDate: "2028-09-30",
    conditions: undefined,
  },
  vehicles: [
    {
      registration: "ABC123",
      make: "Toyota",
      model: "Corolla",
      year: 2021,
      fuelType: "Petrol",
      registrationExpiry: "2026-12-01",
    },
  ],
  ruc: [
    {
      vehicleRego: "ABC123",
      licenceType: "Heavy vehicle RUC",
      expiryDate: "2027-06-30",
      unitsRemaining: 1500,
    },
  ],
};

const nztaBeneficiary: NZTADataBundle = {
  driverLicenceNumber: "NZ7654321",
  driverLicence: {
    licenceNumber: "NZ7654321",
    fullName: "Alex Tane",
    licenceClass: "6 (motorcycle)",
    expiryDate: "2027-12-31",
    conditions: undefined,
  },
  vehicles: [
    {
      registration: "XYZ789",
      make: "Honda",
      model: "CBR",
      year: 2020,
      fuelType: "Petrol",
      registrationExpiry: "2027-01-01",
    },
  ],
  ruc: [],
};

const nztaNewParent: NZTADataBundle = {
  driverLicenceNumber: "NZ1234567",
  driverLicence: {
    licenceNumber: "NZ1234567",
    fullName: "Alex Tane",
    licenceClass: "1 (car)",
    expiryDate: "2028-09-30",
    conditions: undefined,
  },
  vehicles: [
    {
      registration: "ABC123",
      make: "Toyota",
      model: "Corolla",
      year: 2021,
      fuelType: "Petrol",
      registrationExpiry: "2026-12-01",
    },
    {
      registration: "DEF456",
      make: "Mitsubishi",
      model: "Outlander",
      year: 2024,
      fuelType: "Plug-in Hybrid",
      registrationExpiry: "2027-03-15",
    },
  ],
  ruc: [
    {
      vehicleRego: "DEF456",
      licenceType: "Light vehicle RUC",
      expiryDate: "2027-03-15",
      unitsRemaining: 4000,
    },
  ],
};

const accStandard: ACCDataBundle = {
  clientNumber: "ACC-100001",
  claims: [
    {
      claimNumber: "ACC-5001",
      claimType: "work",
      status: "open",
      injuryDate: "2025-02-10",
      description: "Lower back strain",
      weeklyCompensation: 420,
    },
  ],
  entitlements: {
    hasEntitlement: true,
    type: "Weekly compensation",
    weeklyAmount: 420,
    remainingWeeks: 18,
  },
  rehabilitation: [
    {
      planId: "PLAN-1",
      description: "Physio + return-to-work",
      status: "active",
      provider: "Metro Rehab",
      nextReview: "2026-01-15",
    },
  ],
};

const accBeneficiary: ACCDataBundle = {
  clientNumber: "ACC-100001",
  claims: [
    {
      claimNumber: "ACC-5001",
      claimType: "work",
      status: "open",
      injuryDate: "2025-02-10",
      description: "Lower back strain",
      weeklyCompensation: 420,
    },
  ],
  entitlements: {
    hasEntitlement: true,
    type: "Weekly compensation",
    weeklyAmount: 420,
    remainingWeeks: 6,
  },
  rehabilitation: [
    {
      planId: "PLAN-1",
      description: "Physio + return-to-work",
      status: "active",
      provider: "Metro Rehab",
      nextReview: "2025-09-30",
    },
  ],
};

const accNewParent: ACCDataBundle = {
  clientNumber: "ACC-100001",
  claims: [],
  entitlements: {
    hasEntitlement: false,
  },
  rehabilitation: [],
};

const mojStandard: MOJDataBundle = {
  clientNumber: "MOJ-100001",
  fines: [],
  disputes: [],
  courtRecords: [],
};

const mojBeneficiary: MOJDataBundle = {
  clientNumber: "MOJ-100001",
  fines: [
    {
      fineNumber: "MOJ-F5001",
      fineType: "traffic",
      status: "unpaid",
      amount: 150,
      offenseDate: "2026-05-01",
      dueDate: "2026-06-15",
      description: "Speeding 15km/h over limit",
    },
  ],
  disputes: [
    {
      disputeNumber: "MOJ-D2001",
      claimType: "tenancy",
      status: "filed",
      amountClaimed: 1200,
      hearingDate: "2026-08-20",
      description: "Bond dispute with former landlord",
    },
  ],
  courtRecords: [],
};

const mojNewParent: MOJDataBundle = {
  clientNumber: "MOJ-100001",
  fines: [],
  disputes: [],
  courtRecords: [],
};

const policeStandard: PoliceDataBundle = {
  clientNumber: "POL-100001",
  infringements: [],
  reports: [],
};

const policeBeneficiary: PoliceDataBundle = {
  clientNumber: "POL-100001",
  infringements: [
    {
      ticketNumber: "POL-T5001",
      offenseType: "speeding",
      status: "unpaid",
      amount: 120,
      issueDate: "2026-06-01",
      location: "State Highway 1, Wellington",
      demeritPoints: 20,
      description: "Exceeding speed limit by 15km/h",
    },
  ],
  reports: [
    {
      reportNumber: "POL-R2001",
      reportType: "theft",
      status: "under-investigation",
      filedDate: "2026-05-20",
      description: "Bicycle stolen from outside address",
    },
  ],
};

const policeNewParent: PoliceDataBundle = {
  clientNumber: "POL-100001",
  infringements: [],
  reports: [],
};

const hudStandard: HUDDataBundle = {
  clientNumber: "HUD-100001",
  applications: [],
  tenancies: [],
  maintenanceRequests: [],
};

const hudBeneficiary: HUDDataBundle = {
  clientNumber: "HUD-100001",
  applications: [
    {
      applicationNumber: "HUD-A5001",
      applicationType: "public-housing",
      status: "waitlisted",
      priorityBand: "B",
      bedroomsNeeded: 2,
      submittedDate: "2026-01-15",
    },
  ],
  tenancies: [
    {
      tenancyId: "HUD-TEN-1",
      propertyAddress: "12 Totara Street, Porirua",
      weeklyRent: 180,
      incomeRelatedRent: true,
      startDate: "2025-11-01",
      status: "active",
    },
  ],
  maintenanceRequests: [
    {
      requestNumber: "HUD-M3001",
      category: "plumbing",
      status: "scheduled",
      description: "Leaking kitchen tap",
      requestedDate: "2026-06-20",
    },
  ],
};

const hudNewParent: HUDDataBundle = {
  clientNumber: "HUD-100001",
  applications: [],
  tenancies: [],
  maintenanceRequests: [],
};

const nzqaStandard: NZQADataBundle = {
  nsn: "NSN-100001",
  qualifications: [
    { qualificationId: "NZQA-Q1", title: "Bachelor of Science", level: 7, awardedDate: "2024-12-10", provider: "University of Auckland" },
  ],
  transcript: { recordSummary: "Full Record of Achievement on file.", totalCredits: 360, creditSummary: "Level 7: 360 credits" },
};

const nzqaBeneficiary: NZQADataBundle = nzqaStandard;

const nzqaNewParent: NZQADataBundle = nzqaStandard;

const msdStandard: MSDDataBundle = {
  clientNumber: "MSD-100001",
  studylink: { hasStudentLoan: false, hasAllowance: false },
  caseHistory: [],
};

const msdBeneficiary: MSDDataBundle = {
  clientNumber: "MSD-100001",
  studylink: {
    hasStudentLoan: true,
    loanBalance: 24000,
    repaymentPlan: "standard",
    hasAllowance: true,
    allowanceType: "student-allowance",
    nextPaymentDate: "2026-07-15",
    weeklyAmount: 272,
  },
  caseHistory: [
    { eventDate: "2025-02-01", serviceLine: "StudyLink", summary: "Student allowance application approved." },
    { eventDate: "2025-07-01", serviceLine: "StudyLink", summary: "Student loan repayment plan set to standard." },
  ],
};

const msdNewParent: MSDDataBundle = {
  clientNumber: "MSD-100001",
  studylink: { hasStudentLoan: false, hasAllowance: false },
  caseHistory: [
    { eventDate: "2026-05-20", serviceLine: "Parental Leave", summary: "Parental leave payment established." },
  ],
};

const mbieStandard: MBIEDataBundle = {
  personId: "MBIE-P-100001",
  businessRegistrations: [
    { nzbn: "9429000000001", entityName: "Tane Digital Ltd", entityType: "company", status: "registered", registeredDate: "2022-03-04" },
  ],
  directorships: [
    { nzbn: "9429000000001", entityName: "Tane Digital Ltd", role: "Director", appointedDate: "2022-03-04" },
  ],
};

const mbieBeneficiary: MBIEDataBundle = mbieStandard;

const mbieNewParent: MBIEDataBundle = mbieStandard;

const linzStandard: LINZDataBundle = {
  customerId: "LINZ-100001",
  titles: [
    { titleNumber: "NA12B/123", propertyAddress: "12 Totara Street, Porirua", landAreaSqm: 612, estateType: "Fee simple" },
  ],
  ownership: [
    { titleNumber: "NA12B/123", ownershipShare: "1/1", registeredOwners: ["Alex Tane"] },
  ],
};

const linzBeneficiary: LINZDataBundle = linzStandard;

const linzNewParent: LINZDataBundle = {
  customerId: "LINZ-100001",
  titles: [
    { titleNumber: "NA12B/123", propertyAddress: "12 Totara Street, Porirua", landAreaSqm: 612, estateType: "Fee simple" },
    { titleNumber: "NA20C/456", propertyAddress: "4 Rata Crescent, Lower Hutt", landAreaSqm: 480, estateType: "Fee simple" },
  ],
  ownership: [
    { titleNumber: "NA12B/123", ownershipShare: "1/1", registeredOwners: ["Alex Tane"] },
    { titleNumber: "NA20C/456", ownershipShare: "1/2", registeredOwners: ["Alex Tane", "Mere Tane"] },
  ],
};

const statsnzStandard: STATSNZDataBundle = {
  statsId: "STATS-100001",
  census: [{ censusYear: 2023, dwellingType: "House", householdSize: 3, region: "Wellington" }],
  profile: { dataSummary: "2023 Census respondent. Standard data profile on file.", recordCount: 1, lastUpdated: "2023-11-01" },
};
const statsnzBeneficiary: STATSNZDataBundle = statsnzStandard;
const statsnzNewParent: STATSNZDataBundle = statsnzStandard;

const correctionsStandard: CORRECTIONSDataBundle = {
  correctionsId: "COR-100001",
  probation: { status: "not-under-supervision", officerName: "", nextReportDate: "", location: "" },
  case: [],
};
const correctionsBeneficiary: CORRECTIONSDataBundle = correctionsStandard;
const correctionsNewParent: CORRECTIONSDataBundle = correctionsStandard;

const customsStandard: CUSTOMSDataBundle = {
  travellerId: "CUST-100001",
  travel: { passportNumber: "C1234567", lastArrival: "2026-04-12", arrivalPort: "Auckland", frequentTraveller: true },
  declarations: [
    { declarationId: "CUST-D1", date: "2026-04-12", countryFrom: "Australia", goodsDeclared: "Personal effects", status: "cleared" },
  ],
};
const customsBeneficiary: CUSTOMSDataBundle = customsStandard;
const customsNewParent: CUSTOMSDataBundle = customsStandard;

const mpiStandard: MPIDataBundle = {
  mpiId: "MPI-100001",
  registrations: [
    { nzbn: "9429000000002", businessName: "Tane Orchards", type: "orchard", status: "registered", registeredDate: "2021-09-15" },
  ],
  certifications: [
    { certNumber: "MPI-C1", category: "food-safety", issuedDate: "2025-01-20", expiresDate: "2026-01-20" },
  ],
};
const mpiBeneficiary: MPIDataBundle = mpiStandard;
const mpiNewParent: MPIDataBundle = mpiStandard;

const docStandard: DOCDataBundle = {
  docId: "DOC-100001",
  permits: [
    { permitNumber: "DOC-P1", activity: "Recreational fishing", location: "Wellington region", status: "active", expiresDate: "2026-12-31" },
  ],
  concessions: [],
};
const docBeneficiary: DOCDataBundle = docStandard;
const docNewParent: DOCDataBundle = docStandard;

const tpkStandard: TPKDataBundle = {
  tpkId: "TPK-100001",
  programmes: [
    { programmeName: "Te Arawhiti mātauranga", status: "enrolled", region: "Wellington" },
  ],
  funding: [
    { grantId: "TPK-G1", amount: 5000, purpose: "Marae renovation", status: "approved" },
  ],
};
const tpkBeneficiary: TPKDataBundle = tpkStandard;
const tpkNewParent: TPKDataBundle = tpkStandard;

export function getDemoData(scenario: ScenarioId): DemoData {
  switch (scenario) {
    case "beneficiary":
      return {
        ird: irdBeneficiary,
        winz: winzBeneficiary,
        moh: mohBeneficiary,
        dia: diaBeneficiary,
        nzta: nztaBeneficiary,
        acc: accBeneficiary,
        moj: mojBeneficiary,
        police: policeBeneficiary,
        hud: hudBeneficiary,
        nzqa: nzqaBeneficiary,
        msd: msdBeneficiary,
        mbie: mbieBeneficiary,
        linz: linzBeneficiary,
        statsnz: statsnzBeneficiary,
        corrections: correctionsBeneficiary,
        customs: customsBeneficiary,
        mpi: mpiBeneficiary,
        doc: docBeneficiary,
        tpk: tpkBeneficiary,
      };
    case "new-parent":
      return {
        ird: irdNewParent,
        winz: winzNewParent,
        moh: mohNewParent,
        dia: diaNewParent,
        nzta: nztaNewParent,
        acc: accNewParent,
        moj: mojNewParent,
        police: policeNewParent,
        hud: hudNewParent,
        nzqa: nzqaNewParent,
        msd: msdNewParent,
        mbie: mbieNewParent,
        linz: linzNewParent,
        statsnz: statsnzNewParent,
        corrections: correctionsNewParent,
        customs: customsNewParent,
        mpi: mpiNewParent,
        doc: docNewParent,
        tpk: tpkNewParent,
      };
    case "standard":
    default:
      return {
        ird: irdStandard,
        winz: winzStandard,
        moh: mohStandard,
        dia: diaStandard,
        nzta: nztaStandard,
        acc: accStandard,
        moj: mojStandard,
        police: policeStandard,
        hud: hudStandard,
        nzqa: nzqaStandard,
        msd: msdStandard,
        mbie: mbieStandard,
        linz: linzStandard,
        statsnz: statsnzStandard,
        corrections: correctionsStandard,
        customs: customsStandard,
        mpi: mpiStandard,
        doc: docStandard,
        tpk: tpkStandard,
      };
  }
}

/** Default bundle (standard) used when a scenario-specific lookup is not needed. */
export const DEFAULT_DEMO_DATA: DemoData = getDemoData("standard");
