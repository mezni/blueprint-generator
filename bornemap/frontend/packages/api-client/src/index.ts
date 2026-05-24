export interface StationMarker {
  id: string;
  name: string;
  coord: [number, number];
  is_active: boolean;
  under_maintenance: boolean;
}

export interface StationDetail {
  id: string;
  company: { id: string; name: string };
  name: string;
  address: string;
  coord: [number, number];
  is_active: boolean;
  under_maintenance: boolean;
  opening_hours_osm: string | null;
  chargers: Charger[];
  created_at: string;
  updated_at: string;
}

export interface Charger {
  id: string;
  connector: ConnectorType;
  power_kw: number;
  is_active: boolean;
}

export type ConnectorType = "Type2" | "CCS" | "CHAdeMO" | "Type2_Tethered";

export interface StationListResponse {
  viewport: { west: number; south: number; east: number; north: number };
  markers: StationMarker[];
  truncated: boolean;
}

export interface MockLoginRequest {
  username: string;
  role: string;
}

export interface MockLoginResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
}

export interface AdminStationCreate {
  company_id: string;
  name: string;
  address: string;
  coord: [number, number];
  is_active?: boolean;
  under_maintenance?: boolean;
  opening_hours_osm?: string | null;
  chargers?: { connector: ConnectorType; power_kw: number; is_active?: boolean }[];
}

export interface AdminStationPatch {
  name?: string;
  address?: string;
  coord?: [number, number];
  is_active?: boolean;
  under_maintenance?: boolean;
  opening_hours_osm?: string | null;
}

export const CONNECTOR_TYPES: readonly ConnectorType[] = [
  "Type2",
  "CCS",
  "CHAdeMO",
  "Type2_Tethered",
] as const;
