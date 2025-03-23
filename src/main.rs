use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let router = Router::new()
        .route("/health", get(hello_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "Healthy and running!"
}