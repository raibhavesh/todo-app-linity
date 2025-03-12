use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{ToSchema,IntoParams};  // Add this import

#[derive(FromRow, Serialize, Deserialize, ToSchema)]  // Add ToSchema
pub struct User {
    #[schema(example = 1)]  // Add example values
    pub id: i32,
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "password123", write_only)]  // mark as write-only
    pub password: String,
}

#[derive(FromRow, Debug, Serialize, Deserialize, ToSchema)]  // Add ToSchema
pub struct Todo {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
    #[schema(example = 1)]
    pub user_id: i32,
}

#[derive(Deserialize, ToSchema)]  // Add ToSchema
pub struct NewTodo {
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = false)]
    pub completed: Option<bool>,
    #[serde(skip)]
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]  // Add ToSchema
pub struct UpdateTodo {
    #[schema(example = "Buy more groceries")]
    pub title: Option<String>,
    #[schema(example = true)]
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]  // Add ToSchema
pub struct RegisterPayload {
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]  // Add ToSchema
pub struct LoginPayload {
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "password123")]
    pub password: String,
}

// Add a response type for the token
#[derive(Deserialize, ToSchema, IntoParams)]  // Add IntoParams derive
pub struct TodoQueryParams {
    #[schema(example = true)]
    pub completed: Option<bool>,
    #[schema(example = "grocery")]
    pub search: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}