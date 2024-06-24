use axum::{ Router, routing::get };
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello Axum!" }))
        .nest("/todos", handlers::todo::routes())
}