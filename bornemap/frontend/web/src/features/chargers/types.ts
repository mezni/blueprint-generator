export interface Charger {
  id: number;
  station_id: number | null;
  connector: string;
  power_kw: number | null;
  status: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface ChargerCreate {
  station_id?: number;
  connector: string;
  power_kw?: number;
  status?: string;
  is_active?: boolean;
}

export interface ChargerUpdate {
  station_id?: number;
  connector?: string;
  power_kw?: number;
  status?: string;
  is_active?: boolean;
}
