/**
 * Parse environment variable as number with validation
 */
const parseCoordinate = (value: string | undefined, defaultValue: number): number => {
  if (!value) return defaultValue;
  const parsed = parseFloat(value);
  if (isNaN(parsed)) {
    console.warn(`Invalid coordinate value: ${value}, using default: ${defaultValue}`);
    return defaultValue;
  }
  return parsed;
};

/**
 * Parse environment variable as integer with validation
 */
const parseInteger = (value: string | undefined, defaultValue: number): number => {
  if (!value) return defaultValue;
  const parsed = parseInt(value, 10);
  if (isNaN(parsed)) {
    console.warn(`Invalid integer value: ${value}, using default: ${defaultValue}`);
    return defaultValue;
  }
  return parsed;
};

/**
 * Application configuration loaded from environment variables
 * All EXPO_PUBLIC_* variables are accessible at runtime
 */
export const config = {
  api: {
    baseUrl: (process.env.EXPO_PUBLIC_API_BASE_URL as string) ?? 'https://api.example.tn',
    authUrl: (process.env.EXPO_PUBLIC_AUTH_BASE_URL as string) ?? 'https://auth.example.tn',
  },
  keycloak: {
    realm: (process.env.EXPO_PUBLIC_REALM as string) ?? 'bornemap',
    clientId: (process.env.EXPO_PUBLIC_CLIENT_ID as string) ?? 'bornemap-driver-mobile',
    supportedLanguages: ((process.env.EXPO_PUBLIC_SUPPORTED_LANGUAGES as string) ?? 'ar,fr').split(','),
  },
  map: {
    defaultLat: parseCoordinate(process.env.EXPO_PUBLIC_MAP_LAT as string, 36.8065),
    defaultLng: parseCoordinate(process.env.EXPO_PUBLIC_MAP_LNG as string, 10.1815),
    defaultRadiusKm: parseInteger(process.env.EXPO_PUBLIC_MAP_DEFAULT_RADIUS_KM as string, 10),
    maxRadiusKm: parseInteger(process.env.EXPO_PUBLIC_MAP_MAX_RADIUS_KM as string, 50),
  },
  featureFlags: {
    enableReviews: (process.env.EXPO_PUBLIC_FF_ENABLE_REVIEWS as string) === 'true',
    enableGisSync: (process.env.EXPO_PUBLIC_FF_ENABLE_GIS_SYNC as string) === 'true',
    enableAnalytics: (process.env.EXPO_PUBLIC_FF_ENABLE_ANALYTICS as string) === 'true',
  },
  offline: {
    cacheSizeMb: parseInteger(process.env.EXPO_PUBLIC_OFFLINE_CACHE_SIZE_MB as string, 100),
  },
} as const;
