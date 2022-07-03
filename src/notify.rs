use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use serde::Deserialize;

use crate::{
    settings::Settings,
    svc::{self, homeassistant::HomeAssistant},
};

type SmtpTx = AsyncSmtpTransport<Tokio1Executor>;

pub fn router(settings: &Settings) -> Router {
    let homeassistant = HomeAssistant::new(&settings.home_assistant);
    let mailer: SmtpTx = SmtpTx::builder_dangerous(&settings.mail.host).build();

    Router::new()
        .route("/mail", post(mail))
        .route("/text", post(text))
        .layer(Extension(mailer))
        .layer(Extension(homeassistant))
}

#[derive(Deserialize)]
struct Notification {
    subject: Option<String>,
    body: String,
    url: Option<String>,
}

async fn mail(
    Extension(mailer): Extension<SmtpTx>,
    Json(note): Json<Notification>,
) -> impl IntoResponse {
    if let Err(err) = svc::mail::send_alert(
        &mailer,
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
    Extension(ha): Extension<HomeAssistant>,
    Json(note): Json<Notification>,
) -> impl IntoResponse {
    ha.notify(
        note.subject.as_deref(),
        note.body.as_ref(),
        note.url.as_deref(),
    )
    .await
}
