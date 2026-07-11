"use client";

export default function Error({ error, reset }: { error: Error; reset: () => void }) {
  return (
    <section className="card">
      <h1 style={{ marginTop: 0 }}>Something went wrong</h1>
      <p>An unexpected error occurred while loading this page.</p>
      <p style={{ color: "var(--muted)", fontSize: "0.85rem" }}>
        {error.message || "Unknown error"}
      </p>
      <button type="button" className="btn" onClick={reset}>
        Try again
      </button>
    </section>
  );
}
