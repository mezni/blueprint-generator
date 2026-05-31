import React, { useEffect } from 'react';
import { MapContainer, TileLayer, Marker, useMap } from 'react-leaflet';
import { divIcon } from 'leaflet';
import 'leaflet/dist/leaflet.css';

const TILE_SERVER = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
const ATTRIBUTION = '&copy; <a href="https://openstreetmap.org/copyright">OpenStreetMap</a>';

function toZoom(delta) {
  return Math.max(1, Math.min(18, Math.round(Math.log(360 / Math.max(delta, 0.001)) / Math.LN2)));
}

function stationIcon(status, isLive) {
  const isAvailable = status === 'Available';
  let bg;
  if (!isLive) {
    bg = '#FF9800';
  } else {
    bg = isAvailable ? '#4CAF50' : '#F44336';
  }
  return divIcon({
    className: '',
    html: `<div style="
      width:22px;height:22px;border-radius:50%;
      background:${bg};
      border:3px solid #fff;box-shadow:0 2px 6px rgba(0,0,0,.3);
    "></div>`,
    iconSize: [22, 22],
    iconAnchor: [11, 11],
  });
}

function MapInit({ onReady }) {
  const map = useMap();
  useEffect(() => { onReady(map); }, []);
  return null;
}

export default function MapView({ stations, onMarkerPress, initialRegion, style, onMapReady }) {
  const center = [initialRegion.latitude, initialRegion.longitude];
  const zoom = toZoom(initialRegion.latitudeDelta || initialRegion.longitudeDelta);

  return (
    <div style={style}>
      <MapContainer
        center={center}
        zoom={zoom}
        style={{ width: '100%', height: '100%' }}
        zoomControl={true}
      >
        {onMapReady && <MapInit onReady={onMapReady} />}
        <TileLayer url={TILE_SERVER} attribution={ATTRIBUTION} />
        {stations.map((s) => (
          <Marker
            key={s.id}
            position={[s.latitude, s.longitude]}
            icon={stationIcon(s.status, s.is_live)}
            eventHandlers={{ click: () => onMarkerPress(s) }}
          />
        ))}
      </MapContainer>
    </div>
  );
}
