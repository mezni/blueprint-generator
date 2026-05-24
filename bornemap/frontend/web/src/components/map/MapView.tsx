import { MapContainer, TileLayer } from "react-leaflet";
import TileFallback from "./TileFallback";
import "leaflet/dist/leaflet.css";

export default function MapView() {
  return (
    <MapContainer
      center={[33.8869, 9.5375]}
      zoom={7}
      className="h-full w-full"
      zoomControl={true}
    >
      <TileFallback />
    </MapContainer>
  );
}
