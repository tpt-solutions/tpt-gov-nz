import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  webpack: (config) => {
    // Workspace packages (@tpt/*) are consumed as TypeScript source and import
    // their siblings with `.js` extensions. Teach webpack to resolve those to
    // the corresponding `.ts`/`.tsx` files.
    config.resolve.extensionAlias = {
      ...config.resolve.extensionAlias,
      ".js": [".ts", ".tsx", ".js"],
    };
    return config;
  },
  async headers() {
    // Content-Security-Policy is set per request in middleware.ts (nonce-based).
    return [
      {
        source: "/(.*)",
        headers: [
          { key: "X-Frame-Options", value: "DENY" },
          { key: "X-Content-Type-Options", value: "nosniff" },
          { key: "Referrer-Policy", value: "strict-origin-when-cross-origin" },
          { key: "Strict-Transport-Security", value: "max-age=63072000; includeSubDomains" },
          { key: "Permissions-Policy", value: "camera=(), microphone=(), geolocation=()" },
        ],
      },
    ];
  },
};

export default nextConfig;
