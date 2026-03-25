use axum::{body::Body, http::{Request, StatusCode}, Router, routing::get};
use std::time::Duration;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Level;

use crate::{model::ApiState, handlers};

pub fn api_router(state: ApiState) -> Router {
    Router::new()
        .route("/xn/{xn}", get(handlers::get_pacs_xn))
        .route("/thumbnail", get(handlers::get_pacs_thumbnail))
        .route("/image", get(handlers::get_pacs_image))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(ulid::Ulid::new()),
                )
            }),
        )
}
