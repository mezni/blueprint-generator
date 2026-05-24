import { create } from "zustand";
import { apiFetch } from "../../lib/apiClient";

interface AuthState {
  token: string | null;
  isAuthenticated: boolean;
  login: (username: string) => Promise<void>;
  logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: localStorage.getItem("bornemap_token"),
  isAuthenticated: !!localStorage.getItem("bornemap_token"),

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
    set({ token: null, isAuthenticated: false });
  },
}));
