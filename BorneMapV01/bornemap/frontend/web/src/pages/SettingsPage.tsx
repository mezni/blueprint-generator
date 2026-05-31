import AppInfoCard from "../components/settings/AppInfoCard";
import GeneralSettingsCard from "../components/settings/GeneralSettingsCard";

export default function SettingsPage() {
  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div>
        <h1 className="text-lg font-semibold text-slate-900">Settings</h1>
        <p className="text-sm text-muted">Manage application configuration</p>
      </div>
      <AppInfoCard />
      <GeneralSettingsCard />
    </div>
  );
}
