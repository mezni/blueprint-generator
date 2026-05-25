import { useState } from "react";
import { ChevronDown } from "lucide-react";
import type { LucideIcon } from "lucide-react";
import { cn } from "../../lib/utils";
import NavItem from "./NavItem";

interface NestedItem {
  icon: LucideIcon;
  label: string;
  active?: boolean;
  badge?: string | number;
  onClick?: () => void;
}

interface NavAccordionProps {
  icon: LucideIcon;
  label: string;
  defaultOpen?: boolean;
  active?: boolean;
  items: NestedItem[];
}

export default function NavAccordion({
  icon: Icon,
  label,
  defaultOpen = false,
  active = false,
  items,
}: NavAccordionProps) {
  const [open, setOpen] = useState(defaultOpen);

  return (
    <div>
      <button
        onClick={() => setOpen(!open)}
        className={cn(
          "group flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-all duration-150 ease-in-out",
          active
            ? "bg-sidebar-active text-sidebar-text-active"
            : "text-sidebar-text hover:bg-sidebar-hover hover:text-slate-900",
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
        <ChevronDown
          size={16}
          className={cn(
            "shrink-0 text-sidebar-text transition-transform duration-200",
            open && "rotate-180",
          )}
        />
      </button>
      <div
        className={cn(
          "mt-1 space-y-0.5 overflow-hidden transition-all duration-200 ease-in-out",
          open ? "max-h-96 opacity-100" : "max-h-0 opacity-0",
        )}
      >
        {items.map((item) => (
          <div key={item.label} className="ml-3 border-l-2 border-slate-200 pl-3">
            <NavItem
              icon={item.icon}
              label={item.label}
              active={item.active}
              badge={item.badge}
              onClick={item.onClick}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
