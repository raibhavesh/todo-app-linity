'use client';

import { useState, useEffect } from 'react';
import api from '../lib/api';

// Define a simple Todo type until your generated types are working
type Todo = {
  id: number;        // Changed from string to number
  title: string;
  completed: boolean;
  user_id: number;   // Added this field
};

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    async function fetchTodos() {
      try {
        // Make sure your backend allows unauthenticated requests to /todos for testing
        const result = await api.GET('/todos', {
          params: {
            path: {
              completed: null,  // or true/false if filtering by completion status
              search: null      // or a search string if searching
            }
          }
        });
        
        if (result.data) {
          setTodos(result.data);
        } else {
          setError('No data received');
        }
      } catch (err) {
        setError('Error fetching todos');
        console.error(err);
      } finally {
        setLoading(false);
      }
    }

    fetchTodos();
  }, []);

  if (loading) return <div className="text-center p-4">Loading...</div>;
  if (error) return <div className="text-red-500 p-4">{error}</div>;

  return (
    <main className="max-w-4xl mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">My Todos</h1>
      {todos.length === 0 ? (
        <p>No todos found.</p>
      ) : (
        <ul className="space-y-2">
          {todos.map(todo => (
            <li key={todo.id} className="p-3 bg-white rounded shadow">
              {todo.title}
              <span className="ml-2 text-sm text-gray-500">
                {todo.completed ? '✅ Done' : '⏳ Pending'}
              </span>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}