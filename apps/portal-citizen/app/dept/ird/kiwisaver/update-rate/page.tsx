import { fetchIrdData } from "../../actions";
import { getAiLevel } from "../../../../ai/level";
import UpdateKiwiSaverRateForm from "./form";

export const metadata = { title: "Change KiwiSaver Rate — IRD — My Gov NZ" };

export default async function UpdateKiwiSaverRatePage() {
  const aiLevel = getAiLevel();
  const data = await fetchIrdData(["ird:kiwisaver"]);
  const currentRate = data?.kiwiSaver?.contributionRate != null
    ? Number(data.kiwiSaver.contributionRate)
    : null;

  return <UpdateKiwiSaverRateForm aiLevel={aiLevel} currentRate={currentRate} />;
}
