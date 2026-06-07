import Logger from './logger';
import StationService from './station-service';
import AuthService from './auth-service';

// Mock data for development
export class MockService {
  static async initialize(): Promise<void> {
    Logger.info('Initializing mock services...');
    
    // Mock stations data
    await StationService.getStations(); // Triggers cache population
    Logger.info('Mock services initialized successfully');
  }

  static getMockStations() {
    return [
      {
        id: 'STN-001',
        name: 'Tunis Nord',
        description: 'Large charging station with multiple chargers',
        latitude: 36.8780,
        longitude: 10.1885,
        status: 'active',
        is_live: true,
        is_public: true,
        chargers: [
          { type: 'dc_fast', count: 4, power: '50kW' },
          { type: 'ac_standard', count: 2, power: '22kW' },
        ],
      },
      {
        id: 'STN-002',
        name: 'Sidi Bou Said',
        description: 'Scenic charging station with café',
        latitude: 36.9068,
        longitude: 10.3606,
        status: 'active',
        is_live: true,
        is_public: true,
        chargers: [
          { type: 'dc_fast', count: 2, power: '150kW' },
        ],
      },
      {
        id: 'STN-003',
        name: 'Sfax',
        description: 'Regional station with fast chargers',
        latitude: 34.7407,
        longitude: 10.7603,
        status: 'active',
        is_live: true,
        is_public: true,
        chargers: [
          { type: 'dc_fast', count: 6, power: '150kW' },
        ],
      },
      {
        id: 'STN-004',
        name: 'Ariana',
        description: 'Urban charging station near highway',
        latitude: 36.8635,
        longitude: 10.1903,
        status: 'active',
        is_live: true,
        is_public: true,
        chargers: [
          { type: 'dc_fast', count: 3, power: '50kW' },
        ],
      },
      {
        id: 'STN-005',
        name: 'Monastir',
        description: 'Coastal charging station',
        latitude: 35.7844,
        longitude: 10.8387,
        status: 'active',
        is_live: true,
        is_public: true,
        chargers: [
          { type: 'dc_fast', count: 4, power: '150kW' },
          { type: 'ac_standard', count: 3, power: '22kW' },
        ],
      },
    ];
  }

  static getMockReviews() {
    return [
      {
        id: 'REV-001',
        stationId: 'STN-001',
        rating: { cleanliness: 5, chargingSpeed: 5, staff: 4, overall: 5 },
        reviewText: 'Excellent service and very clean facilities!',
        createdAt: new Date().toISOString(),
        user: {
          id: 'USER-001',
          name: 'Ahmed Ben Ali',
        },
      },
      {
        id: 'REV-002',
        stationId: 'STN-002',
        rating: { cleanliness: 4, chargingSpeed: 5, staff: 5, overall: 4 },
        reviewText: 'Beautiful location with great cafe nearby.',
        createdAt: new Date().toISOString(),
        user: {
          id: 'USER-002',
          name: 'Fatma Zouari',
        },
      },
    ];
  }
}

export default MockService;
