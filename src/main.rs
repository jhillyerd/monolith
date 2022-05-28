use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing_subscriber::{prelude::*, EnvFilter};

mod notify;
mod svc;

#[tokio::main]
async fn main() {
    // Setup async tracing.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    // Configure base router.
    let app = Router::new()
        .nest("/notify", notify::router())
        .route("/", get(|| async { "You've found your waypoint, Axum." }));

    // Launch server.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
