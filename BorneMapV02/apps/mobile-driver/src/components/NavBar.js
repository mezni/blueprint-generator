import React from 'react';
import { useAppContext } from '../context/AppContext';

const NAV_ITEMS = [
  { id: 'map', label: 'Map', icon: '🗺️' },
  { id: 'explore', label: 'Explore', icon: '🔍' },
  { id: 'saved', label: 'Saved', icon: '⭐' },
  { id: 'profile', label: 'Profile', icon: '👤' },
];

export default function NavBar() {
  const { activeTab, setActiveTab } = useAppContext();

  return (
    <nav style={styles.nav}>
      <div style={styles.brand}>BorneMap</div>
      <div style={styles.items}>
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            onClick={() => setActiveTab(item.id)}
            style={{
              ...styles.item,
              ...(activeTab === item.id ? styles.activeItem : {}),
            }}
            aria-label={item.label}
            aria-current={activeTab === item.id ? 'page' : undefined}
          >
            <span style={styles.icon}>{item.icon}</span>
            <span style={styles.label}>{item.label}</span>
            {activeTab === item.id && <div style={styles.underline} />}
          </button>
        ))}
      </div>
    </nav>
  );
}

const styles = {
  nav: {
    display: 'flex',
    alignItems: 'center',
    height: 56,
    backgroundColor: '#FFFFFF',
    borderBottomWidth: 1,
    borderBottomStyle: 'solid',
    borderBottomColor: '#EEEEEE',
    paddingHorizontal: 16,
    zIndex: 100,
  },
  brand: {
    fontSize: 18,
    fontWeight: '700',
    color: '#111111',
    marginRight: 32,
  },
  items: {
    display: 'flex',
    gap: 4,
    flex: 1,
    justifyContent: 'center',
  },
  item: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: 2,
    padding: '8px 16px',
    border: 'none',
    background: 'none',
    cursor: 'pointer',
    position: 'relative',
    minHeight: 44,
    minWidth: 44,
  },
  activeItem: {
    opacity: 1,
  },
  icon: {
    fontSize: 20,
    lineHeight: 1,
  },
  label: {
    fontSize: 11,
    fontWeight: '600',
    color: '#666666',
  },
  underline: {
    position: 'absolute',
    bottom: 0,
    left: 16,
    right: 16,
    height: 3,
    backgroundColor: '#007AFF',
    borderRadius: '1.5px 1.5px 0 0',
  },
};
