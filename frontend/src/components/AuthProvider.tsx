import { useState, useEffect, useCallback, type ReactNode } from "react";
import { setAccessToken } from "../api/client";
import { me as meRequest, update as updateRequest } from "../api/user";
import {
  login as loginRequest,
  refresh as refreshRequest,
  logout as logoutRequest,
} from "../api/auth";
import { AuthContext, type User } from "../context/AuthContext";
import type { UpdateUser } from "../types/users";

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const clearAuth = useCallback(() => {
    setAccessToken(null);
    setUser(null);
  }, []);

  const fetchMe = useCallback(async () => {
    const { data } = await meRequest();
    setUser({
      username: data.username,
      bio: data.bio,
      avatar_key: data.avatar_key,
      online: data.online,
    });
  }, []);

  const updateProfile = useCallback(async (data: UpdateUser) => {
    const { data: updated } = await updateRequest(data);
    setUser((prev) => (prev ? { ...prev, ...updated } : prev));
  }, []);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const { data } = await refreshRequest();
        setAccessToken(data.access_token);
        if (!cancelled) await fetchMe();
      } catch {
        if (!cancelled) clearAuth();
      } finally {
        if (!cancelled) setIsLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [fetchMe, clearAuth]);

  const login = useCallback(
    async (username: string, password: string) => {
      const { data } = await loginRequest({ username, password });
      setAccessToken(data.access_token);
      await fetchMe();
    },
    [fetchMe],
  );

  const logout = useCallback(async () => {
    try {
      await logoutRequest();
    } finally {
      clearAuth();
    }
  }, [clearAuth]);

  return (
    <AuthContext.Provider
      value={{ user, isLoading, login, logout, updateProfile }}
    >
      {children}
    </AuthContext.Provider>
  );
}
