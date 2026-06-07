export interface Station {
  id: string;
  name: string;
  description: string | null;
  latitude: number;
  longitude: number;
  status: string;
  is_live: boolean;
  is_public: boolean;
  chargers?: Charger[];
}

export interface Charger {
  id: string;
  type: 'dc_fast' | 'ac_standard' | 'ac_fast';
  power: string;
  status: 'available' | 'occupied' | 'maintenance';
}

export interface Review {
  id: string;
  stationId: string;
  rating: {
    cleanliness: number;
    chargingSpeed: number;
    staff: number;
    overall: number;
  };
  reviewText: string;
  createdAt: string;
  updatedAt: string;
  user: {
    id: string;
    name: string;
    avatar?: string;
  };
}

export interface Favorite {
  id: string;
  stationId: string;
  createdAt: string;
}

export interface User {
  id: string;
  name: string;
  email: string;
  avatar?: string;
  favorites?: Favorite[];
  createdAt: string;
}

export interface AuthToken {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token: string;
}

export interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

export interface Error {
  code: string;
  message: string;
  details?: any;
}

export interface MapFilters {
  status?: 'active' | 'offline';
  minPower?: number;
  maxDistance?: number;
}

export interface NearbyStation extends Station {
  distance?: number;
  bearing?: number;
}

export interface ClickstreamEvent {
  event_id: string;
  event_name: string;
  user_id?: string;
  session_id?: string;
  timestamp: string;
  device_id?: string;
  app_version?: string;
  platform: string;
  data?: Record<string, any>;
}
