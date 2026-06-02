export type UserRole = 'registered_driver' | 'partner' | 'admin'

export interface AuthUser {
  id: string
  email: string
  role: UserRole
}

export interface LoginRequest {
  email: string
  password: string
}

export interface LoginResponse {
  accessToken: string
  refreshToken: string
  user: AuthUser
}
