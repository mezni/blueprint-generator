import { useQuery } from '@tanstack/react-query';
import { apiClient } from '@/lib/api';
import { useTheme } from '@/hooks/useTheme';
import config from '@/theme/config';

interface Station {
  id: string;
  name: string;
  description: string | null;
  latitude: number;
  longitude: number;
  status: string;
  is_live: boolean;
  is_public: boolean;
  chargers: any[];
}

export function useStations() {
  const { mode } = useTheme();
  const isRTL = mode === 'rtl';

  return useQuery<Station[]>({
    queryKey: ['stations', isRTL],
    queryFn: async () => {
      const { defaultLat, defaultLng, defaultRadiusKm, maxRadiusKm } = config.map;
      
      // Implement map query with filters
      // For MVP, we'll start with a basic implementation
      // TODO: Add real API call when API endpoint is available
      
      // Mock data for development
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
          chargers: [],
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
          chargers: [],
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
          chargers: [],
        },
      ];
    },
  });
}
