interface QueueItem {
  id: string;
  priority: "P1" | "P2" | "P3" | "P4" | "P5";
  station_id: string;
  fault_code: string;
  status: "open" | "acknowledged" | "in_progress" | "resolved";
  created_at: string;
  assigned_to: string | null;
}

const MOCK_QUEUE: QueueItem[] = [
  { id: "Q1", priority: "P1", station_id: "CH-003", fault_code: "OCPP_InternalError", status: "open", created_at: new Date().toISOString(), assigned_to: null },
  { id: "Q2", priority: "P2", station_id: "CH-004", fault_code: "OCPP_HighTemperature", status: "acknowledged", created_at: new Date(Date.now() - 1800000).toISOString(), assigned_to: "ops-1" },
  { id: "Q3", priority: "P3", station_id: "CH-001", fault_code: "OCPP_ConnectorLockFailure", status: "in_progress", created_at: new Date(Date.now() - 3600000).toISOString(), assigned_to: "eng-2" },
  { id: "Q4", priority: "P5", station_id: "CH-005", fault_code: "OCPP_MeterReadError", status: "open", created_at: new Date(Date.now() - 7200000).toISOString(), assigned_to: null },
];

const priorityStyles: Record<string, string> = {
  P1: "bg-red-500/20 text-red-400",
  P2: "bg-orange-500/20 text-orange-400",
  P3: "bg-yellow-500/20 text-yellow-400",
  P4: "bg-blue-500/20 text-blue-400",
  P5: "bg-slate-500/20 text-slate-400",
};

const statusStyles: Record<string, string> = {
  open: "bg-red-500/10 text-red-400",
  acknowledged: "bg-yellow-500/10 text-yellow-400",
  in_progress: "bg-blue-500/10 text-blue-400",
  resolved: "bg-green-500/10 text-green-400",
};

function timeAgo(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60000);
  if (mins < 60) return `${mins}m`;
  const hrs = Math.floor(mins / 60);
  return `${hrs}h ${mins % 60}m`;
}

export function FaultQueue() {
  return (
    <div className="h-full rounded-xl border border-slate-800 bg-slate-900 overflow-hidden flex flex-col">
      <div className="flex items-center justify-between px-4 py-3 border-b border-slate-800">
        <span className="text-sm font-medium">Active Fault Remediation Queue</span>
        <span className="text-xs text-slate-500 bg-slate-800 px-2 py-1 rounded">
          {MOCK_QUEUE.filter((q) => q.status !== "resolved").length} unresolved
        </span>
      </div>
      <div className="flex-1 overflow-y-auto">
        <table className="w-full text-xs">
          <thead className="text-slate-500 text-[10px] uppercase tracking-wider bg-slate-950/50 sticky top-0">
            <tr>
              <th className="text-left px-4 py-2.5 font-medium">Priority</th>
              <th className="text-left px-4 py-2.5 font-medium">Station</th>
              <th className="text-left px-4 py-2.5 font-medium">Fault Code</th>
              <th className="text-left px-4 py-2.5 font-medium">Status</th>
              <th className="text-left px-4 py-2.5 font-medium">Age</th>
              <th className="text-left px-4 py-2.5 font-medium">Assigned</th>
              <th className="text-left px-4 py-2.5 font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {MOCK_QUEUE.map((item) => (
              <tr key={item.id} className="border-t border-slate-800/50 hover:bg-slate-800/30 transition-colors">
                <td className="px-4 py-3">
                  <span className={`px-2 py-0.5 rounded text-[10px] font-bold ${priorityStyles[item.priority]}`}>
                    {item.priority}
                  </span>
                </td>
                <td className="px-4 py-3 font-mono text-slate-300">{item.station_id}</td>
                <td className="px-4 py-3 font-mono text-slate-400">{item.fault_code}</td>
                <td className="px-4 py-3">
                  <span className={`px-2 py-0.5 rounded text-[10px] font-medium ${statusStyles[item.status]}`}>
                    {item.status.replace("_", " ")}
                  </span>
                </td>
                <td className="px-4 py-3 text-slate-500">{timeAgo(item.created_at)}</td>
                <td className="px-4 py-3 text-slate-500">{item.assigned_to || "—"}</td>
                <td className="px-4 py-3">
                  <button className="text-blue-400 hover:text-blue-300 text-[10px] font-medium transition-colors">
                    Acknowledge
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
