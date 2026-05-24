import { create } from "zustand";
import { apiFetch } from "../../lib/apiClient";
import { queryClient } from "../../app/queryClient";

function base64UrlDecode(str: string): string {
  return atob(str.replace(/-/g, "+").replace(/_/g, "/"));
}

function parseToken(token: string): { exp: number } | null {
  try {
    return JSON.parse(base64UrlDecode(token.split(".")[1]));
  } catch {
    return null;
  }
}

function isTokenExpired(token: string | null): boolean {
  if (!token) return true;
  const payload = parseToken(token);
  if (!payload) return true;
  return payload.exp * 1000 < Date.now();
}

interface AuthState {
  token: string | null;
  isAuthenticated: boolean;
  login: (username: string) => Promise<void>;
  logout: () => void;
}

const initialToken = localStorage.getItem("bornemap_token");
const tokenValid = initialToken && !isTokenExpired(initialToken);

export const useAuthStore = create<AuthState>((set) => ({
  token: tokenValid ? initialToken : null,
  isAuthenticated: !!tokenValid,

  login: async (username: string) => {
    const data = await apiFetch<{
      access_token: string;
      token_type: string;
      expires_in: number;
    }>("/api/v1/auth/mock-login", {
      method: "POST",
      body: JSON.stringify({ username, role: "admin" }),
    });
    localStorage.setItem("bornemap_token", data.access_token);
    set({ token: data.access_token, isAuthenticated: true });
  },

  logout: () => {
    localStorage.removeItem("bornemap_token");
    queryClient.clear();
    set({ token: null, isAuthenticated: false });
  },
}));
