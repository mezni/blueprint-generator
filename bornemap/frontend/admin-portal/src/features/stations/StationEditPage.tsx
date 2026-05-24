import { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { apiFetch } from "../../lib/apiClient";

interface StationDetail {
  id: string;
  name: string;
  address: string;
  coord: [number, number];
  is_active: boolean;
  under_maintenance: boolean;
  opening_hours_osm: string | null;
}

export function StationEditPage({ isNew = false }: { isNew?: boolean }) {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [address, setAddress] = useState("");
  const [lng, setLng] = useState("10.1815");
  const [lat, setLat] = useState("36.8065");
  const [isActive, setIsActive] = useState(true);
  const [underMaintenance, setUnderMaintenance] = useState(false);
  const [openingHours, setOpeningHours] = useState("");
  const [companyId] = useState("00000000-0000-0000-0000-000000000001");
  const [loading, setLoading] = useState(false);
  const [fetching, setFetching] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    if (!isNew && id) {
      setFetching(true);
      apiFetch<StationDetail>(`/api/v1/stations/${id}`)
        .then((s) => {
          setName(s.name);
          setAddress(s.address);
          setLng(String(s.coord[0]));
          setLat(String(s.coord[1]));
          setIsActive(s.is_active);
          setUnderMaintenance(s.under_maintenance);
          setOpeningHours(s.opening_hours_osm || "");
        })
        .catch((err: Error) => setError(err.message))
        .finally(() => setFetching(false));
    }
  }, [id, isNew]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    const lngNum = parseFloat(lng);
    const latNum = parseFloat(lat);
    if (isNaN(lngNum) || isNaN(latNum)) {
      setError("Invalid coordinates");
      return;
    }
    setLoading(true);
    try {
      const body = {
        company_id: companyId,
        name,
        address,
        coord: [lngNum, latNum],
        is_active: isActive,
        under_maintenance: underMaintenance,
        opening_hours_osm: openingHours || null,
      };
      if (isNew) {
        await apiFetch("/api/v1/admin/stations", {
          method: "POST",
          body: JSON.stringify(body),
        });
      } else {
        await apiFetch(`/api/v1/admin/stations/${id}`, {
          method: "PATCH",
          body: JSON.stringify({
            name,
            address,
            coord: [lngNum, latNum],
            is_active: isActive,
            under_maintenance: underMaintenance,
            opening_hours_osm: openingHours || null,
          }),
        });
      }
      navigate("/admin/stations");
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="h-full flex flex-col p-6 overflow-y-auto max-w-2xl">
      <h1 className="text-lg font-bold mb-6">
        {isNew ? "New Station" : "Edit Station"}
      </h1>
      {error && (
        <div className="bg-red-500/20 text-red-400 p-3 rounded-lg mb-4 text-xs border border-red-500/30">
          {error}
        </div>
      )}
      {fetching ? (
        <p className="text-sm text-slate-500">Loading station data...</p>
      ) : (
      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="rounded-xl border border-slate-800 bg-slate-900 p-5 space-y-4">
          <div>
            <label className="block text-xs font-medium text-slate-400 mb-1.5">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
              required
            />
          </div>
          <div>
            <label className="block text-xs font-medium text-slate-400 mb-1.5">Address</label>
            <input
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
              required
            />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5">Longitude</label>
              <input
                type="number"
                step="0.0001"
                value={lng}
                onChange={(e) => setLng(e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
                required
              />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5">Latitude</label>
              <input
                type="number"
                step="0.0001"
                value={lat}
                onChange={(e) => setLat(e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
                required
              />
            </div>
          </div>
          <div>
            <label className="block text-xs font-medium text-slate-400 mb-1.5">Opening Hours (OSM format)</label>
            <input
              value={openingHours}
              onChange={(e) => setOpeningHours(e.target.value)}
              placeholder="e.g. Mo-Fr 08:00-20:00; Sa 09:00-13:00"
              className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
          </div>
          <div className="flex items-center gap-6">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={isActive}
                onChange={(e) => setIsActive(e.target.checked)}
                className="rounded border-slate-700 bg-slate-800 text-blue-500 focus:ring-blue-500/40"
              />
              <span className="text-sm text-slate-300">Active</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={underMaintenance}
                onChange={(e) => setUnderMaintenance(e.target.checked)}
                className="rounded border-slate-700 bg-slate-800 text-blue-500 focus:ring-blue-500/40"
              />
              <span className="text-sm text-slate-300">Under Maintenance</span>
            </label>
          </div>
        </div>
        <div className="flex gap-3">
          <button
            type="submit"
            disabled={loading}
            className="px-5 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-500 disabled:opacity-50 transition-colors"
          >
            {loading ? "Saving..." : isNew ? "Create" : "Save"}
          </button>
          <button
            type="button"
            onClick={() => navigate("/admin/stations")}
            className="px-5 py-2 bg-slate-800 text-slate-300 text-sm rounded-lg hover:bg-slate-700 transition-colors"
          >
            Cancel
          </button>
        </div>
      </form>
      )}
    </div>
  );
}
