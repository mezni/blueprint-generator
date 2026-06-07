import { ApiClient } from '@bornemap/api-client'
import * as auth from '@bornemap/auth-client'

const GATEWAY_BASE_URL =
  (import.meta.env.VITE_GATEWAY_BASE_URL as string | undefined) ?? ''

export const apiClient = new ApiClient({
  baseUrl: `${GATEWAY_BASE_URL}/api/v1/partner`,
  getToken: async () => auth.getToken(),
})
