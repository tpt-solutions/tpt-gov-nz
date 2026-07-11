import type { Metadata } from "next";
import "@tpt/gov-ui/styles.css";
import "./globals.css";
import { LanguageProvider } from "@/app/components/LanguageProvider";
import Header from "@/app/components/Header";
import Footer from "@/app/components/Footer";
import DemoBanner from "@/app/components/DemoBanner";
import GuidedTour from "@/app/components/GuidedTour";
import AiChatWidget from "@/app/components/AiChatWidget";
import ServiceWorkerRegister from "@/app/components/ServiceWorkerRegister";
import { getSession } from "@/app/lib/session";
import { getDemoScenario } from "@/app/lib/demo";
import { PORTAL_CONFIG } from "@/app/lib/config";
import { getAiLevel } from "@/app/ai/level";

export const metadata: Metadata = {
  title: {
    default: "My Gov NZ",
    template: "%s · My Gov NZ",
  },
  description: "Your unified New Zealand government services portal",
  manifest: "/manifest.webmanifest",
};

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  const session = await getSession();
  const scenario = await getDemoScenario();
  const aiEnabled = getAiLevel() !== "none";

  return (
    <html lang="en-NZ">
      <body>
        <LanguageProvider>
          <a href="#main" className="skip-link">
            Skip to main content
          </a>
          {PORTAL_CONFIG.demoMode && <DemoBanner scenario={scenario} />}
          <Header did={session?.did ?? null} demo={session?.demo ?? false} scenario={scenario} />
          <main id="main" className="container">
            {children}
          </main>
          <Footer />
          {PORTAL_CONFIG.demoMode && <GuidedTour />}
          {aiEnabled && <AiChatWidget enabled={aiEnabled} />}
          <ServiceWorkerRegister />
        </LanguageProvider>
      </body>
    </html>
  );
}
