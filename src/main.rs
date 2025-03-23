use axum::{
    routing::get,
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use rust_rest_server::http::{health, predictions};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    println!("Hello, world! How are you?");
    dotenv().ok();

    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:123456@localhost/postgres")
        .await
        .unwrap();

    let router = Router::new()
        .route("/health", get(health::hello_handler))
        .route("/predictions", get(predictions::get_predictions))
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
