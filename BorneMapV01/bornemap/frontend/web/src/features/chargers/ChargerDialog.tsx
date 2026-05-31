import { useState } from "react";
import { X } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Charger, ChargerCreate, ChargerUpdate } from "./types";

interface Props {
  open: boolean;
  onClose: () => void;
  onSave: (data: ChargerCreate | ChargerUpdate) => void;
  charger?: Charger | null;
  saving?: boolean;
}

const connectors = ["CCS", "Type2", "CHAdeMO", "GBT"];
const statuses = ["available", "occupied", "offline", "maintenance"];

export default function ChargerDialog({ open, onClose, onSave, charger, saving }: Props) {
  const [station_id, setStation] = useState(charger?.station_id?.toString() ?? "");
  const [connector, setConnector] = useState(charger?.connector ?? "CCS");
  const [power_kw, setPower] = useState(charger?.power_kw?.toString() ?? "");
  const [status, setStatus] = useState(charger?.status ?? "available");

  if (!open) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const data: Record<string, unknown> = { connector, status };
    if (station_id) data.station_id = Number(station_id);
    if (power_kw) data.power_kw = Number(power_kw);
    if (charger) {
      const upd: ChargerUpdate = {};
      if (station_id !== (charger.station_id?.toString() ?? "")) upd.station_id = Number(station_id) || undefined;
      if (connector !== charger.connector) upd.connector = connector;
      if (power_kw !== (charger.power_kw?.toString() ?? "")) upd.power_kw = Number(power_kw) || undefined;
      if (status !== charger.status) upd.status = status;
      onSave(upd);
    } else {
      onSave(data as ChargerCreate);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="w-full max-w-lg rounded-2xl border border-border bg-white shadow-float">
        <div className="flex items-center justify-between border-b border-border px-6 py-4">
          <h2 className="text-sm font-semibold text-slate-900">{charger ? "Edit Charger" : "Add Charger"}</h2>
          <button onClick={onClose} className="rounded-lg p-1 hover:bg-slate-100"><X size={16} className="text-slate-500" /></button>
        </div>
        <form onSubmit={handleSubmit} className="space-y-4 px-6 py-4">
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Station ID</label>
            <input type="number" value={station_id} onChange={(e) => setStation(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Connector</label>
            <select value={connector} onChange={(e) => setConnector(e.target.value)} className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20">
              {connectors.map((c) => <option key={c} value={c}>{c}</option>)}
            </select>
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Power (kW)</label>
            <input type="number" step="0.1" value={power_kw} onChange={(e) => setPower(e.target.value)} className="w-full rounded-lg border border-border px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20" />
          </div>
          <div>
            <label className="mb-1 block text-xs font-medium text-slate-700">Status</label>
            <select value={status} onChange={(e) => setStatus(e.target.value)} className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20">
              {statuses.map((s) => <option key={s} value={s}>{s.charAt(0).toUpperCase() + s.slice(1)}</option>)}
            </select>
          </div>
          <div className="flex items-center justify-end gap-3 border-t border-border pt-4">
            <Button type="button" variant="outline" size="sm" onClick={onClose}>Cancel</Button>
            <Button type="submit" size="sm" disabled={saving}>{saving ? "Saving..." : charger ? "Update" : "Create"}</Button>
          </div>
        </form>
      </div>
    </div>
  );
}
