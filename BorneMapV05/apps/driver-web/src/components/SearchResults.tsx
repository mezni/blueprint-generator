import type { StationListItem } from '@/lib/types'

interface SearchResultsProps {
  results: StationListItem[]
  totalResults: number
  onSelectStation: (stationId: string) => void
}

function SearchResults({ results, totalResults, onSelectStation }: SearchResultsProps) {
  if (results.length === 0) {
    return (
      <p className="py-8 text-center text-sm text-[var(--color-text-muted)]">
        No stations found
      </p>
    )
  }

  return (
    <div className="flex flex-col gap-2">
      <p className="text-xs text-[var(--color-text-muted)]">
        {totalResults} station{totalResults !== 1 ? 's' : ''} found
      </p>
      <div className="flex flex-col gap-1">
        {results.map((station) => (
          <button
            key={station.id}
            onClick={() => onSelectStation(station.id)}
            className="flex flex-col gap-1 rounded-lg border border-[var(--color-border-muted)] px-3 py-2 text-start transition-colors hover:bg-[var(--color-surface-hover)]"
          >
            <span className="text-sm font-medium text-[var(--color-text-base)]">
              {station.name}
            </span>
            <span className="text-xs text-[var(--color-text-muted)]">
              {station.city} · {station.distance_km?.toFixed(1)} km
            </span>
          </button>
        ))}
      </div>
    </div>
  )
}

export default SearchResults
