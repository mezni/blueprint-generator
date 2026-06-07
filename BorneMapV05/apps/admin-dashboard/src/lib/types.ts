import type { ItemEnvelope, SuccessEnvelope } from '@bornemap/api-contracts'

export type PartnerStatus = 'active' | 'suspended'
export type PartnerType = 'business' | 'private'
export type StationStatus = 'active' | 'inactive' | 'maintenance' | 'draft'
export type ChargerType = 'CCS' | 'Type2' | 'CHAdeMO'
export type ChargerStatus = 'available' | 'offline' | 'fault'
export type ReviewStatus = 'submitted' | 'published' | 'flagged' | 'hidden' | 'deleted'
export type UserStatus = 'active' | 'disabled'
export type UserRole = 'registered_driver' | 'partner' | 'admin'

export interface OverviewMetrics {
  total_partners: number
  total_stations: number
  active_stations: number
  pending_reviews: number
}

export interface Partner {
  id: string
  name: string
  email: string | null
  phone: string | null
  status: PartnerStatus
  created_at: string
  updated_at: string
  deleted_at: string | null
}

export interface PartnerCreate {
  name: string
  email?: string
  phone?: string
}

export interface PartnerUpdate {
  name?: string
  email?: string
  phone?: string
  status?: PartnerStatus
}

export interface Charger {
  id: string
  station_id: string
  type: ChargerType
  power_kw: number
  status: ChargerStatus
}

export interface Station {
  id: string
  partner_id: string
  partner_name: string
  name: string
  description: string | null
  latitude: number
  longitude: number
  status: StationStatus
  is_live: boolean
  is_public: boolean
  city: string | null
  chargers: Charger[]
  created_at: string
  deleted_at: string | null
}

export interface StationUpdate {
  name?: string
  description?: string
  latitude?: number
  longitude?: number
  status?: StationStatus
  is_live?: boolean
  is_public?: boolean
}

export interface Review {
  id: string
  station_id: string
  station_name: string
  user_id: string
  user_email: string
  rating: number
  comment: string
  status: ReviewStatus
  created_at: string
}

export interface ReviewStatusUpdate {
  status: ReviewStatus
}

export interface User {
  id: string
  keycloak_user_id: string
  email: string | null
  role: string | null
  created_at: string
  updated_at: string
}

export type OverviewResponse = ItemEnvelope<OverviewMetrics>
export type PartnerListResponse = SuccessEnvelope<Partner[]>
export type PartnerItemResponse = ItemEnvelope<Partner>
export type StationListResponse = SuccessEnvelope<Station[]>
export type StationItemResponse = ItemEnvelope<Station>
export type ReviewListResponse = SuccessEnvelope<Review[]>
export type ReviewItemResponse = ItemEnvelope<Review>
export type UserListResponse = SuccessEnvelope<User[]>
