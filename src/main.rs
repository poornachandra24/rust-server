use axum::{
    routing::get,
    Router,
    Json,
    response::{IntoResponse, Response},
};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};
use http::StatusCode;

#[tokio::main]
async fn main() {
    println!("Hello, world! How are you?");

    let router = Router::new()
        .route("/health", get(hello_handler))
        .route("/predictions", get(preditictions_handler));

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "Healthy and running!"
}

#[derive(Serialize, Deserialize)]
struct Prediction {
    product_id: String,
    value: f64
}

async fn preditictions_handler() -> Result<Json<Prediction>, StatusCode> {
    Ok(Json(Prediction {
        product_id: "123".to_string(),
        value: 123.45,
    }))
}