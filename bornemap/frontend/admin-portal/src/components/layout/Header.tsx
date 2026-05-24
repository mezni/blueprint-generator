import { useAuthStore } from "../../features/auth/authStore";
import { useLocation, useNavigate } from "react-router-dom";

const breadcrumbLabels: Record<string, string> = {
  "/map": "Dashboard",
  "/admin/stations": "Stations",
  "/admin/stations/new": "New Station",
};

export function Header() {
  const location = useLocation();
  const navigate = useNavigate();
  const logout = useAuthStore((s) => s.logout);
  const token = useAuthStore((s) => s.token);

  const username = token
    ? (() => {
        try {
          const payload = JSON.parse(atob(token.split(".")[1]));
          return payload.preferred_username || payload.sub || "Admin";
        } catch {
          return "Admin";
        }
      })()
    : "Admin";

  const isEdit = location.pathname.match(/\/admin\/stations\/([^/]+)\/edit/);
  const crumbs = isEdit
    ? ["Stations", "Edit Station"]
    : [breadcrumbLabels[location.pathname] || "Dashboard"];

  return (
    <header className="h-16 border-b border-slate-800 bg-slate-900 flex items-center justify-between px-6 flex-shrink-0">
      <div className="flex items-center gap-4">
        <div className="relative">
          <svg
            className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            placeholder="Search stations..."
            className="w-64 pl-10 pr-4 py-1.5 text-sm bg-slate-800 border border-slate-700 rounded-lg text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40 focus:border-blue-500/40"
          />
        </div>
        <nav className="flex items-center gap-2 text-sm text-slate-500">
          {crumbs.map((crumb, i) => (
            <span key={crumb} className="flex items-center gap-2">
              {i > 0 && (
                <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              )}
              <span className={i === crumbs.length - 1 ? "text-slate-200" : ""}>
                {crumb}
              </span>
            </span>
          ))}
        </nav>
      </div>
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <div className="flex gap-1">
            <span className="w-2 h-2 rounded-full bg-green-500" title="All systems operational" />
            <span className="w-2 h-2 rounded-full bg-green-500" />
            <span className="w-2 h-2 rounded-full bg-amber-500" title="1 degraded" />
          </div>
        </div>
        <span className="text-sm text-slate-300 bg-slate-800 px-2.5 py-1 rounded-md font-medium">
          admin
        </span>
        <span className="text-sm text-slate-400">{username}</span>
        <button
          onClick={() => { logout(); navigate("/login"); }}
          className="text-xs text-slate-500 hover:text-slate-300 transition-colors"
        >
          Logout
        </button>
      </div>
    </header>
  );
}
