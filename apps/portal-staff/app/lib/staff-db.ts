import { Pool } from "pg";

/**
 * Connection pool for the staff portal's own database (case notes and their
 * access log). Server-only: the connection string never reaches the browser.
 *
 * Configure with `TPT__GOV__STAFF_DATABASE_URL`, for example
 * `postgresql://staff:…@staff-db:5432/staff`.
 */
let pool: Pool | undefined;

export function staffDbConfigured(): boolean {
  return Boolean(process.env.TPT__GOV__STAFF_DATABASE_URL);
}

export function staffPool(): Pool {
  const connectionString = process.env.TPT__GOV__STAFF_DATABASE_URL;
  if (!connectionString) {
    throw new Error("TPT__GOV__STAFF_DATABASE_URL is not set");
  }
  pool ??= new Pool({ connectionString, max: 5 });
  return pool;
}
