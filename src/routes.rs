use axum::{
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct WebhookData {
    // Define the data structure for your webhook here
    event: String,
    // ... other fields
}

pub async fn stream_live(Json(payload): Json<WebhookData>) -> StatusCode {
    // Handle the incoming webhook data
    // For example, log it, process it, etc.
    println!("Received webhook: {:?}", payload);
    StatusCode::OK
}

pub async fn metrics() -> String{
    // Return some metrics or a simple message
    "Metrics data here".to_string()
}