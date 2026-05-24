import { MapView } from "../map/MapView";
import { TelemetryFeed } from "../telemetry/TelemetryFeed";
import { FaultQueue } from "../queue/FaultQueue";

export function DashboardPage() {
  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-hidden">
      <section className="grid grid-cols-5 gap-6 h-1/2 flex-shrink-0 min-h-[350px]">
        <div className="col-span-3 rounded-xl border border-slate-800 bg-slate-900 overflow-hidden flex flex-col">
          <div className="px-4 py-3 border-b border-slate-800">
            <span className="text-sm font-medium">Live Fleet Map</span>
          </div>
          <div className="flex-1 relative">
            <MapView />
          </div>
        </div>
        <div className="col-span-2">
          <TelemetryFeed />
        </div>
      </section>
      <section className="flex-1 min-h-[200px]">
        <FaultQueue />
      </section>
    </div>
  );
}
