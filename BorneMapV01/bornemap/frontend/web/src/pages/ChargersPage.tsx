import { useState } from "react";
import DataTable from "../components/ui/DataTable";
import ChargerDialog from "../features/chargers/ChargerDialog";
import { useChargers, useCreateCharger, useUpdateCharger, useToggleChargerActive, useDeleteCharger } from "../features/chargers/api";
import type { Charger, ChargerCreate, ChargerUpdate } from "../features/chargers/types";

const statusColors: Record<string, string> = {
  available: "bg-green-100 text-green-700",
  occupied: "bg-amber-100 text-amber-700",
  offline: "bg-red-100 text-red-700",
  maintenance: "bg-blue-100 text-blue-700",
};

export default function ChargersPage() {
  const { data: chargers, isLoading } = useChargers();
  const create = useCreateCharger();
  const update = useUpdateCharger();
  const toggle = useToggleChargerActive();
  const remove = useDeleteCharger();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editing, setEditing] = useState<Charger | null>(null);

  const columns = [
    { key: "connector", header: "Connector", render: (c: Charger) => <span className="font-medium text-slate-900">{c.connector}</span> },
    { key: "station_id", header: "Station ID", render: (c: Charger) => <span className="text-slate-600">{c.station_id ?? "—"}</span> },
    { key: "power_kw", header: "Power", render: (c: Charger) => <span className="text-slate-600">{c.power_kw ? `${c.power_kw} kW` : "—"}</span> },
    { key: "status", header: "Status", render: (c: Charger) => (
      <span className={`inline-block rounded-full px-2 py-0.5 text-[11px] font-medium ${statusColors[c.status] ?? "bg-slate-100 text-slate-700"}`}>
        {c.status.charAt(0).toUpperCase() + c.status.slice(1)}
      </span>
    )},
    { key: "is_active", header: "Active", render: (c: Charger) => (
      <span className={`inline-flex items-center gap-1 text-xs font-medium ${c.is_active ? "text-green-600" : "text-slate-400"}`}>
        <span className={`h-1.5 w-1.5 rounded-full ${c.is_active ? "bg-green-500" : "bg-slate-300"}`} />
        {c.is_active ? "Yes" : "No"}
      </span>
    )},
  ];

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-6">
        <h1 className="text-lg font-semibold text-slate-900">Chargers</h1>
        <p className="text-sm text-muted">Manage individual charger connectors</p>
      </div>
      {isLoading ? (
        <div className="flex items-center justify-center py-12"><div className="h-6 w-6 animate-spin rounded-full border-2 border-accent border-t-transparent" /></div>
      ) : (
        <div className="rounded-2xl border border-border bg-white p-6 shadow-card">
          <DataTable data={chargers ?? []} columns={columns} onEdit={(c) => { setEditing(c); setDialogOpen(true); }}
            onToggleActive={(c) => toggle.mutate(c.id)} onDelete={(c) => { if (window.confirm(`Delete charger #${c.id}?`)) remove.mutate(c.id); }}
            onAdd={() => { setEditing(null); setDialogOpen(true); }} title="All Chargers" addLabel="Add Charger" />
        </div>
      )}
      <ChargerDialog open={dialogOpen} onClose={() => setDialogOpen(false)}
        onSave={(data) => { editing ? update.mutate({ id: editing.id, data: data as ChargerUpdate }, { onSuccess: () => setDialogOpen(false) }) : create.mutate(data as ChargerCreate, { onSuccess: () => setDialogOpen(false) }); }}
        charger={editing} saving={create.isPending || update.isPending} />
    </div>
  );
}
