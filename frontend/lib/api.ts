import createFetch from 'openapi-fetch';
import type { paths } from './types/api';

let authToken: string | null = null;

export function dynamicHeaders(): Record<string, string> {
  const headers: Record<string, string> = {};
  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`;
  }
  return headers;
}

const api = createFetch<paths>({
  baseUrl: 'http://localhost:3001',
  // Initialize with no token.
  headers: {} 
});

export const setAuthToken = (token: string) => {
  authToken = token;
  localStorage.setItem('auth_token', token);
};

export const getAuthToken = (): string | null => {
  if (!authToken) {
    authToken = localStorage.getItem('auth_token');
  }
  return authToken;
};

export const clearAuthToken = () => {
  authToken = null;
  localStorage.removeItem('auth_token');
};

if (typeof window !== 'undefined') {
  getAuthToken();
}

export default api;