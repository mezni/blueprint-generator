import MapView from "../components/map/MapView";

export default function PreviewPage() {
  return (
    <div className="relative h-full w-full overflow-hidden">
      <MapView />
      <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
        <div className="pointer-events-auto rounded-2xl bg-white/90 px-6 py-4 shadow-float backdrop-blur-md border border-white/60">
          <p className="text-sm text-slate-600">
            Preview mode — map without admin panels
          </p>
        </div>
      </div>
    </div>
  );
}
