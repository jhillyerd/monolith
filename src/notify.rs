use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use crate::svc;

pub fn router() -> Router<svc::State> {
    Router::new()
        .route("/mail", post(mail))
        .route("/text", post(text))
}

#[derive(Deserialize)]
struct Notification {
    subject: Option<String>,
    body: String,
    url: Option<String>,
}

async fn mail(
    State(state): State<svc::State>,
    Json(note): Json<Notification>,
) -> impl IntoResponse {
    if let Err(err) = svc::mail::send_alert(
        &state.mailer,
        &note.subject.unwrap_or(String::from("Notification")),
        &note.body,
    )
    .await
    {
        tracing::error!("Failed to send mail: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send");
    }

    (StatusCode::OK, "Sent")
}

async fn text(
    State(state): State<svc::State>,
    Json(note): Json<Notification>,
) -> impl IntoResponse {
    state
        .homeassistant
        .notify(
            note.subject.as_deref(),
            note.body.as_ref(),
            note.url.as_deref(),
        )
        .await
}
