import { useState, useCallback, useMemo, useEffect, useRef } from 'react'
import { MapContainer } from './ui/map-container'
import StationMarkers from './StationMarkers'
import MapStateOverlay from './MapStateOverlay'
import StationDetailPanel from './StationDetailPanel'
import SearchOverlay from './SearchOverlay'
import { useStationMarkers } from '@/hooks/useStationMarkers'
import { useFavorites } from '@/hooks/useFavorites'
import { useViewport } from '@/hooks/useViewport'
import { useClickstream } from '@/hooks/useClickstream'
import type L from 'leaflet'

type MapState = 'idle' | 'active' | 'station-selected'

interface MapViewProps {
  searchOpen: boolean
  onSearchClose: () => void
}

function MapView({ searchOpen, onSearchClose }: MapViewProps) {
  const [mapInstance, setMapInstance] = useState<L.Map | null>(null)
  const [mapState, setMapState] = useState<MapState>('idle')
  const [selectedStationId, setSelectedStationId] = useState<string | null>(null)
  const [showFavoritesOnly, setShowFavoritesOnly] = useState(false)
  const [viewportParams, setViewportParams] = useState<{ lat: number; lng: number; radiusKm: number } | null>(null)
  const { emit } = useClickstream()
  const hasEmittedPageView = useRef(false)

  useEffect(() => {
    if (!hasEmittedPageView.current) {
      hasEmittedPageView.current = true
      emit('page.viewed')
    }
  }, [emit])

  const { data: stations = [] } = useStationMarkers(
    viewportParams ?? { lat: 36.8065, lng: 10.1815, radiusKm: 50 },
  )
  const { data: favoriteIds } = useFavorites()

  const visibleStations = useMemo(() => {
    if (!showFavoritesOnly || !favoriteIds) return stations
    return stations.filter((s) => favoriteIds.includes(s.id))
  }, [stations, showFavoritesOnly, favoriteIds])

  useViewport(
    mapInstance,
    500,
    useCallback((state) => {
      setViewportParams({ lat: state.center[0], lng: state.center[1], radiusKm: state.radiusKm })
      setMapState('active')
      emit('map.viewport_changed', { lat: state.center[0], lng: state.center[1], radiusKm: state.radiusKm })
    }, [emit]),
  )

  const handleMarkerClick = useCallback((stationId: string) => {
    setSelectedStationId(stationId)
    setMapState('station-selected')
    onSearchClose()
    emit('station.marker_clicked', { stationId })
  }, [onSearchClose, emit])

  const handleMapMount = useCallback((map: L.Map) => {
    setMapInstance(map)
    setMapState('active')
    emit('map.loaded')
  }, [emit])

  const handleCloseDetail = useCallback(() => {
    setSelectedStationId(null)
    setMapState('active')
  }, [])

  return (
    <div className="relative flex flex-1">
      <MapContainer
        className="h-full flex-1"
        onMount={handleMapMount}
        onViewportChange={() => {}}
      />
      {mapInstance && (
        <StationMarkers
          map={mapInstance}
          stations={visibleStations}
          selectedStationId={selectedStationId}
          onMarkerClick={handleMarkerClick}
        />
      )}
      <MapStateOverlay
        state={mapState}
        hasStations={visibleStations.length > 0}
      />
      <SearchOverlay
        isOpen={searchOpen}
        onClose={onSearchClose}
        onSelectStation={handleMarkerClick}
      />
      <div className="absolute start-4 top-4 z-[1000] flex flex-col gap-2">
        <button
          onClick={() => setShowFavoritesOnly((p) => !p)}
          className={`rounded-lg border px-3 py-1.5 text-sm shadow transition-colors ${
            showFavoritesOnly
              ? 'border-[var(--color-primary-base)] bg-[var(--color-primary-base)] text-white'
              : 'border-[var(--color-border-base)] bg-[var(--color-surface-base)] text-[var(--color-text-base)] hover:bg-[var(--color-surface-hover)]'
          }`}
        >
          {showFavoritesOnly ? 'Favorites' : 'All Stations'}
        </button>
      </div>
      {selectedStationId && (
        <StationDetailPanel
          stationId={selectedStationId}
          onClose={handleCloseDetail}
        />
      )}
    </div>
  )
}

export default MapView
