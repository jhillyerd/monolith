use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use sqlx::FromRow;

use crate::svc;

pub fn router() -> Router<svc::State> {
    Router::new()
        .route("/list", get(list))
        .route("/go/:key", get(go))
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

async fn go(
    Path(key): Path<String>,
    State(state): State<svc::State>,
) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query_as::<_, Shortlink>("select name, url from shortlinks where name = $1")
        .bind(&key)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(link)) => Ok(Redirect::to(&link.url)),
        Ok(None) => {
            tracing::error!("No results for link named {:?}", key);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(err) => {
            tracing::error!("Failed to database: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
