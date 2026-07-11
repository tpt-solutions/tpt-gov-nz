export interface PolicyScenario {
  id: string;
  title: string;
  summary: string;
  affectedDepartments: string[];
  /** A concrete parameter the simulation is run against. */
  parameter: string;
  /** The proposed change expressed as a sentence for the model. */
  change: string;
}

export const POLICY_SCENARIOS: PolicyScenario[] = [
  {
    id: "min-wage-25",
    title: "Raise adult minimum wage to $25.50",
    summary:
      "Increase the adult minimum wage from $23.15 to $25.50 and model flow-on effects.",
    affectedDepartments: ["WINZ", "IRD", "MBIE", "MSD"],
    parameter: "adult_minimum_wage = 25.50",
    change:
      "The adult minimum wage is increased from $23.15 to $25.50 per hour, effective next tax year.",
  },
  {
    id: "parental-leave-26",
    title: "Extend paid parental leave to 26 weeks",
    summary:
      "Lengthen paid parental leave from 22 to 26 weeks and estimate fiscal and service impact.",
    affectedDepartments: ["MSD", "WINZ", "MOH"],
    parameter: "paid_parental_leave_weeks = 26",
    change:
      "Paid parental leave is extended from 22 weeks to 26 weeks, with no change to eligibility.",
  },
  {
    id: "benefit-abatement",
    title: "Lift benefit abatement threshold",
    summary:
      "Raise the income abatement threshold so beneficiaries keep more of part-time earnings.",
    affectedDepartments: ["WINZ", "MSD", "IRD"],
    parameter: "abatement_threshold = 160 (from 150)",
    change:
      "The benefit abatement threshold is raised from $150 to $160 of weekly earnings before abatement begins.",
  },
  {
    id: "gst-15",
    title: "Reduce GST to 12.5%",
    summary:
      "Model a reduction in the goods and services tax rate and its cross-department revenue effect.",
    affectedDepartments: ["IRD", "Treasury", "MBIE"],
    parameter: "gst_rate = 0.125",
    change:
      "The goods and services tax (GST) rate is reduced from 15% to 12.5%, applied uniformly.",
  },
  {
    id: "fuel-excise-cut",
    title: "Cut fuel excise by 12c/litre",
    summary:
      "Temporary cut to fuel excise and its effect on transport, cost of living and revenue.",
    affectedDepartments: ["Treasury", "IRD", "NZTA", "MOT"],
    parameter: "fuel_excise_cpl = 0.12 (reduction)",
    change:
      "The per-litre fuel excise duty is cut by 12 cents for a 12-month period.",
  },
];

export function getScenario(id: string): PolicyScenario | undefined {
  return POLICY_SCENARIOS.find((s) => s.id === id);
}
