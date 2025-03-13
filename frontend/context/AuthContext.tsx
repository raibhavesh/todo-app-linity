'use client';

import React, { createContext, useState, useEffect, useContext } from 'react';
import { useRouter } from 'next/navigation';
import api, { setAuthToken, clearAuthToken, getAuthToken } from '../lib/api';

type User = {
  id: number;
  username: string;
};

type AuthContextType = {
  user: User | null;
  login: (username: string, password: string) => Promise<void>;
  signup: (username: string, password: string) => Promise<void>;
  logout: () => void;
  loading: boolean;
  error: string | null;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  // On mount, load the token and user from localStorage
  useEffect(() => {
    const token = getAuthToken(); // retrieves token from localStorage
    if (!token) {
      setLoading(false);
      return;
    }

    const storedUser = localStorage.getItem('auth_user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
    }
    setLoading(false);
  }, []);

  // Login
  const login = async (username: string, password: string) => {
    setError(null);
    setLoading(true);
    try {
        const result = await api.POST('/login', {
            body: { username, password },
        });
      // The backend returns just { "token": "..." }
      if (result.data?.token) {
        setAuthToken(result.data.token);
        // Since we only have the token, create a minimal user object
        const minimalUser: User = {
          id: -1, // Dummy ID (we don't get an ID from /login)
          username: username,
        };
        localStorage.setItem('auth_user', JSON.stringify(minimalUser));
        setUser(minimalUser);

        router.push('/');
      } else {
        setError('Invalid login response');
      }
    } catch (err) {
      setError('Login failed');
    } finally {
      setLoading(false);
    }
  };

  // Signup
  const signup = async (username: string, password: string) => {
    setError(null);
    setLoading(true);
    try {
      // 1) Call /register => returns a full user object
      const registerResult = await api.POST('/register', {
        body: { username, password },
      });
      if (!registerResult.data?.id) {
        setError('Invalid register response');
        setLoading(false);
        return;
      }

      // 2) Immediately call /login to get the token
      const loginResult = await api.POST('/login', {
        body: { username, password },
      });

      if (loginResult.data?.token) {
        setAuthToken(loginResult.data.token);

        // We have the user from /register
        const newUser: User = {
          id: registerResult.data.id,
          username: registerResult.data.username,
        };
        localStorage.setItem('auth_user', JSON.stringify(newUser));
        setUser(newUser);

        router.push('/');
      } else {
        setError('Failed to log in after signup');
      }
    } catch (err) {
      setError('Signup failed');
    } finally {
      setLoading(false);
    }
  };

  // Logout
  const logout = () => {
    clearAuthToken();
    localStorage.removeItem('auth_user');
    setUser(null);
    router.push('/login');
  };

  return (
    <AuthContext.Provider value={{ user, login, signup, logout, loading, error }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}