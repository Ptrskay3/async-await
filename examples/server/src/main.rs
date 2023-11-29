use axum::{extract::Path, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/:id", get(handler))
        .route("/fast/:id", get(handler_fast))
        .route("/blocking/:id", get(handler_blocking))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Path(name): Path<String>) -> String {
    tracing::warn!("incoming request");
    tokio::time::sleep(Duration::from_secs(2)).await;
    name
}

async fn handler_fast(Path(name): Path<String>) -> String {
    tracing::warn!("incoming request");
    name
}

// To illustrate blocking the executor with a long-running synchronous call.
async fn handler_blocking(Path(name): Path<String>) -> String {
    tracing::warn!("incoming request (blocking)");
    std::thread::sleep(Duration::from_secs(2));
    name
}
