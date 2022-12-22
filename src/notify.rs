use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use serde::Deserialize;

use crate::{
    settings::Settings,
    svc::{self, homeassistant::HomeAssistant},
};

type SmtpTx = AsyncSmtpTransport<Tokio1Executor>;

#[derive(Clone)]
struct AppState {
    homeassistant: HomeAssistant,
    mailer: SmtpTx,
}

pub fn router(settings: &Settings) -> Router {
    let homeassistant = HomeAssistant::new(&settings.home_assistant);
    let mailer: SmtpTx = SmtpTx::builder_dangerous(&settings.mail.host).build();

    let state = AppState {
        homeassistant,
        mailer,
    };

    Router::new()
        .route("/mail", post(mail))
        .route("/text", post(text))
        .with_state(state)
}

#[derive(Deserialize)]
struct Notification {
    subject: Option<String>,
    body: String,
    url: Option<String>,
}

async fn mail(State(state): State<AppState>, Json(note): Json<Notification>) -> impl IntoResponse {
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

async fn text(State(state): State<AppState>, Json(note): Json<Notification>) -> impl IntoResponse {
    state
        .homeassistant
        .notify(
            note.subject.as_deref(),
            note.body.as_ref(),
            note.url.as_deref(),
        )
        .await
}
