import { NavLink } from "react-router-dom";

const navItems = [
  { to: "/map", label: "Dashboard", icon: "◉" },
  { to: "/admin/stations", label: "Stations", icon: "⚡" },
];

export function Sidebar() {
  return (
    <aside className="w-64 h-full flex-shrink-0 border-r border-slate-800 bg-slate-900 flex flex-col">
      <div className="h-16 flex items-center px-6 border-b border-slate-800">
        <span className="text-lg font-bold tracking-tight">EV Admin</span>
      </div>
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            end
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? "bg-slate-800 text-white"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/50"
              }`
            }
          >
            <span className="text-base">{item.icon}</span>
            {item.label}
          </NavLink>
        ))}
      </nav>
      <div className="p-4 border-t border-slate-800">
        <div className="flex items-center gap-3 px-3 py-2 text-xs text-slate-500">
          <span className="w-2 h-2 rounded-full bg-green-500" />
          System Online
        </div>
      </div>
    </aside>
  );
}
