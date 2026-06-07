import { useRef, useCallback, useEffect } from 'react'
import type L from 'leaflet'

interface ViewportState {
  center: [number, number]
  radiusKm: number
}

function zoomToRadius(zoom: number): number {
  if (zoom >= 15) return 2
  if (zoom >= 13) return 5
  if (zoom >= 11) return 10
  if (zoom >= 9) return 25
  return 50
}

export function useViewport(
  map: L.Map | null,
  debounceMs = 500,
  onViewportChange?: (state: ViewportState) => void,
) {
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const abortRef = useRef<AbortController | null>(null)

  const handleMoveEnd = useCallback(() => {
    if (!map) return

    const center = map.getCenter()
    const zoom = map.getZoom()
    const state: ViewportState = {
      center: [center.lat, center.lng],
      radiusKm: zoomToRadius(zoom),
    }

    if (timerRef.current) clearTimeout(timerRef.current)
    timerRef.current = setTimeout(() => {
      if (abortRef.current) abortRef.current.abort()
      abortRef.current = new AbortController()
      onViewportChange?.(state)
    }, debounceMs)
  }, [map, debounceMs, onViewportChange])

  useEffect(() => {
    if (!map) return
    map.on('moveend', handleMoveEnd)
    return () => {
      map.off('moveend', handleMoveEnd)
    }
  }, [map, handleMoveEnd])

  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current)
      if (abortRef.current) abortRef.current.abort()
    }
  }, [])

  return { abortRef }
}
