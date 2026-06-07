import { useEffect, useRef } from 'react'
import L from 'leaflet'
import type { StationListItem } from '@/lib/types'

interface StationMarkersProps {
  map: L.Map
  stations: StationListItem[]
  selectedStationId: string | null
  onMarkerClick: (stationId: string) => void
}

function createStationIcon(): L.DivIcon {
  return L.divIcon({
    className: 'station-marker',
    html: '<div class="station-marker-icon" />',
    iconSize: [24, 24],
    iconAnchor: [12, 12],
  })
}

function StationMarkers({ map, stations, selectedStationId, onMarkerClick }: StationMarkersProps) {
  const clusterGroupRef = useRef<L.MarkerClusterGroup | null>(null)

  useEffect(() => {
    if (!clusterGroupRef.current) {
      const group = L.markerClusterGroup({
        chunkedLoading: true,
        maxClusterRadius: 50,
        spiderfyOnMaxZoom: true,
        showCoverageOnHover: false,
        zoomToBoundsOnClick: true,
      })
      map.addLayer(group)
      clusterGroupRef.current = group
    }

    const group = clusterGroupRef.current
    group.clearLayers()

    stations.forEach((station) => {
      const marker = L.marker([station.latitude, station.longitude], {
        icon: createStationIcon(),
      })

      const tooltipText = station.availability
        ? `${station.name} — ${station.availability}`
        : station.name
      marker.bindTooltip(tooltipText, { direction: 'top' })

      marker.on('click', () => onMarkerClick(station.id))

      if (station.id === selectedStationId) {
        marker.setZIndexOffset(1000)
      }

      group.addLayer(marker)
    })

    return () => {
      group.clearLayers()
    }
  }, [map, stations, selectedStationId, onMarkerClick])

  return null
}

export default StationMarkers
