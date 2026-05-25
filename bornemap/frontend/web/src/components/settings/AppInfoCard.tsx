import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Info, Server, Globe, Database, Activity } from "lucide-react";

const infoItems = [
  { icon: Server, label: "Application", value: "BorneMap" },
  { icon: Info, label: "Version", value: "0.1.0" },
  { icon: Globe, label: "Environment", value: "Development" },
  { icon: Database, label: "Database", value: "Connected" },
  { icon: Activity, label: "API Status", value: "Operational" },
];

export default function AppInfoCard() {
  return (
    <Card>
      <CardHeader className="pb-4">
        <div className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted">
            <Info size={16} className="text-accent-dark" />
          </div>
          <div>
            <h2 className="text-sm font-semibold text-accent">App Info</h2>
            <p className="text-xs text-muted">First configuration section</p>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-3">
          {infoItems.map((item) => (
            <div
              key={item.label}
              className="flex items-center justify-between rounded-lg bg-surface-card px-4 py-3"
            >
              <div className="flex items-center gap-3">
                <item.icon size={16} className="text-muted" />
                <span className="text-sm text-slate-700">{item.label}</span>
              </div>
              <span className="text-sm font-medium text-slate-900">
                {item.value}
              </span>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
