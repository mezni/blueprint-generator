export interface ApiSuccessResponse<T = unknown> {
  success: true
  data: T
  meta?: Record<string, unknown>
}

export interface ApiErrorResponse {
  success: false
  error: {
    code: string
    message: string
  }
}

export type ApiResponse<T = unknown> = ApiSuccessResponse<T> | ApiErrorResponse

export interface HealthResponse {
  status: 'ok'
  service: string
  version: string
}
