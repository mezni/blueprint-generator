import { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import {
  Search,
  LayoutDashboard,
  Users,
  Database,
  BarChart3,
  TrendingUp,
  Navigation,
  Zap,
  Plug,
  Settings,
} from "lucide-react";
import NavItem from "./NavItem";
import NavAccordion from "./NavAccordion";

const mainNavItems = [
  { icon: LayoutDashboard, label: "Overview", path: "/", badge: undefined },
  { icon: Users, label: "Users", path: "/users", badge: "12" },
  { icon: Database, label: "Data", path: "/data", badge: undefined },
  { icon: BarChart3, label: "Analytics", path: "/analytics", badge: undefined },
  { icon: TrendingUp, label: "Growth", path: "/growth", badge: undefined },
  { icon: Settings, label: "Settings", path: "/settings", badge: undefined },
];

const dataPaths = ["/data/partners", "/data/stations", "/data/chargers"];

const dataNavItems = [
  { icon: Navigation, label: "Partners", active: false },
  { icon: Zap, label: "Stations", active: true },
  { icon: Plug, label: "Chargers", active: false },
];

export default function Sidebar() {
  const [search, setSearch] = useState("");
  const location = useLocation();
  const navigate = useNavigate();
  const currentPath = location.pathname;

  return (
    <aside className="flex h-full w-64 flex-col border-r border-border bg-sidebar-bg">
      <div className="border-b border-border px-4 py-3.5">
        <div className="relative">
          <Search
            size={16}
            className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-sidebar-text"
          />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search..."
            className="w-full rounded-lg border border-border bg-white py-2 pl-9 pr-3 text-sm placeholder:text-sidebar-text/50 focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent/20"
          />
        </div>
      </div>

      <nav className="flex-1 space-y-1 overflow-y-auto px-3 py-4">
        {mainNavItems.map((item) => {
          const isActive = item.path === "/" && currentPath === "/"
            ? true
            : item.path !== "/" && currentPath.startsWith(item.path);

          if (item.label === "Data") {
            return (
              <NavAccordion
                key={item.label}
                icon={item.icon}
                label={item.label}
                defaultOpen={true}
                active={isActive}
                items={dataNavItems.map((ni, idx) => ({
                  ...ni,
                  active: currentPath === dataPaths[idx],
                  onClick: () => navigate(dataPaths[idx]),
                }))}
              />
            );
          }
          return (
            <NavItem
              key={item.label}
              icon={item.icon}
              label={item.label}
              active={isActive}
              badge={item.badge}
              onClick={() => navigate(item.path)}
            />
          );
        })}
      </nav>

      <div className="border-t border-border px-4 py-3">
        <div className="flex items-center gap-3 rounded-lg px-3 py-2 text-xs text-sidebar-text">
          <div className="h-6 w-6 rounded-full bg-accent-muted flex items-center justify-center">
            <span className="text-[10px] font-bold text-accent-dark">B</span>
          </div>
          <span className="font-medium">BorneMap v0.1.0</span>
        </div>
      </div>
    </aside>
  );
}
