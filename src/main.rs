use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use rust_rest_server::http::{health, predictions};


#[tokio::main]
async fn main() {
    println!("Hello, world! How are you?");

    let router = Router::new()
        .route("/health", get(health::hello_handler))
        .route("/predictions", get(predictions::get_predictions));

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
