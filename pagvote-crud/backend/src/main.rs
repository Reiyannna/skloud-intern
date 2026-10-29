mod handlers;
mod models;
mod store;

use axum::Router;
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use handlers::{
    SharedState, create_contestant, delete_contestant, get_contestant, health, list_contestants,
    update_contestant,
};
use std::sync::Arc;
use store::AppState;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(AppState::new());

    // Local demo only: allow just the Nuxt dev server origin.
    // This is intentionally NOT permissive — a deployed API would
    // not open itself to every origin.
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
            "http://localhost:3000",
        )))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(AllowHeaders::list([axum::http::header::CONTENT_TYPE]));

    let app = Router::new()
        .route("/health", get(health))
        .route(
            "/api/contestants",
            get(list_contestants).post(create_contestant),
        )
        .route(
            "/api/contestants/{id}",
            get(get_contestant)
                .put(update_contestant)
                .delete(delete_contestant),
        )
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("failed to bind 127.0.0.1:3001");
    println!("Backend listening on http://127.0.0.1:3001");
    axum::serve(listener, app).await.expect("server failed");
}
