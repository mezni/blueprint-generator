import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
  plugins: [react()],
  resolve: {
    // Force a single copy of React. The monorepo has react@19.2.3 hoisted to
    // the root (pinned by driver-mobile/react-native) and react@19.2.7 nested
    // under apps/driver-web, which otherwise causes "Invalid hook call" from
    // two React instances. Dedupe + explicit aliases pin everything to this
    // app's own copy.
    dedupe: ["react", "react-dom"],
    alias: {
      "@": path.resolve(__dirname, "./src"),
      react: path.resolve(__dirname, "./node_modules/react"),
      "react-dom": path.resolve(__dirname, "./node_modules/react-dom"),
    },
  },
  server: {
    proxy: {
      // Proxy API + auth traffic to the Traefik gateway (port 80) so the
      // browser makes same-origin requests and avoids CORS in dev.
      "/api": {
        target: "http://localhost",
        changeOrigin: true,
      },
      "/auth": {
        target: "http://localhost",
        changeOrigin: true,
      },
    },
  },
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/lib/test-utils.ts"],
  },
});
