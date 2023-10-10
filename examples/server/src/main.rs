use axum::{extract::Path, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/:id", get(handler))
        .route("/fast/:id", get(handler_fast))
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
