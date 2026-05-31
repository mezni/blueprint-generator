import React from 'react';
import theme from '../styles/theme';

const TOP_LINKS = [
  { key: 'overview', label: '📊 OVERVIEW' },
];

const FLAT_LINKS = [
  { key: 'users', label: 'USERS' },
  { key: 'analytics', label: 'ANALYTICS' },
  { key: 'settings', label: 'SETTINGS' },
  { key: 'logs', label: 'LOGS' },
];

const NESTED_LINKS = [
  { key: 'partners', label: '🤝 PARTNERS' },
  { key: 'stations', label: '⚡ STATIONS' },
];

export default function Sidebar({ activeTab, onTabChange, entitiesOpen, onEntitiesToggle }) {
  return (
    <aside style={styles.sidebar}>
      {TOP_LINKS.map((link) => (
        <button
          key={link.key}
          onClick={() => onTabChange(link.key)}
          style={activeTab === link.key ? styles.sideLinkActive : styles.sideLink}
        >
          {link.label}
        </button>
      ))}

      <button onClick={onEntitiesToggle} style={styles.sideLinkDropdown}>
        📦 ENTITIES {entitiesOpen ? '▼' : '▲'}
      </button>

      {entitiesOpen && (
        <div style={styles.nestedGroup}>
          {NESTED_LINKS.map((link) => (
            <button
              key={link.key}
              onClick={() => onTabChange(link.key)}
              style={activeTab === link.key ? styles.nestedLinkActive : styles.nestedLink}
            >
              {link.label}
            </button>
          ))}
        </div>
      )}

      {FLAT_LINKS.map((link) => (
        <button
          key={link.key}
          onClick={() => onTabChange(link.key)}
          style={activeTab === link.key ? styles.sideLinkActive : styles.sideLink}
        >
          {link.label}
        </button>
      ))}
    </aside>
  );
}

const styles = {
  sidebar: {
    width: '240px',
    backgroundColor: theme.colors.surface,
    borderRight: `1px solid ${theme.colors.border}`,
    display: 'flex',
    flexDirection: 'column',
    padding: '16px 0',
  },
  sideLink: {
    background: 'none',
    border: 'none',
    textAlign: 'left',
    padding: '14px 24px',
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
    color: theme.colors.textSecondary,
    cursor: 'pointer',
  },
  sideLinkActive: {
    background: '#F4F6F8',
    border: 'none',
    textAlign: 'left',
    padding: '14px 24px',
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.extrabold,
    color: theme.colors.textPrimary,
    borderLeft: '4px solid #111111',
    cursor: 'pointer',
  },
  sideLinkDropdown: {
    background: 'none',
    border: 'none',
    textAlign: 'left',
    padding: '14px 24px',
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.extrabold,
    color: theme.colors.textPrimary,
    cursor: 'pointer',
  },
  nestedGroup: {
    backgroundColor: '#F8F9FA',
    display: 'flex',
    flexDirection: 'column',
    paddingLeft: '16px',
    borderLeft: '2px solid #E5E5E5',
    margin: '0 16px 8px 24px',
  },
  nestedLink: {
    background: 'none',
    border: 'none',
    textAlign: 'left',
    padding: '10px 12px',
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: theme.colors.textSecondary,
    cursor: 'pointer',
  },
  nestedLinkActive: {
    background: 'none',
    border: 'none',
    textAlign: 'left',
    padding: '10px 12px',
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.extrabold,
    color: theme.colors.primary,
    cursor: 'pointer',
  },
};
