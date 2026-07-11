import type { MetadataRoute } from "next";

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: "My Gov NZ",
    short_name: "My Gov NZ",
    description: "Your unified New Zealand government services portal",
    start_url: "/",
    display: "standalone",
    background_color: "#f4f5f7",
    theme_color: "#005a9c",
    icons: [],
  };
}
