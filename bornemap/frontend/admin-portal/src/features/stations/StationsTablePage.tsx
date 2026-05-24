import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "../../lib/apiClient";
import { useNavigate } from "react-router-dom";

interface StationRow {
  id: string;
  name: string;
  address: string;
  company: { id: string; name: string };
  is_active: boolean;
  under_maintenance: boolean;
}

export function StationsTablePage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["admin-stations"],
    queryFn: () =>
      apiFetch<{ items: StationRow[]; next_cursor: string | null }>(
        "/api/v1/admin/stations?limit=50"
      ),
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) =>
      apiFetch(`/api/v1/admin/stations/${id}`, { method: "DELETE" }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["admin-stations"] }),
    onError: (err: Error) => {
      alert(err.message);
    },
  });

  return (
    <div className="h-full flex flex-col p-6 overflow-y-auto">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-lg font-bold">Stations</h1>
        <div className="flex gap-3">
          <button
            onClick={() => navigate("/map")}
            className="px-3 py-1.5 text-xs bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition-colors"
          >
            Map View
          </button>
          <button
            onClick={() => navigate("/admin/stations/new")}
            className="px-3 py-1.5 text-xs bg-blue-600 text-white rounded-lg hover:bg-blue-500 transition-colors"
          >
            Add Station
          </button>
        </div>
      </div>

      {isError && (
        <div className="bg-red-500/20 text-red-400 p-3 rounded-lg mb-4 text-xs border border-red-500/30">
          Failed to load stations: {(error as Error)?.message || "Unknown error"}
        </div>
      )}

      {isLoading ? (
        <p className="text-sm text-slate-500">Loading stations...</p>
      ) : (
        <div className="rounded-xl border border-slate-800 overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-slate-900 text-left text-xs text-slate-500 uppercase tracking-wider">
                <th className="p-3 font-medium">Name</th>
                <th className="p-3 font-medium">Address</th>
                <th className="p-3 font-medium">Company</th>
                <th className="p-3 font-medium">Active</th>
                <th className="p-3 font-medium">Maintenance</th>
                <th className="p-3 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {data?.items.map((s) => (
                <tr key={s.id} className="border-t border-slate-800 hover:bg-slate-800/40 transition-colors">
                  <td className="p-3 text-slate-200">{s.name}</td>
                  <td className="p-3 text-slate-400">{s.address}</td>
                  <td className="p-3 text-slate-400">{s.company.name}</td>
                  <td className="p-3">
                    <span
                      className={`px-2 py-0.5 rounded text-[11px] font-medium ${
                        s.is_active
                          ? "bg-green-500/20 text-green-400"
                          : "bg-red-500/20 text-red-400"
                      }`}
                    >
                      {s.is_active ? "Active" : "Inactive"}
                    </span>
                  </td>
                  <td className="p-3">
                    {s.under_maintenance && (
                      <span className="px-2 py-0.5 rounded text-[11px] font-medium bg-yellow-500/20 text-yellow-400">
                        Maintenance
                      </span>
                    )}
                  </td>
                  <td className="p-3">
                    <div className="flex gap-2">
                      <button
                        onClick={() => navigate(`/admin/stations/${s.id}/edit`)}
                        className="text-blue-400 hover:text-blue-300 text-xs transition-colors"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => deleteMutation.mutate(s.id)}
                        className="text-red-400 hover:text-red-300 text-xs transition-colors"
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
