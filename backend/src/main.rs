use axum::{
    routing::{get, post, put, delete},
    Router,
    Extension,
    middleware,
};
use std::net::SocketAddr;
use tokio;
use dotenvy::dotenv;
use anyhow::Result;
use db::connect_to_db;
//use sqlx::Pool;
//use crate::auth::require_auth;

mod db;
mod models;
mod handlers;
mod auth;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Load .env
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = connect_to_db(&database_url).await?;

    println!("Successfully connected to the database!");

    sqlx::query("SELECT setval('users_id_seq', (SELECT MAX(id) FROM users))")
        .execute(&pool)
        .await?;
    
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/register", post(handlers::register_handler))
        .route("/login", post(handlers::login_handler));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route(
            "/todos",
            get(handlers::get_all_todos_handler)
                .post(handlers::create_todo_handler),
        )
        .route(
            "/todos/:id",
            get(handlers::get_todo_handler)
                .put(handlers::update_todo_handler)
                .delete(handlers::delete_todo_handler),
        )
        // Apply the authentication middleware to all routes in this group
        .layer(middleware::from_fn(auth::require_auth));

    // Combine the routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}