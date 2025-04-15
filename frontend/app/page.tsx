'use client';

// Import useCallback along with other hooks
import React, { useState, useEffect, useCallback } from 'react';
import api, { dynamicHeaders } from '../lib/api';
import TodoItem, { Todo } from '../components/TodoItem';
import TodoForm from '../components/TodoForm';
import ProtectedRoute from '../components/ProtectedRoute';
import { useAuth } from '../context/AuthContext';

// Define a specific type for query parameters to avoid 'any'
interface TodoQueryParams {
  completed?: boolean | null;
  search?: string;
  // Add other potential query param types here if necessary
}

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  // State values for search and filter
  const [search, setSearch] = useState<string>('');
  // null means no filter, true means completed, false means incomplete.
  const [completedFilter, setCompletedFilter] = useState<boolean | null>(null);
  const { logout } = useAuth();

  // Wrap fetchTodos in useCallback to stabilize its reference
  const fetchTodos = useCallback(async () => {
    setLoading(true);
    setError(''); // Clear previous errors on new fetch
    try {
      // Prepare query parameters using the specific type
      const queryParams: TodoQueryParams = {};
      if (completedFilter !== null) {
        queryParams.completed = completedFilter;
      }
      if (search.trim() !== '') {
        queryParams.search = search.trim();
      }

      // Send the parameters as query parameters.
      // NOTE: The 'as any' below is a WORKAROUND for potentially incorrect API types
      // generated for `api.GET`. The IDEAL fix is to correct the OpenAPI spec
      // (ensure 'completed' and 'search' are 'in: query') and regenerate the types.
      // If the types ARE corrected, you should remove the 'as any' and the eslint disable comment.
      const result = await api.GET('/todos', {
        headers: dynamicHeaders(),
        params: queryParams,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } as any); // Disable lint rule for this line if 'as any' is necessary

      if (result.data) {
        setTodos(result.data);
      }
    } catch (err) {
      console.error('Fetch todos error:', err); // Log the actual error
      setError('Failed to fetch todos');
    } finally {
      setLoading(false);
    }
    // Dependencies for useCallback: include variables from the outer scope that the function depends on.
    // api, dynamicHeaders, setError, setLoading, setTodos are assumed stable (refs/setters).
  }, [search, completedFilter]); // fetchTodos depends on search and completedFilter values

  // useEffect now depends on the stable fetchTodos function.
  // It will run fetchTodos initially and whenever search or completedFilter changes (because that changes fetchTodos).
  useEffect(() => {
    fetchTodos();
  }, [fetchTodos]);

  const addTodo = async (newTodo: Omit<Todo, 'id' | 'completed'>) => { // More specific type for newTodo
    try {
      setError(''); // Clear previous errors
      const result = await api.POST('/todos', {
        headers: dynamicHeaders(),
        body: { title: newTodo.title, completed: false },
      });
      if (result.data) {
        setTodos((prev) => [...prev, result.data]);
      }
    } catch (err) {
      // Log the error (err was already used here correctly)
      console.error('Add todo error:', err);
      setError('Failed to add todo');
    }
  };

  const updateTodo = async (updatedTodo: Todo) => {
    try {
      setError(''); // Clear previous errors
      const result = await api.PUT('/todos/{id}', {
        headers: dynamicHeaders(),
        params: { path: { id: updatedTodo.id } },
        body: { title: updatedTodo.title, completed: updatedTodo.completed },
      });
      if (result.data) {
        setTodos((prev) =>
          prev.map((t) => (t.id === updatedTodo.id ? result.data : t))
        );
      }
    } catch (err) {
      // Log the error object to console
      console.error('Update todo error:', err);
      setError('Failed to update todo');
    }
  };

  const deleteTodo = async (id: number) => {
    try {
      setError(''); // Clear previous errors
      await api.DELETE('/todos/{id}', {
        headers: dynamicHeaders(),
        params: { path: { id } },
      });
      setTodos((prev) => prev.filter((t) => t.id !== id));
    } catch (err) {
      // Log the error object to console
      console.error('Delete todo error:', err);
      setError('Failed to delete todo');
    }
  };

  return (
    <ProtectedRoute>
      <div className="container mx-auto p-4">
        <header className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold">Todo App</h1>
          <button
            onClick={logout}
            className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
          >
            Logout
          </button>
        </header>

        {/* Error display */}
        {error && (
           <div className="mb-4 p-3 bg-red-100 text-red-700 rounded">
             {error}
           </div>
         )}

        {/* Search and Filter UI */}
        <div className="mb-4 flex items-center gap-2">
          <input
            type="text"
            placeholder="Search todos..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="p-2 border border-gray-300 rounded mr-2 flex-grow" // Added flex-grow
          />
          <select
            value={completedFilter === null ? '' : String(completedFilter)}
            onChange={(e) =>
              setCompletedFilter(
                e.target.value === ''
                  ? null
                  : e.target.value === 'true'
              )
            }
            className="p-2 border border-gray-300 rounded"
          >
            <option value="">All Statuses</option>
            <option value="true">Completed</option>
            <option value="false">Incomplete</option>
          </select>
        </div>

        <TodoForm onAdd={addTodo} />

        {/* Conditional Rendering for Todos */}
        <div className="mt-6">
          {loading ? (
            <p>Loading todos...</p>
          ) : todos.length === 0 ? ( // Check after loading is done
            <p>No todos found matching your criteria. Add one!</p>
          ) : (
            todos.map((todo) => (
              <TodoItem
                key={todo.id}
                todo={todo}
                onUpdate={updateTodo}
                onDelete={deleteTodo}
              />
            ))
          )}
        </div>
      </div>
    </ProtectedRoute>
  );
}