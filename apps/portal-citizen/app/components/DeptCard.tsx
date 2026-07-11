"use client";

import Link from "next/link";
import type { Route } from "next";
import type { DeptMeta } from "@/app/lib/config";
import { Card, Button } from "@tpt/gov-ui";

export default function DeptCard({ dept }: { dept: DeptMeta }) {
  return (
    <Card as="div">
      <h2 style={{ marginBottom: "0.25rem" }}>{dept.name}</h2>
      <p style={{ color: "var(--muted)", margin: "0 0 0.5rem" }}>{dept.shortName}</p>
      <p style={{ margin: 0 }}>{dept.description}</p>
      <Link href={dept.href as Route} style={{ textDecoration: "none" }}>
        <Button size="small" block>
          Open →
        </Button>
      </Link>
    </Card>
  );
}
