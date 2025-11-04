mod error;
mod handlers;
mod model;
mod route;

use axum::Router;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{fmt::time::ChronoLocal, EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct ApiState {
    
}

impl ApiState {
    fn new() -> Self {
        Self {
            
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_timer(ChronoLocal::rfc_3339())
                .with_ansi(true)
                .with_target(true)
                .with_filter(EnvFilter::new("debug,hyper=warn,axum=trace")),
        )
        .init();

    let state = ApiState::new();
    let app = route::api_router(state);
    serve_http(8888, app).await;
}

async fn serve_http(port: u16, app: Router) {
    let http_addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
    info!(
        "HTTP server started listening on {}, please Ctrl-c to terminate server.",
        listener.local_addr().expect("Panic local_addr()"),
    );
    axum::serve(listener, app).await.expect("Panic serve()");
}
