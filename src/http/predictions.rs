use axum::Json;
use serde::{Serialize, Deserialize};
use http::StatusCode;
#[derive(Serialize, Deserialize)]
pub struct Prediction {
    product_id: String,
    value: f64
}

pub async fn get_predictions() -> Result<Json<Prediction>, StatusCode> {
    Ok(Json(Prediction {
        product_id: "123".to_string(),
        value: 123.45,
    }))
}