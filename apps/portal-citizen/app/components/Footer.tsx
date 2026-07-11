"use client";

import { useLanguage } from "./LanguageProvider";

export default function Footer() {
  const { t } = useLanguage();
  return (
    <footer className="site-footer">
      <div className="site-footer__inner">
        <p>{t("footerNote")}</p>
      </div>
    </footer>
  );
}
