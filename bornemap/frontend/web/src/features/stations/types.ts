export interface Station {
  id: number;
  partner_id: number | null;
  name: string;
  operator: string | null;
  address: string | null;
  location: Record<string, unknown> | null;
  plug_types: string[] | null;
  speed_kw: number | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StationCreate {
  partner_id?: number;
  name: string;
  operator?: string;
  address?: string;
  plug_types?: string[];
  speed_kw?: number;
  is_active?: boolean;
}

export interface StationUpdate {
  partner_id?: number;
  name?: string;
  operator?: string;
  address?: string;
  plug_types?: string[];
  speed_kw?: number;
  is_active?: boolean;
}
