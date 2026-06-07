export interface StationListItem {
  id: string
  name: string
  description: string | null
  latitude: number
  longitude: number
  city: string | null
  country: string | null
  distance_km: number | null
  charger_types: ChargerTypeInfo[]
  availability: StationAvailability | null
  review_summary: ReviewSummary | null
}

export interface StationDetail {
  id: string
  name: string
  description: string | null
  latitude: number
  longitude: number
  city: string | null
  country: string | null
  distance_km: number | null
  chargers: Charger[]
  charger_types: ChargerTypeInfo[]
  availability: StationAvailability | null
  review_summary: ReviewSummary | null
}

export interface ChargerTypeInfo {
  connector_type: ConnectorType
  power_kw: number | null
  status: ChargerStatus
}

export interface Charger {
  id: string
  station_id: string
  connector_type: ConnectorType
  power_kw: number | null
  status: ChargerStatus
  created_at: string
  updated_at: string
}

export type ConnectorType = 'CCS' | 'Type2' | 'CHAdeMO'
export type ChargerStatus = 'available' | 'offline' | 'fault'
export type StationAvailability = 'available' | 'limited' | 'unavailable'

export interface Review {
  id: string
  user_id: string
  station_id: string
  rating: number
  comment: string | null
  status: ReviewStatus
  created_at: string
  updated_at: string
}

export type ReviewStatus = 'published' | 'hidden' | 'flagged' | 'deleted'

export interface ReviewSummary {
  average_rating: number | null
  total_reviews: number
}

export interface ReviewCreate {
  station_id: string
  rating: number
  comment?: string
}

export interface ReviewUpdate {
  rating?: number
  comment?: string
}

export interface SearchQuery {
  q?: string
  city?: string
  connector_type?: ConnectorType
  availability?: StationAvailability
  page?: number
  size?: number
}

export interface StationListParams {
  lat: number
  lng: number
  radiusKm: number
  connectorType?: ConnectorType
  availability?: StationAvailability
}
