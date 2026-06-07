import { useStationDetail } from '@/hooks/useStationDetail'
import StationInfo from './StationInfo'
import ChargerList from './ChargerList'
import FavoriteButton from './FavoriteButton'
import ReviewForm from './ReviewForm'
import ReviewList from './ReviewList'

interface StationDetailPanelProps {
  stationId: string
  onClose: () => void
}

function StationDetailPanel({ stationId, onClose }: StationDetailPanelProps) {
  const { data: station, isLoading, isError, refetch } = useStationDetail(stationId)

  return (
    <div className="flex h-full w-[360px] shrink-0 flex-col border-s border-[var(--color-border-base)] bg-[var(--color-surface-base)] shadow-lg">
      <div className="flex items-center justify-between border-b border-[var(--color-border-base)] px-4 py-3">
        <h2 className="text-lg font-semibold text-[var(--color-text-base)]">
          {isLoading ? 'Loading...' : station?.name ?? 'Station'}
        </h2>
        <div className="flex items-center gap-1">
          <FavoriteButton stationId={stationId} />
          <button
            onClick={onClose}
            className="rounded p-1 text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)]"
          >
            ✕
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        {isLoading && (
          <div className="flex items-center justify-center py-8">
            <div className="h-8 w-8 animate-spin rounded-full border-4 border-[var(--color-border-base)] border-t-[var(--color-primary-base)]" />
          </div>
        )}
        {isError && (
          <div className="flex flex-col items-center gap-3 py-8">
            <p className="text-sm text-[var(--color-error-base)]">Failed to load station details</p>
            <button
              onClick={() => refetch()}
              className="rounded bg-[var(--color-primary-base)] px-4 py-2 text-sm text-white hover:bg-[var(--color-primary-hover)]"
            >
              Retry
            </button>
          </div>
        )}
        {station && (
          <div className="flex flex-col gap-4">
            <StationInfo
              name={station.name}
              description={station.description}
              city={station.city}
              country={station.country}
              distanceKm={station.distance_km}
            />
            <ChargerList
              chargers={station.chargers}
              chargerTypes={station.charger_types}
            />
            <div className="border-t border-[var(--color-border-muted)] pt-4">
              <ReviewList stationId={stationId} />
            </div>
            <ReviewForm stationId={stationId} />
          </div>
        )}
      </div>
    </div>
  )
}

export default StationDetailPanel
