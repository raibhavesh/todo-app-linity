'use client';

import React, { useState } from 'react';

export type Todo = {
  id: number;
  title: string;
  completed: boolean;
  user_id: number;
};

type TodoItemProps = {
  todo: Todo;
  onUpdate: (todo: Todo) => void;
  onDelete: (id: number) => void;
};

export default function TodoItem({ todo, onUpdate, onDelete }: TodoItemProps) {
  const [isEditing, setEditing] = useState(false);
  const [title, setTitle] = useState(todo.title);

  const handleToggleComplete = () => {
    onUpdate({ ...todo, completed: !todo.completed });
  };

  const handleSave = () => {
    onUpdate({ ...todo, title });
    setEditing(false);
  };

  return (
    <div className="flex items-center justify-between p-4 bg-white rounded shadow mb-2">
      <div className="flex items-center space-x-3">
        <input
          type="checkbox"
          checked={todo.completed}
          onChange={handleToggleComplete}
          className="h-5 w-5"
        />
        {isEditing ? (
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="border border-gray-300 rounded p-1"
          />
        ) : (
          <span className={`text-lg ${todo.completed ? 'line-through text-gray-500' : ''}`}>
            {todo.title}
          </span>
        )}
      </div>
      <div className="space-x-2">
        {isEditing ? (
          <button onClick={handleSave} className="px-3 py-1 bg-green-500 text-white rounded">
            Save
          </button>
        ) : (
          <button onClick={() => setEditing(true)} className="px-3 py-1 bg-blue-500 text-white rounded">
            Edit
          </button>
        )}
        <button onClick={() => onDelete(todo.id)} className="px-3 py-1 bg-red-500 text-white rounded">
          Delete
        </button>
      </div>
    </div>
  );
}