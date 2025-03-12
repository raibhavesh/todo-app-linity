//// filepath: /home/shivtriv/todo-app-linity/backend/src/main.rs
use axum::{
    routing::{get, post},
    Router,
    extract::Path, Extension,
};
use std::net::SocketAddr;
use tokio;
use dotenvy::dotenv;
use anyhow::Result;

mod db;
mod models;
mod handlers;
use db::connect_to_db;

// --- Handlers (Placeholder) ---
// Registration and Login
async fn register_handler() -> &'static str {
    "User registration (placeholder)"
}

async fn login_handler() -> &'static str {
    "User login (placeholder)"
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Load .env
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = connect_to_db(&database_url).await?;

    println!("Successfully connected to the database!");

    // Define the routes and add the DB pool as a shared extension.
    let app = Router::new()
        // User routes
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        // Todo routes
        .route(
            "/todos",
            get(handlers::get_all_todos_handler).post(handlers::create_todo_handler),
        )
        .route(
            "/todos/:id",
            get(handlers::get_todo_handler)
                .put(handlers::update_todo_handler)
                .delete(handlers::delete_todo_handler),
        )
        .layer(Extension(pool)); // Make the pool available to handlers

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}