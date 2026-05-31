import React from 'react';
import theme from '../styles/theme';

export default function TopBar() {
  return (
    <header style={styles.topBar}>
      <div style={styles.adminTitle}>🌐 BorneMap Sandbox Master Console (No Integration)</div>
      <div style={styles.badge}>MOCK ENGINE ACTIVE</div>
    </header>
  );
}

const styles = {
  topBar: {
    height: '64px',
    backgroundColor: '#111111',
    color: '#FFFFFF',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '0 24px',
  },
  adminTitle: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.extrabold,
  },
  badge: {
    backgroundColor: theme.colors.danger,
    padding: '6px 12px',
    borderRadius: theme.borderRadius.sm,
    fontSize: theme.fontSize.xs,
    fontWeight: theme.fontWeight.extrabold,
  },
};
