use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::FromRow;

use crate::svc;

pub fn router() -> Router<svc::State> {
    Router::new().route("/list", get(list))
}

#[derive(Debug, FromRow)]
struct Shortlink {
    name: String,
    url: String,
}

async fn list(State(state): State<svc::State>) -> impl IntoResponse {
    match sqlx::query_as::<_, Shortlink>("select name, url from shortlinks")
        .fetch_all(&state.db)
        .await
    {
        Ok(links) => {
            for link in links {
                tracing::info!("link: {0} -> {1}", link.name, link.url);
            }
            (StatusCode::OK, "Hi")
        }
        Err(err) => {
            tracing::error!("Failed to database: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to database")
        }
    }
}
