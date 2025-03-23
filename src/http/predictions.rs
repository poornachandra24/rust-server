use std::process::Output;

use axum::{extract::State, Json};
use serde::{Serialize, Deserialize};
use http::StatusCode;
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Prediction {
    product_id: String,
    value: f64
}

pub async fn get_predictions(
    State(db): State<PgPool>
) -> Result<Json<Prediction>, StatusCode> {
    
    let prediction: Prediction = sqlx::query_as!(
        Prediction,
        "SELECT product_id, value
        FROM public.predictions
        WHERE product_id = $1",
        "BTC/EUR"
    )
    .fetch_one(&db) 
    .await.unwrap();
    Ok(Json(prediction))
}