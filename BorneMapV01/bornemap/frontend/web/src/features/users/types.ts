export interface User {
  id: number;
  username: string;
  email: string;
  role: "admin" | "editor" | "viewer";
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface UserCreate {
  username: string;
  email: string;
  role: string;
  password?: string;
  is_active?: boolean;
}

export interface UserUpdate {
  username?: string;
  email?: string;
  role?: string;
  password?: string;
  is_active?: boolean;
}
