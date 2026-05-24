import { useNavigate, useLocation } from "react-router-dom";
import { Eye, LayoutDashboard } from "lucide-react";
import { cn } from "../../lib/utils";

const tabs = [
  { label: "Preview", icon: Eye, path: "/preview" },
  { label: "Dashboard", icon: LayoutDashboard, path: "/" },
];

export default function Header() {
  const navigate = useNavigate();
  const location = useLocation();
  const currentPath = location.pathname;

  return (
    <header className="flex h-14 items-center justify-between border-b border-border bg-white px-6">
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-2">
          <div className="flex h-7 w-7 items-center justify-center rounded-md bg-accent">
            <span className="text-xs font-bold text-white">BM</span>
          </div>
          <span className="text-sm font-semibold text-slate-900">BorneMap</span>
        </div>
      </div>

      <div className="flex items-center gap-1 rounded-lg bg-slate-100 p-0.5">
        {tabs.map((tab) => {
          const isActive =
            tab.path === "/"
              ? currentPath === "/" || currentPath === ""
              : currentPath.startsWith(tab.path);

          return (
            <button
              key={tab.label}
              onClick={() => navigate(tab.path)}
              className={cn(
                "flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-all duration-150",
                isActive
                  ? "bg-white text-slate-900 shadow-sm"
                  : "text-slate-500 hover:text-slate-700",
              )}
            >
              <tab.icon size={14} />
              {tab.label}
            </button>
          );
        })}
      </div>

      <div className="flex items-center gap-3">
        <div className="flex h-7 w-7 items-center justify-center rounded-full bg-accent-muted">
          <span className="text-[10px] font-bold text-accent-dark">A</span>
        </div>
      </div>
    </header>
  );
}
