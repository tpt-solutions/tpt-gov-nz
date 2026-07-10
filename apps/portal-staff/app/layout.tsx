import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "My Gov NZ — Staff",
  description: "Case-worker portal for New Zealand government services",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en-NZ">
      <body>
        <header style={{ borderBottom: "2px solid #ccc", padding: "0.75rem 1rem" }}>
          <strong>My Gov NZ — Staff Portal</strong>
          <span style={{ marginLeft: "1rem", color: "#555", fontSize: "0.85rem" }}>
            Read-only case worker view
          </span>
        </header>
        {children}
      </body>
    </html>
  );
}
