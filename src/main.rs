use std::net::SocketAddr;

use axum::{routing::get, Router};

mod notify;
mod svc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/notify", notify::router())
        .route("/", get(|| async { "You've found your waypoint, Axum." }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
