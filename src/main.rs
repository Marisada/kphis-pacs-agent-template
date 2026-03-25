// Using Jemalloc in MUSL has a 10x boost than MUSL default allocator
// and in some cases performs even better than the libc alloator.
// MiMalloc also has a same boost but MiMalloc eats a bit more memory than Jemalloc.
// https://github.com/clux/muslrust/issues/142#issuecomment-2184935013
#[cfg(all(target_arch = "x86_64", target_env = "musl"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_arch = "aarch64", target_env = "musl"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod error;
mod handlers;
mod model;
mod route;

use axum::Router;
use config::Config;
use std::net::SocketAddr;
use time::{UtcOffset, format_description::well_known::Rfc3339};
use tracing::info;
use tracing_appender::rolling::{Rotation, RollingFileAppender};
use tracing_subscriber::{
    EnvFilter, Layer, fmt::time::OffsetTime, layer::SubscriberExt,
};

#[tokio::main]
async fn main() {
    let config = get_config();

    // create log folder
    create_log_folder("rolling").expect("Cannot create ./volume/logs/rolling");

    // set Tracing Subscriber
    let log_file = config.get_string("log-file").expect("'log-file' not found in config file");
    let log_console = config.get_string("log-console").expect("'log-console' not found in config file");
    let keep_log_day = config.get_int("app-keep-log-day").expect("Not found 'app-keep-log-day' in config file") as usize;
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)
        .max_log_files(keep_log_day * 24)
        .build("./volume/logs/rolling")
        .expect("Error create LogAppender");
    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let timer = OffsetTime::new(UtcOffset::from_hms(7, 0, 0).unwrap_or(UtcOffset::UTC), Rfc3339);
    let subscriber = tracing_subscriber::registry()
        // log to file
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(file_non_blocking)
                .with_timer(timer.clone())
                .with_ansi(false)
                .with_target(false)
                .with_filter(EnvFilter::new(log_file)),
        )
        // log to console
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_timer(timer)
                .with_ansi(true)
                .with_target(true)
                .with_filter(EnvFilter::new(log_console)),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    tracing::info!("Start logging");

    let state = model::ApiState::new();
    let app_listener_port = config.get_int("app-listener-port").expect("Not found 'app-listener-port' in config file") as u16;
    let app = route::api_router(state);
    serve_http(app_listener_port, app).await;
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

/// create logs folder and parse config
fn get_config() -> Config {
    let config_path = "./volume/config/debug.toml";
    let cfg = config::Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("Error create config from file");
    println!("Loading config from {}", config_path);

    cfg
}

/// create ./volume/logs/{name} folder
fn create_log_folder(name: &str) -> Result<(), std::io::Error> {
    let dir = std::env::current_dir()?;
    let log_dir = dir.join("volume").join("logs").join(name);
    std::fs::create_dir_all(log_dir)
}