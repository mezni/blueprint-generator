# Driver Web App — Quickstart

## Setup

```bash
# Install dependencies
cd apps/driver-web
npm install @tanstack/react-query react-router
npm install leaflet.markercluster
npm install --save-dev @types/leaflet.markercluster

# Auth client needs keycloak-js
cd packages/auth-client
npm install keycloak-js

# Link workspace packages
cd apps/driver-web
npm install @bornemap/api-client @bornemap/auth-client @bornemap/event-taxonomy
```

## App Shell

```tsx
// main.tsx
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter } from "react-router";
import { AuthProvider } from "@bornemap/auth-client";
import App from "./App";

const queryClient = new QueryClient();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AuthProvider>
          <App />
        </AuthProvider>
      </BrowserRouter>
    </QueryClientProvider>
  </StrictMode>,
);
```

## Using API Client

```tsx
// lib/api.ts
import { ApiClient } from "@bornemap/api-client";
import { getToken } from "@bornemap/auth-client";

export const apiClient = new ApiClient({
  baseUrl: "http://localhost/api/v1/driver",
  getToken,
});
```

## Using Hooks

```tsx
import { useStationMarkers } from "@/hooks/useStationMarkers";

function MapView() {
  const { stations, isLoading } = useStationMarkers({
    lat: 36.8065,
    lng: 10.1815,
    radiusKm: 10,
  });

  if (isLoading) return <MapStateOverlay state="idle" />;
  return <StationMarkers stations={stations} />;
}
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.tsx` | App entry with providers |
| `src/App.tsx` | Layout shell (header + map) |
| `src/lib/api.ts` | ApiClient instance |
| `src/hooks/useStationMarkers.ts` | Viewport-driven station query |
| `src/hooks/useStationDetail.ts` | Single station detail |
| `src/hooks/useSearch.ts` | Debounced search |
| `src/hooks/useFavorites.ts` | Favorites list + toggle |
| `src/hooks/useReviews.ts` | Reviews CRUD |
| `src/hooks/useAuth.ts` | Auth state + gated actions |
| `src/hooks/useClickstream.ts` | Event emission |
| `src/components/MapView.tsx` | Main map container |
| `src/components/StationDetailPanel.tsx` | Side panel |
| `src/components/SearchOverlay.tsx` | Search overlay |
| `src/components/AuthModal.tsx` | Login modal |
| `src/components/FavoriteButton.tsx` | Favorite toggle |
| `src/components/ReviewForm.tsx` | Review create/edit |
| `src/components/ReviewList.tsx` | Review display |

## Acceptance Checklist

- [ ] Map loads with clustered markers (US1)
- [ ] Viewport changes debounce and refetch (US1)
- [ ] Marker click shows detail side panel (US2)
- [ ] Search returns results within 500ms (US2)
- [ ] Anonymous browsing works (US3)
- [ ] Auth modal appears on gated action (US3)
- [ ] Action completes after login (US3)
- [ ] Favorite toggle works (US4)
- [ ] Review create/edit/delete works (US4)
- [ ] Clickstream events fire for all interactions (US5)
- [ ] RTL layout renders correctly (SC-007)
