interface MapStateOverlayProps {
  state: 'idle' | 'active' | 'station-selected'
  hasStations: boolean
}

function MapStateOverlay({ state, hasStations }: MapStateOverlayProps) {
  if (state === 'idle') {
    return (
      <div className="pointer-events-none absolute inset-0 z-[1000] flex items-center justify-center bg-[var(--color-surface-base)]">
        <div className="flex flex-col items-center gap-4">
          <div className="h-12 w-12 animate-spin rounded-full border-4 border-[var(--color-border-base)] border-t-[var(--color-primary-base)]" />
          <p className="text-sm text-[var(--color-text-muted)]">Loading map...</p>
        </div>
      </div>
    )
  }

  if (state === 'active' && !hasStations) {
    return (
      <div className="pointer-events-none absolute inset-0 z-[1000] flex items-center justify-center">
        <p className="rounded-lg bg-[var(--color-surface-base)] px-6 py-3 shadow-lg">
          No stations in this area
        </p>
      </div>
    )
  }

  return null
}

export default MapStateOverlay
