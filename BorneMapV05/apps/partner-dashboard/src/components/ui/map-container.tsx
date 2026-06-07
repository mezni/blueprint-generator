import * as React from "react";
import L from "leaflet";
import "leaflet/dist/leaflet.css";
import { cn } from "@/lib/utils";

export interface MapContainerProps {
  className?: string;
  center?: [number, number];
  zoom?: number;
  onMount?: (map: L.Map) => void;
}

const MapContainer = React.forwardRef<HTMLDivElement, MapContainerProps>(
  ({ className, center = [36.8065, 10.1815], zoom = 13, onMount }, ref) => {
    const mapRef = React.useRef<L.Map | null>(null);
    const containerRef = React.useRef<HTMLDivElement>(null);

    React.useEffect(() => {
      if (mapRef.current || !containerRef.current) return;

      const map = L.map(containerRef.current, {
        center,
        zoom,
        zoomControl: true,
      });

      L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
        attribution:
          '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
        maxZoom: 19,
      }).addTo(map);

      mapRef.current = map;
      onMount?.(map);

      return () => {
        map.remove();
        mapRef.current = null;
      };
    }, []);

    React.useEffect(() => {
      if (mapRef.current) {
        mapRef.current.setView(center, zoom);
      }
    }, [center, zoom]);

    return (
      <div
        ref={(node) => {
          (containerRef as React.MutableRefObject<HTMLDivElement | null>).current = node;
          if (typeof ref === "function") ref(node);
          else if (ref) ref.current = node;
        }}
        className={cn("h-[500px] w-full rounded-[var(--radius-md)]", className)}
      />
    );
  },
);
MapContainer.displayName = "MapContainer";

export { MapContainer };
