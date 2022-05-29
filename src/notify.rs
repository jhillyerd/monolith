use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use serde::Deserialize;

use crate::svc;

type SmtpTx = AsyncSmtpTransport<Tokio1Executor>;

pub fn router() -> Router {
    let mailer: SmtpTx = SmtpTx::builder_dangerous("nexus.home.arpa").build();

    Router::new()
        .route("/mail", post(mail))
        .layer(Extension(mailer))
}

#[derive(Deserialize)]
struct Notification {
    subject: String,
    body: String,
}

async fn mail(
    Extension(mailer): Extension<SmtpTx>,
    Json(note): Json<Notification>,
) -> impl IntoResponse {
    if let Err(err) = svc::mail::send_alert(&mailer, &note.subject, &note.body).await {
        tracing::error!("Failed to send mail: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send");
    }

    (StatusCode::OK, "Sent")
}
