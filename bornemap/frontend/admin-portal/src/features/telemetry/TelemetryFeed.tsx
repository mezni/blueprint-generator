import { useState, useRef, useEffect } from "react";

interface TelemetryEvent {
  id: string;
  station_id: string;
  event_type: string;
  severity: "info" | "warn" | "error" | "critical";
  timestamp: string;
  payload: string;
}

const MOCK_EVENTS: TelemetryEvent[] = [
  { id: "1", station_id: "CH-001", event_type: "OCPP_Heartbeat", severity: "info", timestamp: new Date().toISOString(), payload: '{"status":"Available"}' },
  { id: "2", station_id: "CH-002", event_type: "OCPP_TransactionStart", severity: "info", timestamp: new Date().toISOString(), payload: '{"connector":1,"tag":"RFID-XXX"}' },
  { id: "3", station_id: "CH-003", event_type: "OCPP_InternalError", severity: "critical", timestamp: new Date().toISOString(), payload: '{"error":"ConnectorLockFailure","vendorCode":"EVSE-03"}' },
  { id: "4", station_id: "CH-001", event_type: "OCPP_MeterValues", severity: "info", timestamp: new Date().toISOString(), payload: '{"energy":45.2,"power":7.4}' },
  { id: "5", station_id: "CH-004", event_type: "OCPP_HighTemperature", severity: "warn", timestamp: new Date().toISOString(), payload: '{"temp":62,"threshold":60}' },
];

const severityStyles: Record<string, string> = {
  critical: "bg-red-500/20 text-red-400 border-red-500/30",
  error: "bg-orange-500/20 text-orange-400 border-orange-500/30",
  warn: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  info: "bg-slate-500/20 text-slate-400 border-slate-500/30",
};

export function TelemetryFeed() {
  const [events, setEvents] = useState<TelemetryEvent[]>(MOCK_EVENTS);
  const [paused, setPaused] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!paused) {
      bottomRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [events, paused]);

  return (
    <div className="h-full rounded-xl border border-slate-800 bg-slate-900 overflow-hidden flex flex-col">
      <div className="flex items-center justify-between px-4 py-3 border-b border-slate-800">
        <span className="text-sm font-medium">OCPP Telemetry Log</span>
        <button
          onClick={() => setPaused(!paused)}
          className={`text-xs px-2.5 py-1 rounded font-medium transition-colors ${
            paused
              ? "bg-green-500/20 text-green-400"
              : "bg-slate-800 text-slate-400 hover:text-slate-200"
          }`}
        >
          {paused ? "● Live" : "❚❚ Pause"}
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-3 font-mono text-[11px] space-y-1.5">
        {events.map((evt) => (
          <div
            key={evt.id}
            className={`flex items-start gap-2 p-2 rounded border ${severityStyles[evt.severity]}`}
          >
            <span className="shrink-0 w-14 text-right text-[10px] opacity-60">
              {new Date(evt.timestamp).toLocaleTimeString()}
            </span>
            <span className="shrink-0 font-bold">{evt.station_id}</span>
            <span className="shrink-0 text-[10px] uppercase tracking-wider opacity-70">
              {evt.event_type}
            </span>
            <span className="truncate text-[10px] opacity-50">{evt.payload}</span>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
