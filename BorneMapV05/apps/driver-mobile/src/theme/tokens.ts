export const colors = {
  primary: { base: "#2563EB", hover: "#1D4ED8", active: "#1E40AF", muted: "#BFDBFE" },
  secondary: { base: "#6B7280", hover: "#4B5563", active: "#374151", muted: "#E5E7EB" },
  accent: { base: "#F59E0B", hover: "#D97706", active: "#B45309", muted: "#FDE68A" },
  success: { base: "#10B981", hover: "#059669", active: "#047857", muted: "#A7F3D0" },
  warning: { base: "#F97316", hover: "#EA580C", active: "#C2410C", muted: "#FED7AA" },
  error: { base: "#EF4444", hover: "#DC2626", active: "#B91C1C", muted: "#FECACA" },
  surface: { base: "#FFFFFF", hover: "#F9FAFB", active: "#F3F4F6", muted: "#E5E7EB" },
  text: { base: "#111827", hover: "#1F2937", active: "#374151", muted: "#9CA3AF" },
  border: { base: "#D1D5DB", hover: "#9CA3AF", active: "#6B7280", muted: "#E5E7EB" },
} as const;

export const spacing = {
  4: "4px",
  8: "8px",
  12: "12px",
  16: "16px",
  20: "20px",
  24: "24px",
  32: "32px",
  48: "48px",
  64: "64px",
} as const;

export const borderRadius = {
  sm: "4px",
  md: "6px",
  lg: "8px",
  full: "9999px",
} as const;

export const shadows = {
  sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)",
  md: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
  lg: "0 10px 15px -3px rgb(0 0 0 / 0.1)",
  card: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
  modal: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
} as const;

export const fontFamily = {
  sans: "Inter, system-ui, -apple-system, sans-serif",
  mono: "ui-monospace, SFMono-Regular, monospace",
} as const;
