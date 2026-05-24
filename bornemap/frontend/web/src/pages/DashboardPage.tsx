import MapView from "../components/map/MapView";
import SearchPanel from "../components/panels/SearchPanel";
import StationListPanel from "../components/panels/StationListPanel";
import StationFormPanel from "../components/panels/StationFormPanel";

export default function DashboardPage() {
  return (
    <div className="relative h-full w-full overflow-hidden">
      <MapView />
      <SearchPanel />
      <StationListPanel />
      <StationFormPanel />
    </div>
  );
}
