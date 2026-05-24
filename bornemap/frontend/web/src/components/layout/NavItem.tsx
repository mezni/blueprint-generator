import type { LucideIcon } from "lucide-react";
import { cn } from "../../lib/utils";

interface NavItemProps {
  icon: LucideIcon;
  label: string;
  active?: boolean;
  badge?: string | number;
  expanded?: boolean;
  onClick?: () => void;
}

const navStates =
  "group flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-all duration-150 ease-in-out";

export default function NavItem({
  icon: Icon,
  label,
  active = false,
  badge,
  expanded = false,
  onClick,
}: NavItemProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        navStates,
        active
          ? "bg-sidebar-active text-sidebar-text-active"
          : "text-sidebar-text hover:bg-sidebar-hover hover:text-slate-900",
        expanded && active && "bg-sidebar-active",
      )}
    >
      <Icon
        size={18}
        className={cn(
          "shrink-0",
          active ? "text-accent-dark" : "text-sidebar-text group-hover:text-slate-900",
        )}
      />
      <span className="flex-1 text-left">{label}</span>
      {badge !== undefined && (
        <span className="flex h-5 min-w-5 items-center justify-center rounded-full bg-accent px-1.5 text-[11px] font-semibold text-white">
          {badge}
        </span>
      )}
    </button>
  );
}
