import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Settings, Moon, Sun, Globe, Bell, MapPin } from "lucide-react";
import { useState } from "react";

const languages = [
  { code: "en", label: "English" },
  { code: "fr", label: "Français" },
  { code: "ar", label: "العربية" },
];

export default function GeneralSettingsCard() {
  const [isDark, setIsDark] = useState(false);
  const [language, setLanguage] = useState("en");
  const [notifications, setNotifications] = useState(true);

  return (
    <Card>
      <CardHeader className="pb-4">
        <div className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted">
            <Settings size={16} className="text-accent-dark" />
          </div>
          <div>
            <h2 className="text-sm font-semibold text-accent">General Settings</h2>
            <p className="text-xs text-muted">Second configuration section</p>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-5">
          <div className="flex items-center justify-between rounded-lg bg-surface-card px-4 py-3">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted/50">
                {isDark ? (
                  <Moon size={16} className="text-accent-dark" />
                ) : (
                  <Sun size={16} className="text-accent-dark" />
                )}
              </div>
              <div>
                <p className="text-sm font-medium text-slate-900">Theme</p>
                <p className="text-xs text-muted">Toggle dark mode</p>
              </div>
            </div>
            <Button
              variant={isDark ? "default" : "outline"}
              size="sm"
              onClick={() => setIsDark(!isDark)}
            >
              {isDark ? "Dark" : "Light"}
            </Button>
          </div>

          <div className="flex items-center justify-between rounded-lg bg-surface-card px-4 py-3">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted/50">
                <Globe size={16} className="text-accent-dark" />
              </div>
              <div>
                <p className="text-sm font-medium text-slate-900">Language</p>
                <p className="text-xs text-muted">Interface language</p>
              </div>
            </div>
            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="rounded-lg border border-border bg-background px-3 py-2 text-sm text-slate-700 focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20"
            >
              {languages.map((lang) => (
                <option key={lang.code} value={lang.code}>
                  {lang.label}
                </option>
              ))}
            </select>
          </div>

          <div className="flex items-center justify-between rounded-lg bg-surface-card px-4 py-3">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted/50">
                <Bell size={16} className="text-accent-dark" />
              </div>
              <div>
                <p className="text-sm font-medium text-slate-900">Notifications</p>
                <p className="text-xs text-muted">Push and email alerts</p>
              </div>
            </div>
            <Button
              variant={notifications ? "default" : "outline"}
              size="sm"
              onClick={() => setNotifications(!notifications)}
            >
              {notifications ? "On" : "Off"}
            </Button>
          </div>

          <div className="flex items-center justify-between rounded-lg bg-surface-card px-4 py-3">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-muted/50">
                <MapPin size={16} className="text-accent-dark" />
              </div>
              <div>
                <p className="text-sm font-medium text-slate-900">Map Defaults</p>
                <p className="text-xs text-muted">Center: Tunisia (33.8869, 9.5375)</p>
              </div>
            </div>
            <Button variant="outline" size="sm">
              Reset
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
