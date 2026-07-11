import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: {
    default: "Policy Lab · My Gov NZ",
    template: "%s · Policy Lab",
  },
  description:
    "AI-assisted simulation of proposed policy changes across government departments",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en-NZ">
      <body>
        <header className="site">
          <a href="/">My Gov NZ — Policy Lab</a>
        </header>
        <main className="container">{children}</main>
      </body>
    </html>
  );
}
