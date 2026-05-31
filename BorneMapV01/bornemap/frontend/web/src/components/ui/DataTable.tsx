import { Pencil, Trash2, ToggleLeft, ToggleRight, UserPlus } from "lucide-react";
import { Button } from "./button";
import { cn } from "../../lib/utils";

export interface Column<T> {
  key: string;
  header: string;
  render: (item: T) => React.ReactNode;
  className?: string;
}

interface DataTableProps<T> {
  data: T[];
  columns: Column<T>[];
  onEdit: (item: T) => void;
  onToggleActive: (item: T) => void;
  onDelete: (item: T) => void;
  onAdd: () => void;
  title: string;
  addLabel?: string;
}

export default function DataTable<T extends { id: number; is_active: boolean }>({
  data, columns, onEdit, onToggleActive, onDelete, onAdd, title, addLabel = "Add",
}: DataTableProps<T>) {
  return (
    <div>
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h2 className="text-sm font-semibold text-slate-900">{title}</h2>
          <p className="text-xs text-muted">{data.length} record{data.length !== 1 ? "s" : ""}</p>
        </div>
        <Button size="sm" onClick={onAdd}>
          <UserPlus size={14} className="mr-1" />
          {addLabel}
        </Button>
      </div>

      <div className="overflow-hidden rounded-xl border border-border">
        <table className="w-full text-left text-sm">
          <thead>
            <tr className="bg-slate-50 text-xs font-medium text-slate-500">
              {columns.map((col) => (
                <th key={col.key} className={cn("px-4 py-3", col.className)}>{col.header}</th>
              ))}
              <th className="px-4 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border">
            {data.map((item) => (
              <tr key={item.id} className="hover:bg-slate-50/50">
                {columns.map((col) => (
                  <td key={col.key} className={cn("px-4 py-3", col.className)}>{col.render(item)}</td>
                ))}
                <td className="px-4 py-3 text-right">
                  <div className="flex items-center justify-end gap-1">
                    <button onClick={() => onToggleActive(item)} className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-700"
                      title={item.is_active ? "Deactivate" : "Activate"}>
                      {item.is_active ? <ToggleRight size={16} /> : <ToggleLeft size={16} />}
                    </button>
                    <button onClick={() => onEdit(item)} className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-700" title="Edit">
                      <Pencil size={16} />
                    </button>
                    <button onClick={() => onDelete(item)} className="rounded-lg p-1.5 text-slate-400 hover:bg-red-50 hover:text-red-500" title="Delete">
                      <Trash2 size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {data.length === 0 && (
              <tr>
                <td colSpan={columns.length + 1} className="px-4 py-8 text-center text-sm text-slate-400">
                  No records found. Click "{addLabel}" to create one.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
