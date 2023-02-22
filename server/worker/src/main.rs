pub mod shutdown;

use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::shutdown::shutdown_signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
