import { useEffect, useState } from "react";
import { MapContainer, TileLayer, Marker, Popup, useMap } from "react-leaflet";
import MarkerClusterGroup from "react-leaflet-cluster";
import { useViewportStations } from "./useViewportStations";
import { pinColor } from "@bornemap/geo-models";
import type { MapViewportModel, StationMarkerModel } from "@bornemap/geo-models";
import L from "leaflet";

const TUNISIA_CENTER: [number, number] = [36.8, 10.2];
const DEFAULT_ZOOM = 7;

function createIcon(color: string) {
  return L.divIcon({
    html: `<div style="background:${color};width:24px;height:24px;border-radius:50%;border:2px solid white;"></div>`,
    className: "",
    iconSize: [24, 24],
    iconAnchor: [12, 12],
  });
}

function MapEvents({
  onViewportChange,
}: {
  onViewportChange: (v: MapViewportModel) => void;
}) {
  const map = useMap();
  useEffect(() => {
    const handler = () => {
      const b = map.getBounds();
      onViewportChange({
        west: b.getWest(),
        south: b.getSouth(),
        east: b.getEast(),
        north: b.getNorth(),
      });
    };
    map.on("moveend", handler);
    map.on("zoomend", handler);
    handler();
    return () => {
      map.off("moveend", handler);
      map.off("zoomend", handler);
    };
  }, [map, onViewportChange]);
  return null;
}

export function MapView() {
  const [viewport, setViewport] = useState<MapViewportModel | null>(null);
  const { markers, isLoading, isError } = useViewportStations(viewport);

  const handleViewportChange = (v: MapViewportModel) => setViewport(v);

  return (
    <div className="absolute inset-0">
      {isError && (
        <div className="absolute top-3 left-3 z-[1000] bg-red-500/20 text-red-400 px-2.5 py-1 rounded text-[11px] font-medium">
          Failed to load markers
        </div>
      )}
      <MapContainer
        center={TUNISIA_CENTER}
        zoom={DEFAULT_ZOOM}
        className="h-full w-full"
      >
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <MapEvents onViewportChange={handleViewportChange} />
        <MarkerClusterGroup
          chunkedLoading
          maxClusterRadius={40}
          spiderfyOnMaxZoom
          showCoverageOnHover={false}
        >
          {markers.map((m: StationMarkerModel) => (
            <Marker
              key={m.id}
              position={[m.coord[1], m.coord[0]]}
              icon={createIcon(pinColor(m))}
            >
              <Popup>
                <strong>{m.name}</strong>
                <br />
                {m.isActive ? "Active" : "Inactive"}
                {m.underMaintenance ? " (Maintenance)" : ""}
              </Popup>
            </Marker>
          ))}
        </MarkerClusterGroup>
      </MapContainer>
      {isLoading && (
        <div className="absolute top-3 right-3 z-[1000] bg-slate-800/80 text-slate-300 px-2.5 py-1 rounded text-[11px] font-medium">
          Loading...
        </div>
      )}
    </div>
  );
}
