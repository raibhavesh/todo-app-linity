// next.config.js
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "export", // Enable static HTML export
  trailingSlash: true, // Optional: helps with routing on S3
};

export default nextConfig;
