import type { StationSummary, StationDetail } from '../types'

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000'

async function request<T>(path: string, params?: Record<string, string | number>): Promise<T> {
  const url = new URL(`${BASE_URL}${path}`)
  if (params) {
    Object.entries(params).forEach(([key, value]) => {
      url.searchParams.set(key, String(value))
    })
  }
  const res = await fetch(url.toString())
  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`)
  }
  return res.json()
}

export async function getNearbyStations(
  lat: number,
  lng: number,
  radiusKm = 10,
  limit = 20
): Promise<{ stations: StationSummary[] }> {
  return request('/stations/nearby', { lat, lng, radius_km: radiusKm, limit })
}

export async function getMarkers(
  bbox: string
): Promise<{ markers: { id: string; latitude: number; longitude: number; available_count: number }[] }> {
  return request('/stations/markers', { bbox })
}

export async function searchStations(
  q: string,
  params?: { lat?: number; lng?: number; connector_type?: string; min_power_kw?: number; limit?: number; offset?: number }
): Promise<{ stations: StationSummary[]; pagination: { offset: number; limit: number; total: number; has_more: boolean } }> {
  return request('/stations/search', { q, ...params })
}

export async function getStationDetail(id: string): Promise<{ station: StationDetail }> {
  return request(`/stations/${id}`)
}
