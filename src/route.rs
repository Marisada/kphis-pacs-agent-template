use axum::{Router, routing::get};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

use crate::{ApiState, handlers};

pub fn api_router(state: ApiState) -> Router {
    Router::new()
        .route("/xn/{xn}", get(handlers::get_pacs_xn))
        .route("/thumbnail", get(handlers::get_pacs_thumbnail))
        .route("/image", get(handlers::get_pacs_image))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}
