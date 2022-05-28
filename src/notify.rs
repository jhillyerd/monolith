use axum::{routing::post, Json, Router};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new().route("/mail", post(mail))
}

#[derive(Debug, Deserialize)]
struct Notification {
    subject: String,
    body: String,
}

async fn mail(Json(note): Json<Notification>) -> String {
    format!("{:?}", note)
}
