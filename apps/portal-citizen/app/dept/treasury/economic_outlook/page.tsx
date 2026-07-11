import Link from "next/link";
import { fetchTreasuryData } from "../actions";

export const metadata = { title: "Economic outlook — The Treasury — My Gov NZ" };

export default async function TreasuryEconomicOutlookPage() {
  const data = await fetchTreasuryData(["treasury:economic-outlook"]);
  if (!data) {
    return (
      <main>
        <h1>Economic outlook</h1>
        <p>Unable to load your Treasury information.</p>
        <Link href={"/consent?dept=treasury"}>Grant Treasury access</Link>
      </main>
    );
  }

  const item = data.economic_outlook;

  return (
    <main>
      <Link href={"/dept/treasury"}>← Back to Treasury</Link>
      <h1>Economic outlook</h1>
      {item ? (
        <div>
        <p><strong>forecastYear:</strong> {item.forecastYear}</p>
        <p><strong>gdpGrowthPct:</strong> {item.gdpGrowthPct}</p>
        <p><strong>inflationPct:</strong> {item.inflationPct}</p>
        <p><strong>netDebtPct:</strong> {item.netDebtPct}</p>
        </div>
      ) : (
        <p>No economic outlook on file.</p>
      )}
    </main>
  );
}
