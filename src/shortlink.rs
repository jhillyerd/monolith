use askama::Template;
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

#[derive(Template)]
#[template(path = "shortlink/list.html")]
struct ListTemplate {
    links: Vec<Shortlink>,
}

async fn list(State(state): State<svc::State>) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query_as::<_, Shortlink>("select name, url from shortlinks")
        .fetch_all(&state.db)
        .await
    {
        Ok(links) => Ok(ListTemplate { links }),
        Err(err) => {
            tracing::error!("Failed to database: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
