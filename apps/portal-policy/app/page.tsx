import Link from "next/link";
import { POLICY_SCENARIOS } from "@/app/lib/policies";

export default function Home() {
  return (
    <div>
      <h1>Policy Lab</h1>
      <p>
        Model the cross-government impact of proposed policy changes with AI
        assistance. Pick a scenario to run a structured simulation across the
        departments it touches.
      </p>
      <div className="notice">
        Simulations are illustrative planning aids for policy makers, not legal
        or fiscal advice.
      </div>

      <h2>Scenarios</h2>
      {POLICY_SCENARIOS.map((s) => (
        <div className="card" key={s.id}>
          <h3>{s.title}</h3>
          <p>{s.summary}</p>
          <p>
            {s.affectedDepartments.map((d) => (
              <span className="badge" key={d}>
                {d}
              </span>
            ))}
          </p>
          <Link className="btn" href={`/simulate?id=${s.id}`}>
            Simulate
          </Link>
        </div>
      ))}
    </div>
  );
}
