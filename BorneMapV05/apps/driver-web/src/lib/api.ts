import { ApiClient } from '@bornemap/api-client'
import * as auth from '@bornemap/auth-client'

// Path uses /api/v1/driver (singular) matching both the Traefik route prefix
// (infra/compose/traefik/dynamic/routes.yml) and the driver-service's own
// internal route paths. No prefix stripping needed — full path forwarded.
const GATEWAY_BASE_URL =
  (import.meta.env.VITE_GATEWAY_BASE_URL as string | undefined) ?? ''

export const apiClient = new ApiClient({
  baseUrl: `${GATEWAY_BASE_URL}/api/v1/driver`,
  getToken: async () => auth.getToken(),
})
