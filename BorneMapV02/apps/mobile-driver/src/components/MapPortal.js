import React, { useState, useRef, useMemo } from 'react';
import MapView from './MapView';
import ZoomControls from './ZoomControls';
import mockStations from '../data/mockData';

const NAV_LINKS = [
  { label: 'ABOUT', href: '#about' },
  { label: 'APP', href: '#app' },
  { label: 'MAP', href: '#map' },
  { label: 'CONTACT', href: '#contact' },
];

const PILLS = ['CCS2', 'Type2', 'Available', 'Online'];

export default function MapPortal() {
  const [query, setQuery] = useState('');
  const [activePills, setActivePills] = useState([]);
  const [selectedStation, setSelectedStation] = useState(null);
  const mapRef = useRef(null);

  const togglePill = (pill) => {
    setActivePills((prev) =>
      prev.includes(pill) ? prev.filter((p) => p !== pill) : [...prev, pill]
    );
  };

  const filtered = useMemo(() => {
    let result = mockStations;
    if (query) {
      const q = query.toLowerCase();
      result = result.filter(
        (s) =>
          s.station_name.toLowerCase().includes(q) ||
          s.id.includes(q) ||
          s.partner_name.toLowerCase().includes(q)
      );
    }
    if (activePills.length > 0) {
      result = result.filter((s) => {
        return activePills.every((pill) => {
          if (pill === 'Available')
            return s.chargers.some((c) => c.status === 'Available');
          if (pill === 'Online')
            return s.status === 'Online';
          return s.connector_types.includes(pill);
        });
      });
    }
    return result;
  }, [query, activePills]);

  const handleMarkerPress = (station) => setSelectedStation(station);
  const closeDetail = () => setSelectedStation(null);

  return (
    <div style={{ width: '100%', height: '100%', position: 'relative', display: 'flex', flexDirection: 'column' }}>
      <nav style={styles.navbar}>
        <div style={styles.brand}>BorneMap</div>
        <div style={styles.navLinks}>
          {NAV_LINKS.map((link) => (
            <a key={link.label} href={link.href} style={styles.navLink}>{link.label}</a>
          ))}
        </div>
        <button style={styles.registerBtn}>REGISTER NOW</button>
      </nav>

      <div style={{ flex: 1, position: 'relative' }}>
        <MapView
          style={{ width: '100%', height: '100%' }}
          initialRegion={{ latitude: 36.8065, longitude: 10.1815, latitudeDelta: 0.08, longitudeDelta: 0.04 }}
          stations={filtered}
          onMarkerPress={handleMarkerPress}
          onMapReady={(map) => { mapRef.current = map; }}
        />

        <div style={styles.searchWrapper}>
          <input
            type="text"
            placeholder="Search stations..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            style={styles.searchInput}
            aria-label="Search charging stations"
          />
          <div style={styles.filterPills}>
            {PILLS.map((pill) => (
              <button
                key={pill}
                onClick={() => togglePill(pill)}
                style={{
                  ...styles.filterPill,
                  ...(activePills.includes(pill) ? styles.filterPillActive : {}),
                }}
              >
                {pill}
              </button>
            ))}
          </div>
        </div>

        <ZoomControls
          onZoomIn={() => mapRef.current?.zoomIn()}
          onZoomOut={() => mapRef.current?.zoomOut()}
          onLocateMe={() => {
            if (navigator.geolocation) {
              navigator.geolocation.getCurrentPosition(
                (pos) => mapRef.current?.setView([pos.coords.latitude, pos.coords.longitude], 14),
                () => {}
              );
            }
          }}
          locationDisabled={false}
        />

        {selectedStation && filtered.includes(selectedStation) && (
          <div style={styles.popoverCard}>
            <div style={styles.popoverHeader}>
              <div>
                <div style={styles.popoverTitle}>{selectedStation.station_name}</div>
                <div style={styles.popoverStatus}>
                  <span style={{ color: selectedStation.status === 'Online' ? '#1E7E34' : '#FF9800' }}>●</span>
                  {' '}{selectedStation.status}
                </div>
              </div>
              <button onClick={closeDetail} style={styles.closeBtn}>✕</button>
            </div>
            <div style={styles.popoverMeta}>{selectedStation.partner_name}</div>
            <div style={styles.chargerRow}>
              {selectedStation.chargers.map((ch) => (
                <span key={ch.id} style={styles.chargerChip}>
                  {ch.plug_type} · {ch.power_output}kW · {ch.status}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

const styles = {
  navbar: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    height: 56,
    backgroundColor: '#111111',
    padding: '0 24px',
    zIndex: 100,
  },
  brand: {
    fontSize: 18,
    fontWeight: '800',
    color: '#FFFFFF',
  },
  navLinks: {
    display: 'flex',
    gap: 24,
  },
  navLink: {
    color: '#CCCCCC',
    textDecoration: 'none',
    fontSize: 13,
    fontWeight: '600',
    letterSpacing: 0.5,
  },
  registerBtn: {
    backgroundColor: '#00B653',
    color: '#FFFFFF',
    border: 'none',
    borderRadius: 4,
    padding: '8px 16px',
    fontSize: 13,
    fontWeight: '700',
    cursor: 'pointer',
  },
  searchInput: {
    width: 320,
    height: 44,
    padding: '0 16px',
    border: '1px solid #E5E5E5',
    borderRadius: 8,
    fontSize: 14,
    backgroundColor: '#FFFFFF',
    boxShadow: '0 2px 8px rgba(0,0,0,0.1)',
    outline: 'none',
  },
  searchWrapper: {
    position: 'absolute',
    top: 8,
    left: '50%',
    transform: 'translateX(-50%)',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    zIndex: 40,
  },
  filterPills: {
    display: 'flex',
    gap: 6,
    marginTop: 8,
  },
  filterPill: {
    background: '#FFFFFF',
    border: '1px solid #DDDDDD',
    borderRadius: 16,
    padding: '4px 12px',
    fontSize: 11,
    fontWeight: '600',
    color: '#666666',
    cursor: 'pointer',
  },
  filterPillActive: {
    background: '#00B653',
    borderColor: '#00B653',
    color: '#FFFFFF',
  },
  popoverCard: {
    position: 'absolute',
    bottom: 24,
    left: '50%',
    transform: 'translateX(-50%)',
    width: 400,
    backgroundColor: '#FFFFFF',
    borderRadius: 12,
    padding: 20,
    zIndex: 40,
    boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
  },
  popoverHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 8,
  },
  popoverTitle: {
    fontSize: 16,
    fontWeight: '700',
    color: '#111111',
  },
  popoverStatus: {
    fontSize: 13,
    fontWeight: '600',
    color: '#666666',
    marginTop: 2,
  },
  popoverMeta: {
    fontSize: 12,
    color: '#888888',
    marginBottom: 12,
  },
  chargerRow: {
    display: 'flex',
    flexWrap: 'wrap',
    gap: 6,
  },
  chargerChip: {
    padding: '4px 10px',
    borderRadius: 12,
    backgroundColor: '#F5F5F5',
    fontSize: 11,
    fontWeight: '600',
    color: '#666666',
  },
  closeBtn: {
    background: 'none',
    border: 'none',
    cursor: 'pointer',
    fontSize: 18,
    color: '#888888',
    padding: 4,
  },
};
