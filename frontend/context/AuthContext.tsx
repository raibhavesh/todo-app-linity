'use client';

import React, { createContext, useState, useEffect, useContext } from 'react';
import { useRouter } from 'next/navigation';
import api, { setAuthToken, getAuthToken, clearAuthToken } from '../lib/api';

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

// Initialize user state from localStorage so that refresh keeps the login state.
export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(() => {
    if (typeof window !== 'undefined') {
      const storedUser = localStorage.getItem('auth_user');
      return storedUser ? JSON.parse(storedUser) : null;
    }
    return null;
  });
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  // Restore token from localStorage into our module variable.
  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      // Ensure that our in-memory token variable is set.
      // (getAuthToken() does this if it's null)
      getAuthToken();
    }
  }, []);

  const login = async (username: string, password: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.POST('/login', {
        body: { username, password },
      });
      if (result.data?.token) {
        setAuthToken(result.data.token);
        const minimalUser: User = { id: -1, username };
        setUser(minimalUser);
        localStorage.setItem('auth_user', JSON.stringify(minimalUser));
        router.push('/');
      } else {
        setError('Invalid login response');
      }
    } catch (err) {
      console.error('Login error:', err);
      setError('Login failed');
    } finally {
      setLoading(false);
    }
  };

  const signup = async (username: string, password: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.POST('/register', {
        body: { username, password },
      });
      if (result.data) {
        const loginResult = await api.POST('/login', {
          body: { username, password },
        });
        if (loginResult.data?.token) {
          setAuthToken(loginResult.data.token);
          const minimalUser: User = { id: result.data.id, username };
          setUser(minimalUser);
          localStorage.setItem('auth_user', JSON.stringify(minimalUser));
          router.push('/');
        } else {
          setError('Signup succeeded but login failed. Please try logging in.');
        }
      } else {
        setError('Signup failed: Invalid server response');
      }
    } catch (err) {
      console.error('Signup error:', err);
      setError('Signup failed');
    } finally {
      setLoading(false);
    }
  };

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