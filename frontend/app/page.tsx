'use client';

import React, { useState, useEffect } from 'react';
import api, { dynamicHeaders } from '../lib/api';
import TodoItem, { Todo } from '../components/TodoItem';
import TodoForm from '../components/TodoForm';
import ProtectedRoute from '../components/ProtectedRoute';
import { useAuth } from '../context/AuthContext';

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  // New state values for search and filter
  const [search, setSearch] = useState<string>('');
  // null means no filter, true means completed, false means incomplete.
  const [completedFilter, setCompletedFilter] = useState<boolean | null>(null);
  const { logout } = useAuth();

  const fetchTodos = async () => {
    setLoading(true);
    try {
      // Prepare query parameters by including only those with values.
      const queryParams: Record<string, any> = {};
      if (completedFilter !== null) {
        queryParams.completed = completedFilter;
      }
      if (search.trim() !== '') {
        queryParams.search = search.trim();
      }

      // Send the parameters as query parameters.
      const result = await api.GET('/todos', {
        headers: dynamicHeaders(),
        params: { query: queryParams },
      });
      if (result.data) {
        setTodos(result.data);
      }
    } catch (err) {
      setError('Failed to fetch todos');
    } finally {
      setLoading(false);
    }
  };

  // Re-fetch todos whenever the search or filter changes.
  useEffect(() => {
    fetchTodos();
  }, [completedFilter, search]);

  const addTodo = async (newTodo: Todo) => {
    try {
      const result = await api.POST('/todos', {
        headers: dynamicHeaders(),
        body: { title: newTodo.title, completed: false },
      });
      if (result.data) {
        setTodos((prev) => [...prev, result.data]);
      }
    } catch (err) {
      console.error('Add todo error:', err);
      setError('Failed to add todo');
    }
  };

  const updateTodo = async (updatedTodo: Todo) => {
    try {
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
      setError('Failed to update todo');
    }
  };

  const deleteTodo = async (id: number) => {
    try {
      await api.DELETE('/todos/{id}', {
        headers: dynamicHeaders(),
        params: { path: { id } },
      });
      setTodos((prev) => prev.filter((t) => t.id !== id));
    } catch (err) {
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
            className="px-4 py-2 bg-red-600 text-white rounded"
          >
            Logout
          </button>
        </header>

        {/* Search and Filter UI */}
        <div className="mb-4">
          <input
            type="text"
            placeholder="Search todos..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="p-2 border border-gray-300 rounded mr-2"
          />
          <select
            value={completedFilter === null ? '' : String(completedFilter)}
            onChange={(e) =>
              setCompletedFilter(
                e.target.value === ''
                  ? null
                  : e.target.value === 'true'
                  ? true
                  : false
              )
            }
            className="p-2 border border-gray-300 rounded"
          >
            <option value="">All</option>
            <option value="true">Completed</option>
            <option value="false">Incomplete</option>
          </select>
        </div>

        <TodoForm onAdd={addTodo} />

        {loading ? (
          <p>Loading todos...</p>
        ) : error ? (
          <p className="text-red-500">{error}</p>
        ) : todos.length === 0 ? (
          <p>No todos found. Add one!</p>
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
    </ProtectedRoute>
  );
}