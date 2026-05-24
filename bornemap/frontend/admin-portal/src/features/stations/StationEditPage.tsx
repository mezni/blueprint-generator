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
    <div className="min-h-screen bg-gray-50 p-6 max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">
        {isNew ? "New Station" : "Edit Station"}
      </h1>
      {error && (
        <div className="bg-red-50 text-red-700 p-3 rounded mb-4 text-sm">
          {error}
        </div>
      )}
      {fetching ? (
        <p className="text-gray-500">Loading station data...</p>
      ) : (
      <form onSubmit={handleSubmit} className="space-y-4 bg-white p-6 rounded shadow">
        <div>
          <label className="block text-sm font-medium mb-1">Name</label>
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full border rounded px-3 py-2"
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Address</label>
          <input
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            className="w-full border rounded px-3 py-2"
            required
          />
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Longitude</label>
            <input
              type="number"
              step="0.0001"
              value={lng}
              onChange={(e) => setLng(e.target.value)}
              className="w-full border rounded px-3 py-2"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Latitude</label>
            <input
              type="number"
              step="0.0001"
              value={lat}
              onChange={(e) => setLat(e.target.value)}
              className="w-full border rounded px-3 py-2"
              required
            />
          </div>
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Opening Hours (OSM format)</label>
          <input
            value={openingHours}
            onChange={(e) => setOpeningHours(e.target.value)}
            placeholder="e.g. Mo-Fr 08:00-20:00; Sa 09:00-13:00"
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div className="flex items-center gap-4">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={isActive}
              onChange={(e) => setIsActive(e.target.checked)}
            />
            Active
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={underMaintenance}
              onChange={(e) => setUnderMaintenance(e.target.checked)}
            />
            Under Maintenance
          </label>
        </div>
        <div className="flex gap-3">
          <button
            type="submit"
            disabled={loading}
            className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          >
            {loading ? "Saving..." : isNew ? "Create" : "Save"}
          </button>
          <button
            type="button"
            onClick={() => navigate("/admin/stations")}
            className="px-6 py-2 bg-gray-200 rounded hover:bg-gray-300"
          >
            Cancel
          </button>
        </div>
      </form>
      )}
    </div>
  );
}
