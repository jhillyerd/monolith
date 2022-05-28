use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use serde::Deserialize;

use crate::svc;

pub fn router() -> Router {
    Router::new().route("/mail", post(mail))
}

#[derive(Deserialize)]
struct Notification {
    subject: String,
    body: String,
}

async fn mail(Json(note): Json<Notification>) -> impl IntoResponse {
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("nexus.home.arpa").build();

    if let Err(err) = svc::mail::send_alert(&mailer, &note.subject, &note.body).await {
        tracing::error!("Failed to send mail: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send");
    }

    (StatusCode::OK, "Sent")
}
