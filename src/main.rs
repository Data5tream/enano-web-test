use tracing::info;

use enano_web_test::{NAME, app};

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!("Starting server with name: {}", *NAME);
    axum::serve(listener, app).await.unwrap();
}
