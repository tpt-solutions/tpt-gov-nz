import type {
  IRDDataBundle,
  WINZDataBundle,
  MOHDataBundle,
  DIADataBundle,
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
    },
  ],
  upcomingAppointments: [
    {
      provider: "Dr L. Chen",
      date: "2025-07-22T10:30:00+12:00",
      type: "General check-up",
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
    },
    {
      medication: "Fluticasone",
      dose: "50mcg",
      repeatsRemaining: 1,
    },
  ],
  upcomingAppointments: [
    {
      provider: "Dr P. Singh",
      date: "2025-07-15T14:00:00+12:00",
      type: "Asthma review",
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
    },
  ],
  upcomingAppointments: [
    {
      provider: "Plunket nurse",
      date: "2025-07-08T11:00:00+12:00",
      type: "Wellchild tamariki check",
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

export function getDemoData(scenario: ScenarioId): DemoData {
  switch (scenario) {
    case "beneficiary":
      return {
        ird: irdBeneficiary,
        winz: winzBeneficiary,
        moh: mohBeneficiary,
        dia: diaBeneficiary,
      };
    case "new-parent":
      return {
        ird: irdNewParent,
        winz: winzNewParent,
        moh: mohNewParent,
        dia: diaNewParent,
      };
    case "standard":
    default:
      return {
        ird: irdStandard,
        winz: winzStandard,
        moh: mohStandard,
        dia: diaStandard,
      };
  }
}

/** Default bundle (standard) used when a scenario-specific lookup is not needed. */
export const DEFAULT_DEMO_DATA: DemoData = getDemoData("standard");
