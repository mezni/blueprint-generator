export interface Station {
  id: string
  name: string
  latitude: number
  longitude: number
  status: 'active' | 'inactive' | 'maintenance'
  isLive: boolean
  isPublic: boolean
}

export interface Charger {
  id: string
  stationId: string
  type: 'CCS' | 'Type2' | 'CHAdeMO'
  powerKw: number
  status: 'available' | 'offline' | 'fault'
}

export interface Partner {
  id: string
  name: string
  status: 'active' | 'suspended'
}
