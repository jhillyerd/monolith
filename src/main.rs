use std::net::SocketAddr;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::settings::Settings;

mod notify;
mod settings;
mod shortlink;
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

    // Load settings.
    let settings = Settings::new().expect("can load settings");

    // Connect to database.
    let db = PgPoolOptions::new()
        .max_connections(3)
        .connect(&settings.database.url)
        .await
        .expect("can connect to postgresql");

    sqlx::migrate!().run(&db).await.expect("db migrations succeed");

    // Configure base router.
    let app = Router::new()
        .nest("/notify", notify::router(&settings))
        .nest("/shortlink", shortlink::router(&settings, db))
        .route("/", get(|| async { "You've found your waypoint, Axum." }));

    // Launch server.
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
