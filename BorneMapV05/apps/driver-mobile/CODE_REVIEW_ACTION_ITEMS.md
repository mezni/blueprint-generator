# Code Review - Action Items

**Priority**: HIGH - Fix before Phase 2 implementation  
**Estimated Time**: 4-6 hours

---

## Critical Issues to Fix Immediately

### 1. FIX: Duplicate `version` in package.json

**Current State** (WRONG):
```json
{
  "name": "@bornemap/driver-mobile",
  "version": "0.0.0",
  "main": "expo-router/entry",
  "version": "0.1.0",
  ...
}
```

**Action**: Remove duplicate field
```json
{
  "name": "@bornemap/driver-mobile",
  "version": "0.1.0",
  "main": "expo-router/entry",
  ...
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 5 minutes

---

### 2. FIX: Expo Environment Variables

**Current State** (WRONG):
```typescript
// src/lib/api.ts
const GATEWAY_BASE_URL = (import.meta.env.VITE_GATEWAY_BASE_URL as string) ?? 'http://localhost';
```

**Action**: Change to Expo format
```typescript
// src/lib/api.ts
const GATEWAY_BASE_URL = (process.env.EXPO_PUBLIC_API_BASE_URL as string) ?? 'https://api.example.tn';
const API_BASE_URL = `${GATEWAY_BASE_URL}/api/v1/driver`;
```

**Also Update** `src/theme/config.ts`:
```typescript
// WRONG
defaultLat: parseFloat(import.meta.env.EXPO_PUBLIC_MAP_LAT || '36.8065'),

// RIGHT
defaultLat: parseFloat(process.env.EXPO_PUBLIC_MAP_LAT || '36.8065'),
```

**Status**: ❌ Not Fixed  
**Est. Time**: 15 minutes

---

### 3. FIX: Replace `any` Types with Proper Interfaces

**Files to Update**:
- `src/hooks/useAuth.ts`
- `src/services/auth-service.ts`
- `src/services/station-service.ts`
- `src/services/offline-manager.ts`

**Action**: Create interfaces file
```typescript
// src/types/auth.ts
export interface LoginCredentials {
  email: string;
  password: string;
}

export interface UserData {
  id: string;
  name: string;
  email: string;
  avatar?: string;
  createdAt: string;
}

export interface AuthResponse {
  token: string;
  user: UserData;
}

// src/types/station.ts
export interface Station {
  id: string;
  name: string;
  description: string | null;
  latitude: number;
  longitude: number;
  status: 'active' | 'inactive';
  is_live: boolean;
  is_public: boolean;
  chargers?: Charger[];
  createdAt: string;
  updatedAt: string;
}

export interface Charger {
  id: string;
  stationId: string;
  connector_type: string;
  power_rating: number;
  status: 'AVAILABLE' | 'OCCUPIED' | 'MAINTENANCE';
  is_reserved: boolean;
}
```

**Then Update Services**:
```typescript
// src/hooks/useAuth.ts
interface AuthContextType {
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (credentials: LoginCredentials) => Promise<boolean>;  // ← Typed
  logout: () => Promise<void>;
  user: UserData | null;  // ← Typed
}

// src/services/auth-service.ts
static async login(credentials: LoginCredentials): Promise<AuthResponse> {
  try {
    // Implementation
    return { token, user };
  } catch (error) {
    throw new AuthenticationError('Login failed', { cause: error });
  }
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 1-2 hours

---

### 4. FIX: Add Custom Error Classes

**Action**: Create error classes file
```typescript
// src/lib/errors.ts
export class ApiError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public details?: any
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export class AuthenticationError extends Error {
  constructor(message: string, public details?: any) {
    super(message);
    this.name = 'AuthenticationError';
  }
}

export class NetworkError extends Error {
  constructor(message: string, public offline?: boolean) {
    super(message);
    this.name = 'NetworkError';
  }
}

export class ValidationError extends Error {
  constructor(message: string, public field?: string) {
    super(message);
    this.name = 'ValidationError';
  }
}
```

**Then Update Services**:
```typescript
// src/services/auth-service.ts
import { AuthenticationError } from '@/lib/errors';

static async login(credentials: LoginCredentials): Promise<AuthResponse> {
  try {
    const response = await apiClient.post('/auth/login', credentials);
    return response.data;
  } catch (error) {
    if (error instanceof ApiError) {
      throw new AuthenticationError('Invalid credentials', { 
        statusCode: error.statusCode 
      });
    }
    throw new AuthenticationError('Login failed', { cause: error });
  }
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 45 minutes

---

### 5. FIX: Separate AuthProvider from Hook

**Current Structure** (WRONG):
```
src/hooks/useAuth.ts  (contains both Provider and Hook)
```

**Action**: Create new context file
```typescript
// src/context/AuthContext.tsx
import React, { createContext, ReactNode } from 'react';

export interface AuthContextType {
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (credentials: LoginCredentials) => Promise<boolean>;
  logout: () => Promise<void>;
  user: UserData | null;
}

export const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  // Provider implementation
  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}
```

**Update Hook File**:
```typescript
// src/hooks/useAuth.ts
import { useContext } from 'react';
import { AuthContext, AuthContextType } from '@/context/AuthContext';

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 30 minutes

---

## Major Issues to Fix Soon

### 6. FIX: Add Input Validation to Services

**Action**: Add validation at service boundaries
```typescript
// src/services/station-service.ts
import { ValidationError } from '@/lib/errors';

export class StationService {
  static async getStation(id: string): Promise<Station> {
    if (!id || id.trim() === '') {
      throw new ValidationError('Station ID cannot be empty', 'id');
    }
    if (!/^[A-Z0-9-]+$/.test(id)) {
      throw new ValidationError('Invalid station ID format', 'id');
    }
    
    try {
      const response = await apiClient.get(`/stations/${id}`);
      return response.data;
    } catch (error) {
      throw new ApiError('Failed to fetch station', 404, { cause: error });
    }
  }

  static async getNearbyStations(
    lat: number,
    lng: number,
    radiusKm: number
  ): Promise<Station[]> {
    // Validate inputs
    if (lat < -90 || lat > 90) {
      throw new ValidationError('Latitude must be between -90 and 90', 'latitude');
    }
    if (lng < -180 || lng > 180) {
      throw new ValidationError('Longitude must be between -180 and 180', 'longitude');
    }
    if (radiusKm <= 0 || radiusKm > 100) {
      throw new ValidationError('Radius must be between 0 and 100 km', 'radius');
    }

    try {
      const response = await apiClient.get('/stations/nearby', {
        params: { lat, lng, radius: radiusKm },
      });
      return response.data;
    } catch (error) {
      throw new NetworkError('Failed to fetch nearby stations');
    }
  }
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 1 hour

---

### 7. FIX: Use Logger Service Consistently

**Action**: Replace all console.error with Logger
```typescript
// Find and replace pattern:
console.error('...')  → Logger.error('...')
console.log('...')    → Logger.info('...')
console.warn('...')   → Logger.warn('...')

// Example:
// Before
catch (error) {
  console.error('Failed to cache stations:', error);
  throw error;
}

// After
catch (error) {
  Logger.error('Failed to cache stations', error);
  throw error;
}
```

**Files to Update**:
- src/hooks/useAuth.ts
- src/hooks/useNetworkStatus.ts
- src/services/auth-service.ts
- src/services/offline-manager.ts
- src/services/station-service.ts
- src/services/notification-service.ts

**Status**: ❌ Not Fixed  
**Est. Time**: 30 minutes

---

### 8. FIX: Fix useFavorites Hook Dependencies

**Action**: Fix React hook dependencies
```typescript
// src/hooks/useFavorites.ts
import { useState, useCallback, useEffect } from 'react';

export function useFavorites() {
  const queryClient = useQueryClient();
  const [favorites, setFavorites] = useState<Set<string>>(new Set());

  // Define loadFavorites first without dependencies
  const loadFavorites = useCallback(async () => {
    try {
      const favoritesData = await AsyncStorage.getItem(FAVORITES_KEY);
      if (favoritesData) {
        setFavorites(new Set(JSON.parse(favoritesData)));
      }
    } catch (error) {
      Logger.error('Failed to load favorites', error);
    }
  }, []);

  // Then use it in useEffect
  useEffect(() => {
    loadFavorites();
  }, [loadFavorites]);

  // Rest of implementation...
}
```

**Status**: ❌ Not Fixed  
**Est. Time**: 20 minutes

---

## Priority Fix Checklist

### CRITICAL (Do First)
- [ ] Fix package.json duplicate version
- [ ] Fix Expo environment variables
- [ ] Replace `any` types with proper interfaces
- [ ] Add custom error classes
- [ ] Separate AuthProvider from hook

**Est. Time**: 2-3 hours

### IMPORTANT (Do Before Phase 2)
- [ ] Add input validation to services
- [ ] Replace console with Logger
- [ ] Fix useEffect dependencies
- [ ] Add JSDoc comments to services
- [ ] Update types to match specification

**Est. Time**: 2-3 hours

### NICE TO HAVE (Phase 2+)
- [ ] Add unit tests
- [ ] Add accessibility attributes
- [ ] Add skeleton loading screens
- [ ] Internationalize UI strings
- [ ] Optimize with memo/useMemo

**Est. Time**: 4-5 hours

---

## Summary

**Total Time to Fix All Issues**: 4-6 hours

**Critical Issues**: 5
**Major Issues**: 3
**Minor Issues**: 12

**Can proceed to Phase 2 after fixing**: Critical + Important items

**Recommended timeline**:
- Today: Fix critical issues (2-3 hours)
- Tomorrow: Fix important issues (2-3 hours)
- Then: Proceed to Phase 2 implementation

---

## Testing After Fixes

After implementing all fixes, run:

```bash
# Type checking
npx tsc --noEmit

# Linting
npx eslint src/

# Build
npm run build

# Manual testing
npm run dev
```

**All checks should pass before Phase 2 implementation.**

---

**Prepared by**: OpenCode Code Review Agent  
**Date**: June 4, 2026  
**Status**: Action Items Ready for Implementation
