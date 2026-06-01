// api-client — typed REST client mirroring crates/contracts DTOs
// Auto-generated from Rust contracts crate — keep in sync

export const API_BASE_URL = "/api/v1";

export enum Role {
  RegisteredDriver = "RegisteredDriver",
  Partner = "Partner",
  Admin = "Admin",
}

export enum StationStatus {
  Active = "Active",
  Inactive = "Inactive",
  Maintenance = "Maintenance",
}

export enum PartnerStatus {
  Active = "Active",
  Suspended = "Suspended",
  Onboarding = "Onboarding",
}

export interface StationDTO {
  id: string;
  name: string;
  partner_id: string;
  address: string;
  latitude: number;
  longitude: number;
  charger_count: number;
  status: StationStatus;
}

export interface UserDTO {
  id: string;
  email: string;
  display_name: string;
  role: Role;
  created_at: string;
}

export interface PartnerDTO {
  id: string;
  name: string;
  contact_email: string;
  status: PartnerStatus;
  created_at: string;
}

export interface ReviewDTO {
  id: string;
  station_id: string;
  user_id: string;
  rating: number;
  comment: string | null;
  created_at: string;
}

export enum EventType {
  StationSearched = "StationSearched",
  StationViewed = "StationViewed",
  ChargingStarted = "ChargingStarted",
  ChargingCompleted = "ChargingCompleted",
  ReviewSubmitted = "ReviewSubmitted",
  PartnerStationCreated = "PartnerStationCreated",
  PartnerStationUpdated = "PartnerStationUpdated",
  UserRegistered = "UserRegistered",
  ErrorOccurred = "ErrorOccurred",
}

export interface ClickstreamEventEnvelope {
  event_id: string;
  event_type: EventType;
  user_id: string | null;
  session_id: string;
  payload: Record<string, unknown>;
  timestamp: string;
  source: string;
  trace_id: string;
}
