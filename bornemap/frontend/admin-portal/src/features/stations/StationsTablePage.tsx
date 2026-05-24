import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "../../lib/apiClient";
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "../auth/authStore";

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
  const logout = useAuthStore((s) => s.logout);
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
    <div className="min-h-screen bg-gray-50 p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Stations</h1>
        <div className="flex gap-3">
          <button
            onClick={() => navigate("/map")}
            className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
          >
            Map View
          </button>
          <button
            onClick={() => navigate("/admin/stations/new")}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Add Station
          </button>
          <button
            onClick={() => {
              logout();
              navigate("/login");
            }}
            className="px-4 py-2 bg-red-100 text-red-700 rounded hover:bg-red-200"
          >
            Logout
          </button>
        </div>
      </div>

      {isError && (
        <div className="bg-red-50 text-red-700 p-3 rounded mb-4 text-sm">
          Failed to load stations: {(error as Error)?.message || "Unknown error"}
        </div>
      )}

      {isLoading ? (
        <p>Loading stations...</p>
      ) : (
        <table className="w-full bg-white rounded shadow">
          <thead>
            <tr className="bg-gray-100 text-left text-sm">
              <th className="p-3">Name</th>
              <th className="p-3">Address</th>
              <th className="p-3">Company</th>
              <th className="p-3">Active</th>
              <th className="p-3">Maintenance</th>
              <th className="p-3">Actions</th>
            </tr>
          </thead>
          <tbody>
            {data?.items.map((s) => (
              <tr key={s.id} className="border-t hover:bg-gray-50">
                <td className="p-3">{s.name}</td>
                <td className="p-3">{s.address}</td>
                <td className="p-3">{s.company.name}</td>
                <td className="p-3">
                  <span
                    className={`px-2 py-1 rounded text-xs ${
                      s.is_active
                        ? "bg-green-100 text-green-700"
                        : "bg-red-100 text-red-700"
                    }`}
                  >
                    {s.is_active ? "Active" : "Inactive"}
                  </span>
                </td>
                <td className="p-3">
                  {s.under_maintenance && (
                    <span className="px-2 py-1 rounded text-xs bg-yellow-100 text-yellow-700">
                      Maintenance
                    </span>
                  )}
                </td>
                <td className="p-3 flex gap-2">
                  <button
                    onClick={() => navigate(`/admin/stations/${s.id}/edit`)}
                    className="text-blue-600 hover:underline text-sm"
                  >
                    Edit
                  </button>
                  <button
                    onClick={() => deleteMutation.mutate(s.id)}
                    className="text-red-600 hover:underline text-sm"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
