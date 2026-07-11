import { fetchTreasuryDataForCitizen } from "../actions";

export const metadata = { title: "Economic outlook — The Treasury — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TreasuryEconomicOutlookStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTreasuryDataForCitizen(did, ["treasury:economic-outlook"]);
  const item = data?.economic_outlook;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/treasury?did=${encodeURIComponent(did)}`}>← Back to Treasury case file</Link>
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
