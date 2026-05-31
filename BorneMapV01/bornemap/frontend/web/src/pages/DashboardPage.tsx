import { useNavigate } from "react-router-dom";
import { Settings, Users, MapPin, Activity } from "lucide-react";
import { Button } from "@/components/ui/button";

const stats = [
  { icon: MapPin, label: "Total Stations", value: "--" },
  { icon: Users, label: "Active Users", value: "--" },
  { icon: Activity, label: "System Health", value: "Operational" },
];

export default function DashboardPage() {
  const navigate = useNavigate();

  return (
    <div className="mx-auto max-w-4xl p-6">
      <div className="mb-6">
        <h1 className="text-lg font-semibold text-slate-900">Dashboard</h1>
        <p className="text-sm text-muted">Overview of your BorneMap instance</p>
      </div>

      <div className="mb-6 grid grid-cols-3 gap-4">
        {stats.map((stat) => (
          <div
            key={stat.label}
            className="rounded-2xl border border-border bg-white p-5 shadow-card"
          >
            <div className="mb-3 flex h-10 w-10 items-center justify-center rounded-xl bg-accent-muted">
              <stat.icon size={20} className="text-accent-dark" />
            </div>
            <p className="text-2xl font-semibold text-slate-900">{stat.value}</p>
            <p className="mt-1 text-xs text-muted">{stat.label}</p>
          </div>
        ))}
      </div>

      <div className="rounded-2xl border border-border bg-white p-6 shadow-card">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-sm font-semibold text-slate-900">Application Settings</h2>
            <p className="mt-1 text-xs text-muted">
              Configure application preferences, user management, and more
            </p>
          </div>
          <Button
            variant="default"
            size="sm"
            onClick={() => navigate("/settings")}
            className="flex items-center gap-1.5"
          >
            <Settings size={14} />
            Settings
          </Button>
        </div>
      </div>
    </div>
  );
}
