'use client';

import React, { useState } from 'react';
import { Todo } from './TodoItem';

type TodoFormProps = {
  onAdd: (newTodo: Todo) => void;
};

export default function TodoForm({ onAdd }: TodoFormProps) {
  const [title, setTitle] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (title.trim() === '') return;
    // Create a new todo object (id will be set by the backend)
    const newTodo = { id: 0, title, completed: false, user_id: 0 };
    onAdd(newTodo);
    setTitle('');
  };

  return (
    <form onSubmit={handleSubmit} className="flex space-x-2 mb-4">
      <input
        type="text"
        placeholder="Add a new todo"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        className="flex-1 p-2 border border-gray-300 rounded"
        required
      />
      <button type="submit" className="px-4 py-2 bg-indigo-600 text-white rounded">
        Add
      </button>
    </form>
  );
}