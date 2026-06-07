import { type ReactNode } from 'react'

interface DataCardProps {
  label: string
  value: string | number
  icon?: ReactNode
  onClick?: () => void
}

export function DataCard({ label, value, icon, onClick }: DataCardProps) {
  return (
    <button
      onClick={onClick}
      className="flex items-center gap-4 rounded-lg border border-[var(--color-border-muted)] bg-[var(--color-surface-base)] p-5 text-left shadow-sm transition-all hover:shadow-md hover:border-[var(--color-border-hover)]"
    >
      {icon && (
        <div className="flex h-10 w-10 items-center justify-center rounded-md bg-[var(--color-primary-muted)] text-[var(--color-primary-base)]">
          {icon}
        </div>
      )}
      <div>
        <p className="text-2xl font-bold text-[var(--color-text-base)]">{value}</p>
        <p className="text-sm text-[var(--color-text-muted)]">{label}</p>
      </div>
    </button>
  )
}
