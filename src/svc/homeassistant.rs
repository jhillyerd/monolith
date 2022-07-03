use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::settings;

#[derive(Clone)]
pub struct HomeAssistant {
    url: String,
    auth: String,
}

const ANDROID_URL_DEFAULT: &str = "settings://notification_history";

impl HomeAssistant {
    pub fn new(config: &settings::HomeAssistant) -> HomeAssistant {
        HomeAssistant {
            url: config.url.clone(),
            auth: String::from("Bearer ") + &config.token,
        }
    }

    pub async fn notify(
        &self,
        title: Option<&str>,
        message: &str,
        url: Option<&str>,
    ) -> impl IntoResponse {
        let failed_resp = (StatusCode::INTERNAL_SERVER_ERROR, "Failed to notify");

        // Construct request body.
        let req = json!({
            "title": title,
            "message": message,
            "data": {
                "clickAction": url.unwrap_or(ANDROID_URL_DEFAULT),
                "url": url, // iOS URL.
            },
        });

        // Send JSON POST to Home Assistant.
        let client = reqwest::Client::new();
        match client
            .post(self.url.clone() + "/api/services/notify/notify")
            .json(&req)
            .header("Authorization", &self.auth)
            .send()
            .await
        {
            Err(err) => {
                tracing::error!("Failed to call API: {err}");
                return failed_resp;
            }
            Ok(resp) => match resp.status() {
                StatusCode::OK | StatusCode::CREATED => tracing::info!("Success: {resp:?}"),
                status => {
                    tracing::error!("API call failed with '{status}': {resp:?}");
                    return failed_resp;
                }
            },
        }

        (StatusCode::OK, "Sent")
    }
}
