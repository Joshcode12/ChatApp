import { createContext } from "react";
import type { UpdateUser } from "../types/users";

export interface User {
  username: string;
  bio?: string;
  avatar_key?: string;
  online: boolean;
}

export interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  updateProfile: (data: UpdateUser) => Promise<void>;
}

export const AuthContext = createContext<AuthContextType | undefined>(
  undefined,
);
