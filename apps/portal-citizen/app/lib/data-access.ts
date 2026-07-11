import { PORTAL_CONFIG, type DeptId } from "./config";
import { getCitizenDid } from "./session";
import { getDemoScenario } from "./demo";
import { getDemoData } from "./mock-data";

export type DeptBundle =
  | import("@tpt/gov-schema").IRDDataBundle
  | import("@tpt/gov-schema").WINZDataBundle
  | import("@tpt/gov-schema").MOHDataBundle
  | import("@tpt/gov-schema").DIADataBundle
  | import("@tpt/gov-schema").NZTADataBundle
  | import("@tpt/gov-schema").ACCDataBundle
  | import("@tpt/gov-schema").MOJDataBundle
  | import("@tpt/gov-schema").PoliceDataBundle
  | import("@tpt/gov-schema").HUDDataBundle
  | import("@tpt/gov-schema").NZQADataBundle
  | import("@tpt/gov-schema").MSDDataBundle
  | import("@tpt/gov-schema").MBIEDataBundle
  | import("@tpt/gov-schema").LINZDataBundle
  | import("@tpt/gov-schema").STATSNZDataBundle
  | import("@tpt/gov-schema").CORRECTIONSDataBundle
  | import("@tpt/gov-schema").CUSTOMSDataBundle
  | import("@tpt/gov-schema").MPIDataBundle
  | import("@tpt/gov-schema").DOCDataBundle
  | import("@tpt/gov-schema").TPKDataBundle;
  | import("@tpt/gov-schema").NzsisDataBundle;
  | import("@tpt/gov-schema").GcsbDataBundle;
  | import("@tpt/gov-schema").NzdfDataBundle;
  | import("@tpt/gov-schema").DefenceDataBundle;
  | import("@tpt/gov-schema").MfatDataBundle;
  | import("@tpt/gov-schema").MchDataBundle;
  | import("@tpt/gov-schema").TecDataBundle;
  | import("@tpt/gov-schema").EroDataBundle;
  | import("@tpt/gov-schema").MoeDataBundle;
  | import("@tpt/gov-schema").FenzDataBundle;
  | import("@tpt/gov-schema").MaritimeDataBundle;
  | import("@tpt/gov-schema").CaaDataBundle;
  | import("@tpt/gov-schema").MotDataBundle;
  | import("@tpt/gov-schema").EqcDataBundle;
  | import("@tpt/gov-schema").MfeDataBundle;
  | import("@tpt/gov-schema").RetirementDataBundle;
  | import("@tpt/gov-schema").WorksafeDataBundle;
  | import("@tpt/gov-schema").RegulationDataBundle;
  | import("@tpt/gov-schema").TearawhitiDataBundle;
  | import("@tpt/gov-schema").EthnicDataBundle;
  | import("@tpt/gov-schema").PacificDataBundle;
  | import("@tpt/gov-schema").WomenDataBundle;
  | import("@tpt/gov-schema").OrangaDataBundle;
  | import("@tpt/gov-schema").SfoDataBundle;
  | import("@tpt/gov-schema").CrownlawDataBundle;
  | import("@tpt/gov-schema").PublicserviceDataBundle;
  | import("@tpt/gov-schema").DpmcDataBundle;
  | import("@tpt/gov-schema").TreasuryDataBundle;

/**
 * Single data-access path used by every department module. In demo mode it
 * returns the seeded Alex Tane fixture for the active scenario; otherwise it
 * calls the department's native service directly with the signed-in DID.
 */
export async function fetchDeptData(dept: DeptId, scopes: string[]): Promise<DeptBundle | null> {
  if (PORTAL_CONFIG.demoMode) {
    const scenario = await getDemoScenario();
    const data = getDemoData(scenario);
    return data[dept] as DeptBundle;
  }

  const did = await getCitizenDid();
  if (!did) return null;

  const url = PORTAL_CONFIG.services[dept];
  try {
    const res = await fetch(`${url}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });
    if (!res.ok) return null;
    return (await res.json()) as DeptBundle;
  } catch {
    return null;
  }
}
