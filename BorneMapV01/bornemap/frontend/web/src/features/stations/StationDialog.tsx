import { useState } from "react";
import { X } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Station, StationCreate, StationUpdate } from "./types";

interface Props {
  open: boolean;
  onClose: () => void;
  onSave: (data: StationCreate | StationUpdate) => void;
  station?: Station | null;
  saving?: boolean;
}

const plugOptions = ["CCS", "Type2", "CHAdeMO", "GBT"];

export default function StationDialog({ open, onClose, onSave, station, saving }: Props) {
  const [name, setName] = useState(station?.name ?? "");
  const [operator, setOperator] = useState(station?.operator ?? "");
  const [address, setAddress] = useState(station?.address ?? "");
  const [partner_id, setPartner] = useState(station?.partner_id?.toString() ?? "");
  const [speed_kw, setSpeed] = useState(station?.speed_kw?.toString() ?? "");
  const [plug_types, setPlugTypes] = useState<string[]>(station?.plug_types ?? []);

  if (!open) return null;

  const togglePlug = (plug: string) => {
    setPlugTypes((prev) => prev.includes(plug) ? prev.filter((p) => p !== plug) : [...prev, plug]);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const base = { name, operator: operator || undefined, address: address || undefined, plug_types: plug_types.length ? plug_types : undefined, speed_kw: speed_kw ? Number(speed_kw) : undefined };
    if (station) {
      const upd: StationUpdate = {};
      if (name !== station.name) upd.name = name;
      if (operator !== (station.operator ?? "")) upd.operator = operator || undefined;
      if (address !== (station.address ?? "")) upd.address = address || undefined;
      if (partner_id !== (station.partner_id?.toString() ?? "")) upd.partner_id = Number(partner_id) || undefined;
      if (speed_kw !== (station.speed_kw?.toString() ?? "")) upd.speed_kw = Number(speed_kw) || undefined;
      if (JSON.stringify(plug_types) !== JSON.stringify(station.plug_types)) upd.plug_types = plug_types.length ? plug_types : undefined;
      onSave(upd);
    } else {
      onSave({ ...base, partner_id: partner_id ? Number(partner_id) : undefined } as StationCreate);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="w-full max-w-lg rounded-2xl border border-border bg-white shadow-float">
        <div className="flex items-center justify-between border-b border-border px-6 py-4">
          <h2 className="text-sm font-semibold text-slate-900">{station ? "Edit Station" : "Add Station"}</h2>
          <button onClick={onClose} className="rounded-lg p-1 hover:bg-slate-100"><X size={16} className="text-slate-500" /></button>
        </div>
        <form onSubmit={handleSubmit} className="space-y-4 px-6 py-4">
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Name</label>
            <input type="text" value={name} onChange={(e) => setName(e.target.value)} required className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Partner ID</label>
            <input type="number" value={partner_id} onChange={(e) => setPartner(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Operator</label>
            <input type="text" value={operator} onChange={(e) => setOperator(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Address</label>
            <input type="text" value={address} onChange={(e) => setAddress(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Speed (kW)</label>
            <input type="number" step="0.1" value={speed_kw} onChange={(e) => setSpeed(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Plug Types</label>
            <div className="flex flex-wrap gap-2">
              {plugOptions.map((plug) => (
                <button key={plug} type="button" onClick={() => togglePlug(plug)}
                  className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${plug_types.includes(plug) ? "bg-accent text-white" : "bg-slate-100 text-slate-600 hover:bg-slate-200"}`}>
                  {plug}
                </button>
              ))}
            </div>
          </div>
          <div className="flex items-center justify-end gap-3 border-t border-border pt-4">
            <Button type="button" variant="outline" size="sm" onClick={onClose}>Cancel</Button>
            <Button type="submit" size="sm" disabled={saving}>{saving ? "Saving..." : station ? "Update" : "Create"}</Button>
          </div>
        </form>
      </div>
    </div>
  );
}
