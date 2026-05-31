import React from 'react';
import theme from '../styles/theme';

const METRICS = [
  { label: 'PARTNERS', value: '148' },
  { label: 'STATIONS', value: '1,240' },
  { label: 'MOCK TELEMETRY HITS', value: 'Offline' },
];

export default function OverviewMetrics() {
  return (
    <div style={styles.metricsGrid}>
      {METRICS.map((m) => (
        <div key={m.label} style={styles.metricCard}>
          <span style={styles.cardLabel}>{m.label}</span>
          <h3 style={styles.cardValue}>{m.value}</h3>
        </div>
      ))}
    </div>
  );
}

const styles = {
  metricsGrid: {
    display: 'flex',
    gap: '20px',
  },
  metricCard: {
    flex: 1,
    backgroundColor: theme.colors.surface,
    border: `1px solid ${theme.colors.border}`,
    borderRadius: theme.borderRadius.md,
    padding: '20px',
  },
  cardLabel: {
    fontSize: theme.fontSize.xs,
    fontWeight: theme.fontWeight.bold,
    color: theme.colors.textMuted,
  },
  cardValue: {
    fontSize: theme.fontSize.xxxl,
    fontWeight: theme.fontWeight.extrabold,
    marginTop: '4px',
  },
};
