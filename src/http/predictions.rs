use axum::{extract::State, extract::Query, Json};
use serde::{Serialize, Deserialize};
use http::StatusCode;
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Prediction {
    product_id: String,
    value: f64
}
#[derive(Deserialize)]
pub struct PredictionParams {
    product_id: String,
}

pub async fn get_predictions(
    State(db): State<PgPool>,
    Query(params): Query<PredictionParams>,
) -> Result<Json<Prediction>, StatusCode> {
    
    let prediction: Prediction = sqlx::query_as!(
        Prediction,
        "SELECT product_id, value
        FROM public.predictions
        WHERE product_id = $1",
        params.product_id,
    )
    .fetch_one(&db) 
    .await.unwrap();

    Ok(Json(prediction))
}