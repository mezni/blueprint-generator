import { useState } from "react";
import { TileLayer, useMap } from "react-leaflet";

const PRIMARY_URL = "https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png";
const PRIMARY_ATTR = '&copy; <a href="https://www.openstreetmap.org/copyright">OSM</a> contributors, &copy; <a href="https://carto.com/">CARTO</a>';

const FALLBACK_URL = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";
const FALLBACK_ATTR = "&copy; OpenStreetMap contributors";

export default function TileFallback() {
  const [useFallback, setUseFallback] = useState(false);
  const [showWarning, setShowWarning] = useState(false);
  const map = useMap();

  return (
    <>
      <TileLayer
        url={useFallback ? FALLBACK_URL : PRIMARY_URL}
        attribution={useFallback ? FALLBACK_ATTR : PRIMARY_ATTR}
        eventHandlers={{
          tileerror: () => {
            if (!useFallback) {
              setUseFallback(true);
              setShowWarning(true);
            }
          },
        }}
      />
      {showWarning && (
        <div className="absolute top-0 left-0 right-0 z-[1000] bg-red-600 text-white text-center text-sm py-1 px-2">
          Map tiles unavailable — using fallback tiles
        </div>
      )}
    </>
  );
}
