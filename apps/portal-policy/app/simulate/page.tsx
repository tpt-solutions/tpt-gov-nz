import { notFound } from "next/navigation";
import { getScenario } from "@/app/lib/policies";
import SimulateForm from "./SimulateForm";

export default async function SimulatePage({
  searchParams,
}: {
  searchParams: Promise<{ id?: string }>;
}) {
  const { id } = await searchParams;
  const scenario = id ? getScenario(id) : undefined;
  if (!scenario) notFound();

  return (
    <SimulateForm
      id={scenario.id}
      title={scenario.title}
      summary={scenario.summary}
      departments={scenario.affectedDepartments}
    />
  );
}
