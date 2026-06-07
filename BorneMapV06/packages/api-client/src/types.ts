export interface Station {
  id: string
  name: string
  address?: string
  latitude: number
  longitude: number
}

export interface StationSummary extends Station {
  distance_m?: number | null
  charger_count: number
  available_count: number
}

export interface Charger {
  id: string
  connector_type: string
  power_kw?: number | null
  status: 'available' | 'in_use' | 'maintenance' | 'offline'
}

export interface StationDetail extends Station {
  chargers: Charger[]
  rating: {
    average: number | null
    review_count: number
  }
}

export interface Pagination {
  offset: number
  limit: number
  total: number
  has_more: boolean
}

export interface ErrorResponse {
  error: string
  message: string
}
