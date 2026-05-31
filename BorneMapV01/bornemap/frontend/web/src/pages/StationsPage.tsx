import { useState } from "react";
import DataTable from "../components/ui/DataTable";
import StationDialog from "../features/stations/StationDialog";
import { useStations, useCreateStation, useUpdateStation, useToggleStationActive, useDeleteStation } from "../features/stations/api";
import type { Station, StationCreate, StationUpdate } from "../features/stations/types";

export default function StationsPage() {
  const { data: stations, isLoading } = useStations();
  const create = useCreateStation();
  const update = useUpdateStation();
  const toggle = useToggleStationActive();
  const remove = useDeleteStation();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editing, setEditing] = useState<Station | null>(null);

  const columns = [
    { key: "name", header: "Name", render: (s: Station) => <span className="font-medium text-slate-900">{s.name}</span> },
    { key: "operator", header: "Operator", render: (s: Station) => <span className="text-slate-600">{s.operator ?? "—"}</span> },
    { key: "partner_id", header: "Partner ID", render: (s: Station) => <span className="text-slate-600">{s.partner_id ?? "—"}</span> },
    { key: "plug_types", header: "Plugs", render: (s: Station) => (
      <div className="flex flex-wrap gap-1">{s.plug_types?.length ? s.plug_types.map((p) => <span key={p} className="rounded-md bg-accent-muted px-1.5 py-0.5 text-[11px] font-medium text-accent-dark">{p}</span>) : <span className="text-slate-400">—</span>}</div>
    )},
    { key: "speed_kw", header: "Speed", render: (s: Station) => <span className="text-slate-600">{s.speed_kw ? `${s.speed_kw} kW` : "—"}</span> },
    { key: "is_active", header: "Status", render: (s: Station) => (
      <span className={`inline-flex items-center gap-1 text-xs font-medium ${s.is_active ? "text-green-600" : "text-slate-400"}`}>
        <span className={`h-1.5 w-1.5 rounded-full ${s.is_active ? "bg-green-500" : "bg-slate-300"}`} />
        {s.is_active ? "Active" : "Inactive"}
      </span>
    )},
  ];

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-6">
        <h1 className="text-lg font-semibold text-slate-900">Stations</h1>
        <p className="text-sm text-muted">Manage EV charging stations</p>
      </div>
      {isLoading ? (
        <div className="flex items-center justify-center py-12"><div className="h-6 w-6 animate-spin rounded-full border-2 border-accent border-t-transparent" /></div>
      ) : (
        <div className="rounded-2xl border border-border bg-white p-6 shadow-card">
          <DataTable data={stations ?? []} columns={columns} onEdit={(s) => { setEditing(s); setDialogOpen(true); }}
            onToggleActive={(s) => toggle.mutate(s.id)} onDelete={(s) => { if (window.confirm(`Delete station "${s.name}"?`)) remove.mutate(s.id); }}
            onAdd={() => { setEditing(null); setDialogOpen(true); }} title="All Stations" addLabel="Add Station" />
        </div>
      )}
      <StationDialog open={dialogOpen} onClose={() => setDialogOpen(false)}
        onSave={(data) => { editing ? update.mutate({ id: editing.id, data: data as StationUpdate }, { onSuccess: () => setDialogOpen(false) }) : create.mutate(data as StationCreate, { onSuccess: () => setDialogOpen(false) }); }}
        station={editing} saving={create.isPending || update.isPending} />
    </div>
  );
}
