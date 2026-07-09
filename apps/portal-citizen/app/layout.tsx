import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "My Gov NZ",
  description: "Your unified New Zealand government services portal",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en-NZ">
      <body>{children}</body>
    </html>
  );
}
