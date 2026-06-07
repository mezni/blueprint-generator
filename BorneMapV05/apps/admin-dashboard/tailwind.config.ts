import type { Config } from "tailwindcss";
import {
  colors as colorsSrc,
  spacing,
  typography,
  shadows,
  borderRadius,
} from "@bornemap/design-tokens";

const colors = colorsSrc as unknown as Record<string, Record<string, string>>;

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: Object.fromEntries(
        Object.entries(colors).map(([key, val]) => {
          if (typeof val === "object" && val !== null && "base" in val) {
            return [key, { DEFAULT: val.base, ...val }];
          }
          return [key, val];
        }),
      ),
      spacing: spacing as Record<string, string>,
      fontFamily: typography.fontFamily,
      fontSize: typography.fontSize,
      fontWeight: typography.fontWeight,
      lineHeight: typography.lineHeight,
      boxShadow: shadows as Record<string, string>,
      borderRadius: borderRadius as Record<string, string>,
    },
  },
  plugins: [],
} satisfies Config;
