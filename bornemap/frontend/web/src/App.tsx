import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import MapView from "./components/map/MapView";
import SearchPanel from "./components/panels/SearchPanel";
import StationListPanel from "./components/panels/StationListPanel";
import StationFormPanel from "./components/panels/StationFormPanel";

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="h-screen w-screen relative overflow-hidden bg-background">
        <MapView />
        <SearchPanel />
        <StationListPanel />
        <StationFormPanel />
      </div>
    </QueryClientProvider>
  );
}
