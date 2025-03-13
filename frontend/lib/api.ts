import createFetch from 'openapi-fetch';
import type { paths } from './types/api';

// Create API client with authentication handling
const api = createFetch<paths>({
  baseUrl: 'http://localhost:3001',
});

// Optional: Add authentication handling
// export const setAuthToken = (token: string) => {
//   api.defaults.headers = {
//     ...api.defaults.headers,
//     Authorization: `Bearer ${token}`,
//   };
// };

export default api;