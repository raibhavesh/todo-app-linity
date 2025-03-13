import createFetch from 'openapi-fetch';
import type { paths } from './types/api';

let authToken: string | null = null;

// A helper function returning a plain object (Record<string, string>)
function dynamicHeaders(): Record<string, string> {
  // Always return an object, not a union
  const headers: Record<string, string> = {};
  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`;
  }
  return headers;
}

const api = createFetch<paths>({
  baseUrl: 'http://localhost:3001',
  headers: dynamicHeaders() // Pass the function reference
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

// Initialize token from localStorage on load (client-side only)
if (typeof window !== 'undefined') {
  getAuthToken();
}

export default api;