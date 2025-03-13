use axum::{
    routing::{get, post},
    Router,
    Extension,
    middleware,
};
use std::net::SocketAddr;
use tokio;
use dotenvy::dotenv;
use anyhow::Result;
use db::connect_to_db;

// Import utoipa and SwaggerUi
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::{CorsLayer, Any};
use axum::http::HeaderValue;

mod db;
mod models;
mod handlers;
mod auth;

// Define the API documentation without security directives for now
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_all_todos_handler,
        handlers::create_todo_handler,
        handlers::get_todo_handler,
        handlers::update_todo_handler,
        handlers::delete_todo_handler,
        handlers::register_handler,
        handlers::login_handler
    ),
    components(
        schemas(
            models::Todo,
            models::NewTodo,
            models::UpdateTodo,
            models::User,
            models::RegisterPayload,
            models::LoginPayload,
            models::TodoQueryParams,
            models::TokenResponse
        )
    ),
    tags(
        (name = "todos", description = "Todo management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    )
)]
struct ApiDoc;

// Serve the OpenAPI schema using the OpenApi trait method
async fn serve_openapi() -> axum::Json<utoipa::openapi::OpenApi> {
    axum::Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = connect_to_db(&database_url).await?;

    println!("Successfully connected to the database!");

    // (Optional) Update sequences if needed
    sqlx::query("SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1))")
        .execute(&pool)
        .await?;
    sqlx::query("SELECT setval('todos_id_seq', COALESCE((SELECT MAX(id) FROM todos), 1))")
        .execute(&pool)
        .await?;

    // Public routes
    let public_routes = Router::new()
        .route("/register", post(handlers::register_handler))
        .route("/login", post(handlers::login_handler));

    // Protected routes requiring authentication
    let protected_routes = Router::new()
        .route(
            "/todos",
            get(handlers::get_all_todos_handler)
                .post(handlers::create_todo_handler)
        )
        .route(
            "/todos/:id",
            get(handlers::get_todo_handler)
                .put(handlers::update_todo_handler)
                .delete(handlers::delete_todo_handler)
        )
        .layer(middleware::from_fn(auth::require_auth));

    // Combine routes:
    // • Serve OpenAPI JSON at /api-docs/openapi.json
    // • Merge a Swagger UI router (it implements Into<Router>) which serves at /swagger-ui
    let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    // Keep only the OpenAPI JSON endpoint
    .route("/api-docs/openapi.json", get(serve_openapi))
    .layer(Extension(pool));


    // Add CORS layer here
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods(Any)
    .allow_headers(Any);

    // Update app with the CORS layer
    let app = app.layer(cors);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running at http://{}", addr);
    println!("API Documentation (OpenAPI JSON) available at http://{}/api-docs/openapi.json", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}