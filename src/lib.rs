use std::{env, sync::LazyLock};

use axum::{
    Router,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::get,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub static NAME: LazyLock<String> =
    LazyLock::new(|| env::var("CONTAINER_NAME").unwrap_or("enano-web-test".to_string()));

async fn index() -> String {
    format!("Hello from {}", *NAME)
}

async fn ping() -> &'static str {
    "pong"
}

async fn status(Path(status): Path<u16>) -> StatusCode {
    StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_REQUEST)
}

async fn headers(headers: HeaderMap) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k.as_str(), v.to_str().unwrap()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn app() -> Router {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/status/{status}", get(status))
        .route("/headers", get(headers))
        .layer(TraceLayer::new_for_http())
}
