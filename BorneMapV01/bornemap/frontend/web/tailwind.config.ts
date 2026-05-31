import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        accent: {
          DEFAULT: "#22c55e",
          light: "#4ade80",
          dark: "#16a34a",
          muted: "#dcfce7",
          foreground: "#ffffff",
        },
        surface: {
          DEFAULT: "#ffffff",
          overlay: "rgba(255,255,255,0.92)",
          card: "#f9fafb",
        },
        sidebar: {
          bg: "#f8fafc",
          hover: "#f1f5f9",
          active: "#dcfce7",
          text: "#475569",
          "text-active": "#166534",
        },
      },
      borderRadius: {
        xl: "0.75rem",
        "2xl": "1rem",
      },
      boxShadow: {
        float: "0 4px 24px rgba(0,0,0,0.10)",
        card: "0 2px 12px rgba(0,0,0,0.07)",
      },
    },
  },
  plugins: [],
};

export default config;
