use axum::{
    extract::{Extension, Path,Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::Pool;
use sqlx::Postgres;
use crate::models::{Todo, NewTodo, UpdateTodo, RegisterPayload, LoginPayload,TodoQueryParams};
use crate::models::User;
use bcrypt::verify;
use crate::auth;
use crate::auth::AuthenticatedUser;
use serde_json::json;

// Update get_all_todos_handler to support filtering
pub async fn get_all_todos_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(params): Query<TodoQueryParams>, // Add this parameter
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Choose the appropriate SQL based on the query parameters
    let todos = match (params.completed, &params.search) {
        // Case 1: Both completed and search are provided
        (Some(completed), Some(search)) => {
            sqlx::query_as::<_, Todo>(
                "SELECT t.id, t.title, t.completed, t.user_id 
                 FROM todos t
                 JOIN users u ON t.user_id = u.id
                 WHERE u.username = $1 AND t.completed = $2 AND t.title ILIKE '%' || $3 || '%'"
            )
            .bind(&auth_user.username)
            .bind(completed)
            .bind(search)
            .fetch_all(&pool)
            .await
        },
        
        // Case 2: Only completed filter is provided
        (Some(completed), None) => {
            sqlx::query_as::<_, Todo>(
                "SELECT t.id, t.title, t.completed, t.user_id 
                 FROM todos t
                 JOIN users u ON t.user_id = u.id
                 WHERE u.username = $1 AND t.completed = $2"
            )
            .bind(&auth_user.username)
            .bind(completed)
            .fetch_all(&pool)
            .await
        },
        
        // Case 3: Only search filter is provided
        (None, Some(search)) => {
            sqlx::query_as::<_, Todo>(
                "SELECT t.id, t.title, t.completed, t.user_id 
                 FROM todos t
                 JOIN users u ON t.user_id = u.id
                 WHERE u.username = $1 AND t.title ILIKE '%' || $2 || '%'"
            )
            .bind(&auth_user.username)
            .bind(search)
            .fetch_all(&pool)
            .await
        },
        
        // Case 4: No filters provided
        (None, None) => {
            sqlx::query_as::<_, Todo>(
                "SELECT t.id, t.title, t.completed, t.user_id 
                 FROM todos t
                 JOIN users u ON t.user_id = u.id
                 WHERE u.username = $1"
            )
            .bind(&auth_user.username)
            .fetch_all(&pool)
            .await
        },
    }
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Error: {}", err),
        )
    })?;

    Ok(Json(todos))
}

pub async fn create_todo_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(auth_user): Extension<AuthenticatedUser>,  // Move before Json
    Json(payload): Json<NewTodo>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // First get the user_id for the authenticated user
    let user_id = sqlx::query_scalar::<_, i32>("SELECT id FROM users WHERE username = $1")
    .bind(&auth_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("User lookup error: {}", err),
        )
    })?;

    // Then use user_id directly in your next query
    let inserted_todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed, user_id) 
        VALUES ($1, $2, $3) 
        RETURNING id, title, completed, user_id"
    )
    .bind(&payload.title)
    .bind(payload.completed.unwrap_or(false))
    .bind(user_id)  // Use the id directly instead of user.id
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Error: {}", err),
        )
    })?;
    
    Ok(Json(inserted_todo))
}

pub async fn update_todo_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(auth_user): Extension<AuthenticatedUser>,  // Move before Path
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Update the todo only if it belongs to the authenticated user
    let updated_todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos t
         SET title = COALESCE($1, t.title),
             completed = COALESCE($2, t.completed)
         FROM users u
         WHERE t.id = $3 
         AND t.user_id = u.id
         AND u.username = $4
         RETURNING t.id, t.title, t.completed, t.user_id"
    )
    .bind(payload.title)
    .bind(payload.completed)
    .bind(id)
    .bind(&auth_user.username)
    .fetch_optional(&pool)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("DB Error: {}", err),
    ))?;

    match updated_todo {
        Some(todo) => Ok(Json(todo)),
        None => Err((StatusCode::NOT_FOUND, format!("Todo with id {} not found or not owned by you", id)))
    }
}

// Update delete_todo_handler to only delete user's todo
pub async fn delete_todo_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(auth_user): Extension<AuthenticatedUser>,  // Move before Path
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Delete the todo only if it belongs to the authenticated user
    let result = sqlx::query(
        "DELETE FROM todos t
         USING users u
         WHERE t.id = $1 
         AND t.user_id = u.id
         AND u.username = $2"
    )
    .bind(id)
    .bind(&auth_user.username)
    .execute(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Error: {}", err),
        )
    })?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, format!("Todo with id {} not found or not owned by you", id)))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// Update get_todo_handler to only return user's todo
pub async fn get_todo_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(auth_user): Extension<AuthenticatedUser>,  // Move before Path
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Only get the todo if it belongs to the authenticated user
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT t.id, t.title, t.completed, t.user_id 
         FROM todos t
         JOIN users u ON t.user_id = u.id
         WHERE t.id = $1 AND u.username = $2"
    )
    .bind(id)
    .bind(&auth_user.username)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
    })?;

    if let Some(todo) = todo {
        Ok(Json(todo))
    } else {
        Err((StatusCode::NOT_FOUND, format!("Todo with id {} not found", id)))
    }
}

pub async fn register_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Hash the password using bcrypt.
    let hashed_password = hash(payload.password, DEFAULT_COST).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Password hashing error: {}", err),
        )
    })?;
    
    // Insert the user into the database, returning the new user.
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password) VALUES ($1, $2)
         RETURNING id, username, password"
    )
    .bind(payload.username)
    .bind(hashed_password)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Error: {}", err),
        )
    })?;
    
    Ok(axum::Json(user))
}

pub async fn login_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Retrieve the user by username.
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {}", err),
        )
    })?;
    
    if let Some(user) = user {
        // Verify the user's password against the stored hashed password.
        let is_valid = verify(&payload.password, &user.password).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Password verification error: {}", err),
            )
        })?;
        if is_valid {
            let token = auth::create_jwt(&user.username).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Token creation error: {}", err),
                )
            })?;
            // Return the token as JSON.
            Ok(axum::Json(json!({ "token": token })))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, "User not found".to_string()))
    }
}