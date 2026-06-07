export const colors = {
  'ev-bg': '#F8FAF6',
  'ev-surface': '#FFFFFF',
  'ev-green': '#007943',
  'ev-glow': '#00E676',
  'ev-mapBg': '#EAF0E6',
  'ev-muted': '#6B7280',
  'ev-border': '#E5E7EB',
  'ev-textMain': '#111827',
} as const

export type EvColor = keyof typeof colors
