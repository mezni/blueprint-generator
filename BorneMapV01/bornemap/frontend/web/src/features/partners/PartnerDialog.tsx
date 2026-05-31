import { useState } from "react";
import { X } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Partner, PartnerCreate, PartnerUpdate } from "./types";

interface Props {
  open: boolean;
  onClose: () => void;
  onSave: (data: PartnerCreate | PartnerUpdate) => void;
  partner?: Partner | null;
  saving?: boolean;
}

export default function PartnerDialog({ open, onClose, onSave, partner, saving }: Props) {
  const [name, setName] = useState(partner?.name ?? "");
  const [contact_email, setEmail] = useState(partner?.contact_email ?? "");
  const [phone, setPhone] = useState(partner?.phone ?? "");

  if (!open) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (partner) {
      const data: PartnerUpdate = {};
      if (name !== partner.name) data.name = name;
      if (contact_email !== (partner.contact_email ?? "")) data.contact_email = contact_email || undefined;
      if (phone !== (partner.phone ?? "")) data.phone = phone || undefined;
      onSave(data);
    } else {
      onSave({ name, contact_email: contact_email || undefined, phone: phone || undefined });
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="w-full max-w-lg rounded-2xl border border-border bg-white shadow-float">
        <div className="flex items-center justify-between border-b border-border px-6 py-4">
          <h2 className="text-sm font-semibold text-slate-900">{partner ? "Edit Partner" : "Add Partner"}</h2>
          <button onClick={onClose} className="rounded-lg p-1 hover:bg-slate-100"><X size={16} className="text-slate-500" /></button>
        </div>
        <form onSubmit={handleSubmit} className="space-y-4 px-6 py-4">
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Name</label>
            <input type="text" value={name} onChange={(e) => setName(e.target.value)} required className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Contact Email</label>
            <input type="email" value={contact_email} onChange={(e) => setEmail(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Phone</label>
            <input type="text" value={phone} onChange={(e) => setPhone(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div className="flex items-center justify-end gap-3 border-t border-border pt-4">
            <Button type="button" variant="outline" size="sm" onClick={onClose}>Cancel</Button>
            <Button type="submit" size="sm" disabled={saving}>{saving ? "Saving..." : partner ? "Update" : "Create"}</Button>
          </div>
        </form>
      </div>
    </div>
  );
}
