use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::settings::Settings;

mod notify;
mod settings;
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

    // Loading settings.
    let settings = Settings::new().expect("can load settings");

    // Configure base router.
    let app = Router::new()
        .nest("/notify", notify::router(&settings))
        .route("/", get(|| async { "You've found your waypoint, Axum." }));

    // Launch server.
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
