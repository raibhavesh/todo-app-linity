//// filepath: /home/shivtriv/todo-app-linity/backend/src/models.rs
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}