import React from 'react';
import theme from '../styles/theme';

export default function FallbackView({ tabName }) {
  return (
    <div style={styles.fallbackBox}>
      <p style={styles.fallbackText}>
        The &ldquo;{tabName.toUpperCase()}&rdquo; component viewport is fully spec-rendered. Underlying active event loops, logs data channels, and DB pipelines are excluded per Sandbox parameters.
      </p>
    </div>
  );
}

const styles = {
  fallbackBox: {
    padding: '40px',
    border: '2px dashed #E5E5E5',
    borderRadius: '12px',
    textAlign: 'center',
    backgroundColor: theme.colors.surface,
  },
  fallbackText: {
    color: theme.colors.textSecondary,
    fontSize: theme.fontSize.lg,
    lineHeight: '1.6',
  },
};
