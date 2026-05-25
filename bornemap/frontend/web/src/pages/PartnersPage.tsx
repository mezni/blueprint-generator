import { useState } from "react";
import DataTable from "../components/ui/DataTable";
import PartnerDialog from "../features/partners/PartnerDialog";
import { usePartners, useCreatePartner, useUpdatePartner, useTogglePartnerActive, useDeletePartner } from "../features/partners/api";
import type { Partner, PartnerCreate, PartnerUpdate } from "../features/partners/types";

export default function PartnersPage() {
  const { data: partners, isLoading } = usePartners();
  const create = useCreatePartner();
  const update = useUpdatePartner();
  const toggle = useTogglePartnerActive();
  const remove = useDeletePartner();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editing, setEditing] = useState<Partner | null>(null);

  const columns = [
    { key: "name", header: "Name", render: (p: Partner) => <span className="font-medium text-slate-900">{p.name}</span> },
    { key: "contact_email", header: "Email", render: (p: Partner) => <span className="text-slate-600">{p.contact_email ?? "—"}</span> },
    { key: "phone", header: "Phone", render: (p: Partner) => <span className="text-slate-600">{p.phone ?? "—"}</span> },
    { key: "is_active", header: "Status", render: (p: Partner) => (
      <span className={`inline-flex items-center gap-1 text-xs font-medium ${p.is_active ? "text-green-600" : "text-slate-400"}`}>
        <span className={`h-1.5 w-1.5 rounded-full ${p.is_active ? "bg-green-500" : "bg-slate-300"}`} />
        {p.is_active ? "Active" : "Inactive"}
      </span>
    )},
  ];

  return (
    <div className="mx-auto max-w-4xl p-6">
      <div className="mb-6">
        <h1 className="text-lg font-semibold text-slate-900">Partners</h1>
        <p className="text-sm text-muted">Manage partner organizations</p>
      </div>
      {isLoading ? (
        <div className="flex items-center justify-center py-12"><div className="h-6 w-6 animate-spin rounded-full border-2 border-accent border-t-transparent" /></div>
      ) : (
        <div className="rounded-2xl border border-border bg-white p-6 shadow-card">
          <DataTable data={partners ?? []} columns={columns} onEdit={(p) => { setEditing(p); setDialogOpen(true); }}
            onToggleActive={(p) => toggle.mutate(p.id)} onDelete={(p) => { if (window.confirm(`Delete "${p.name}"?`)) remove.mutate(p.id); }}
            onAdd={() => { setEditing(null); setDialogOpen(true); }} title="All Partners" addLabel="Add Partner" />
        </div>
      )}
      <PartnerDialog open={dialogOpen} onClose={() => setDialogOpen(false)}
        onSave={(data) => { const mutate = editing ? update.mutate({ id: editing.id, data: data as PartnerUpdate }, { onSuccess: () => setDialogOpen(false) }) : create.mutate(data as PartnerCreate, { onSuccess: () => setDialogOpen(false) }); }}
        partner={editing} saving={create.isPending || update.isPending} />
    </div>
  );
}
