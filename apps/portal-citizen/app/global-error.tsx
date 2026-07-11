"use client";

export default function GlobalError({ error, reset }: { error: Error; reset: () => void }) {
  return (
    <html lang="en-NZ">
      <body
        style={{
          fontFamily: "system-ui, sans-serif",
          margin: 0,
          padding: "2rem",
          background: "#f4f5f7",
          color: "#1b1b1b",
        }}
      >
        <h1>Something went wrong</h1>
        <p>We’re sorry — the page failed to load.</p>
        <p style={{ color: "#5a5f66", fontSize: "0.85rem" }}>{error.message || "Unknown error"}</p>
        <button
          type="button"
          onClick={reset}
          style={{
            background: "#005a9c",
            color: "#fff",
            border: "none",
            borderRadius: 10,
            padding: "0.55rem 1rem",
            cursor: "pointer",
          }}
        >
          Try again
        </button>
      </body>
    </html>
  );
}
